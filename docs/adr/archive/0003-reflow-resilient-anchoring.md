# ADR 0003: Reflow-Resilient Composite Anchoring Engine

**Status:** Archived, superseded by [ADR 0011](../0011-reader-pdf-markdown-and-anchoring.md) for the MVP (reflowable-format cascade deferred, kept as history)  
**Date:** 2026-11-05  
**Deciders:** Principal Software Architect, Product Management  
**Consulted:** QA Lead, Systems Engineering  

---

## 1. Context and Problem Statement

A core tenet of ScribeTrace is non-destructive annotation: source documents are strictly read-only, and markup is stored externally in sidecar files.

In fixed-layout documents (PDF), annotations can be anchored geometrically using page numbers and normalized coordinate bounding boxes (`QuadPoints: [x1, y1, x2, y2]`). 

However, in reflowable formats (EPUB, responsive HTML, and Markdown):
1. Page numbers and fixed pixel coordinates do not exist.
2. Modifying font sizes, line heights, margins, or window dimensions causes text to reflow dynamically across lines, columns, and virtual pages.
3. Pure character offsets (W3C `TextPositionSelector`) are brittle: any variation in DOM parsing, white-space collapsing, or minor document edits shifts the offset, misplacing the highlight.
4. Pure exact text search (W3C `TextQuoteSelector`) fails when whitespace characters, non-breaking spaces, or hyphenated line breaks differ between rendering passes.

ScribeTrace requires an anchoring model that remains 100% stable across reflows, zoom levels, and minor document variations, while guaranteeing that no annotation is ever silently corrupted or discarded.

---

## 2. Decision: Composite Selector Bundle with Tiered Fallback

Every annotation event records a unified, multi-faceted target selector schema conforming to and extending the **W3C Web Annotation Data Model**:

```json
{
  "source_hash": "sha256-e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "format": "epub",
  "container": "EPUB/text/chapter02.xhtml",
  "selectors": {
    "cfi": "/6/4[chap02]!/4/2/10/1:24",
    "text_quote": {
      "exact": "attention mechanism allows modeling dependencies",
      "prefix": "In this section, we show that the ",
      "suffix": " without regard to their distance in"
    },
    "text_position": {
      "start": 14208,
      "end": 14256
    },
    "dom_range": {
      "start_container": "body > section#methods > p:nth-child(4)",
      "start_offset": 34,
      "end_container": "body > section#methods > p:nth-child(4)",
      "end_offset": 82
    },
    "pdf_quads": null
  }
}
```

For PDF documents, `pdf_quads` contains `page_index` and normalized page-relative coordinates `[x1, y1, x2, y2]` in 72-DPI user space, while still capturing `text_quote` to allow robust text search and cross-format export.

---

## 3. Four-Tier Resolution Hierarchy

When a document is opened or reflowed, ScribeTrace resolves anchors against the active DOM through a prioritized cascade:

```
+---------------------------------------------------------------------------------+
|                       TIERED ANCHOR RESOLUTION PIPELINE                         |
|                                                                                 |
|  [Target Selector]                                                              |
|         │                                                                       |
|         ▼                                                                       |
|  Tier 1: Fast Position Verification                                             |
|  Inspect container at `text_position.start` to `end`.                           |
|  Does substring match `text_quote.exact`? ──YES──> [Resolved (Confidence 1.0)]  |
|         │ NO                                                                    |
|         ▼                                                                       |
|  Tier 2: Contextual Quote Search                                                |
|  Search container DOM node for `prefix + exact + suffix`.                       |
|  Exact substring match found? ─────────────YES──> [Resolved (Confidence 0.98)] |
|         │ NO                                                                    |
|         ▼                                                                       |
|  Tier 3: Whitespace & Punctuation Normalization                                 |
|  Collapse `\s+` to single spaces, strip soft hyphens, match quote.              |
|  Normalized match found? ──────────────────YES──> [Resolved (Confidence 0.92)] |
|         │ NO                                                                    |
|         ▼                                                                       |
|  Tier 4: Myers / Bitap Fuzzy Approximation                                      |
|  Run approximate string matching across target section DOM.                     |
|  Levenshtein similarity score ≥ 85%? ──────YES──> [Resolved (Confidence ≥ 0.85)]|
|         │ NO (< 85% confidence)                                                 |
|         ▼                                                                       |
|  [Quarantine as Orphaned (F3.5)]                                                |
|  Do NOT delete. Surface in Orphan Drawer with 1-Click Manual Re-Anchor Tool.    |
+---------------------------------------------------------------------------------+
```

### Myers / Bitap Approximate Matching Details
* The algorithm searches within the target chapter or $\pm 2,000$ characters of the original `text_position`.
* Allows for minor typography changes (e.g. smart quotes vs. straight quotes, Unicode ligatures `fi` vs. `f`+`i`, hyphenation changes).
* If the highest match score is $\ge 0.85$, the anchor resolves with a visual indicator noting a fuzzy match.
* If $< 0.85$, the annotation is marked `status: "orphaned"`.

---

## 4. Consequences

### Positive:
* Annotations survive window resizing, font zooming, theme toggling, and EPUB virtual column pagination without drift.
* In fixed PDFs, dual anchoring preserves text content for search and Markdown export even if PDF text layer extraction differs across versions.
* Total non-destructive guarantee (Constitution Rule 1 & F3.5): orphaned annotations are never deleted and can be manually re-attached by the user.

### Negative & Mitigations:
* *Computational overhead of fuzzy matching:* Running Myers diff over a 500-page book would cause UI lag.
  * *Mitigation:* Scoped search window. Tiers 1–3 run in < 1 ms per annotation. Tier 4 fuzzy search is restricted to the specific container element (`CFI` chapter or PDF page), bounding execution time under 15 ms.

---

## 5. Security Impact

* **ReDoS Prevention in Fuzzy Matcher:** Maliciously crafted target strings in sidecar files (e.g. nested repeating patterns) could cause Regular Expression Denial of Service (ReDoS) or combinatorial explosion in string-matching algorithms. The Myers / Bitap search engine operates on linear-time bit-vector algorithms with a hard timeout of 50 ms per search query.
* **Text Quote Sanitization:** All extracted `exact`, `prefix`, and `suffix` strings in W3C selectors are treated as inert plaintext and encoded before insertion into the DOM or sidecar logs. No raw HTML tags or script blocks in quotes are ever parsed as executable markup.
* **Orphan Quarantine Integrity:** Corrupted or modified documents cannot trick the anchor engine into attaching annotations to unauthorized sections or spoofing annotations across different files.

