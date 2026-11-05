# Product Management Review: REQUIREMENTS.md (v1 Draft)

**Reviewer:** Senior Project Manager  
**Date:** 2026-11-05  
**Document Under Review:** `docs/REQUIREMENTS.md` (v1 draft)  
**Product Target:** Palimpsest (formerly *clickme*)  
**Status:** Complete — Ready for Consolidation with Architect and QA Reviews  

---

## 1. Executive Summary

The draft requirements specification for **Palimpsest** provides a strong, user-centric foundation rooted in clear governing principles (the Constitution). The core value proposition—treating reading history as an immutable, append-only research artifact while never mutating original source files—addresses real, verified dysfunctions in existing academic workflows.

However, from an Agile delivery and product management perspective, the v1 draft suffers from **scope compression**: it bundles features spanning a Minimum Viable Product (MVP), full General Availability (v1.0), and post-v1.0 enhancements into an unphased list where everything appears mandatory simultaneously. Furthermore, several requirement statements conflate multiple UI and architectural responsibilities into single clauses, creating estimation and testability hazards.

This review delivers a structured critique across INVEST criteria, EARS syntax validity, MoSCoW sequencing, brand alignment, and specific recommendations to finalize `REQUIREMENTS.md` into an executable v1.0 specification.

---

## 2. Branding & Terminology Alignment

### 2.1 Product Renaming
* **Observation:** The draft repeatedly cites the working title `clickme` and the directory `.clickme/`.
* **Directive:** Per executive user decision, the product is officially named **Palimpsest**.
* **Action Required:**
  1. Replace all occurrences of `clickme` with **Palimpsest** throughout `REQUIREMENTS.md`.
  2. Change the workspace directory specification from `.clickme/` to `.palimpsest/`.
  3. Update file path examples (`.palimpsest/workspace.json`, `.palimpsest/annotations/<content-hash>.jsonl`, `.palimpsest/notes/<content-hash>.md`).

---

## 3. INVEST Principles & EARS Criteria Analysis

```
+----------------------------------------------------------------------------------------------------+
|                                    INVEST EVALUATION SUMMARY                                       |
+--------------------------+--------+----------------------------------------------------------------+
| INVEST Dimension         | Rating | PM Assessment & Critique                                       |
+--------------------------+--------+----------------------------------------------------------------+
| Independent              | Amber  | F7 (Notes & Log) is tightly coupled with F3 (Anchoring) and   |
|                          |        | F4 (Event Log). Needs clean interface decoupling.              |
| Negotiable               | Green  | Good functional balance; leaves implementation to Architect.   |
| Valuable                 | Green  | Directly addresses researcher pain points and JTBDs.           |
| Estimable                | Amber  | F3.1 (ink/shapes) and F2.1 (multi-engine formats) have high    |
|                          |        | variance without explicit phase boundaries.                    |
| Small (Sized for Flow)   | Amber  | F1.6 (Workspace search) and F3.1 (all mark types) are too large|
|                          |        | for single sprint increments. Must be decomposed.             |
| Testable (EARS Syntax)   | Green  | Strong adoption of EARS (WHEN/THEN/WHILE/IF); minor gaps.      |
+--------------------------+--------+----------------------------------------------------------------+
```

### 3.1 Feature-by-Feature Critique

#### F1: Workspace (Project = Local Folder)
* **Strengths:** F1.1–F1.5 represent an exemplary local-first workspace model. Hash-based re-linking (F1.5) provides resilience against accidental file moves outside the app.
* **Critique & Gaps:**
  * **F1.6 (Workspace Search):** Currently reads: *"The system SHALL offer workspace-wide search over document text, annotation text, notes and tags."* This is an epic-sized requirement disguised as a single story. Searching through raw PDF text layers across 1,000 documents requires either an asynchronous full-text indexer (e.g. SQLite FTS5 / Tantivy) or linear scan. 
  * **Recommendation:** Split F1.6 into two tiers:
    - *MVP:* Fast in-memory search over annotation text, note text, and tags.
    - *v1.0:* Background indexed full-text search across document text bodies.

#### F2: Reading
* **Strengths:** Strict performance budget on F2.3 (< 2 s for 50-page PDF on 4-core / 8 GB hardware) guarantees desktop responsiveness.
* **Critique & Gaps:**
  * **F2.1 (Format Parity):** Bundling PDF, EPUB, HTML (arXiv HTML), and Markdown into a single acceptance clause obscures substantial architectural divergence. PDF requires a canvas-based pagination engine (e.g. PDF.js or native bindings), whereas EPUB and HTML are DOM-based reflowable documents, and Markdown is plain text.
  * **Recommendation:** Decouple delivery: PDF and Markdown are Must-Have for MVP; EPUB and arXiv HTML are Should-Have for v1.0.

#### F3: Annotation
* **Strengths:** F3.2 (Overlay corrections without source rewrite) and F3.4/F3.5 (W3C text quote + position anchoring with orphan recovery) are top-tier specifications that directly protect original document integrity.
* **Critique & Gaps:**
  * **F3.1 (Annotation Breadth):** Lists 9 distinct annotation modalities in one sentence: *"highlight (≥ 6 colours), underline, strikethrough, freehand ink, shapes/arrows, text box, sticky note, comment on a selection, and tags on any annotation."*
  * Freehand ink and vector arrows require coordinate transformations across zoom, window resize, and reflowable text layouts. Delivering all 9 types simultaneously in sprint 1 risks stalling the release.
  * **Recommendation:**
    - *MVP Must-Have:* Text highlight (≥ 6 colors) + text comment on selection + tags.
    - *v1.0 Should-Have:* Underline, strikethrough, text box, sticky note.
    - *v1.1+ Could-Have:* Freehand ink, arrows, and vector shapes.

#### F4: History (Core Value Proposition)
* **Strengths:** F4.1–F4.5 represent the core differentiator of Palimpsest. Treating restore actions as new append-only events (F4.3) preserves full causality and prevents history truncation.
* **Critique & Gaps:**
  * **F4.6 (Named Snapshots):** *"The user SHALL be able to create a named snapshot ('first read', 'after review') and later view the annotation layer as it was at that snapshot (read-only)."*
  * While valuable for reviewers, visual snapshot time-travel requires snapshot tag indexing in the event log and reader read-only state projection.
  * **Recommendation:** Classify F4.6 as a Should-Have for v1.0 so that F4.1–F4.5 (core log engine) can be verified first in the MVP.

#### F5: Storage (Sidecar Files)
* **Strengths:** Plain UTF-8 JSON Lines (F5.1) and zero-loss merge by event ID union (F5.3) are sound, elegant architectural choices for sync compatibility (Git, Syncthing, Dropbox).
* **Critique & Gaps:**
  * Ensure path specification uses `.palimpsest/annotations/<content-hash>.jsonl`.
  * Clarify indexing: F5.1 references *"an index mapping file paths to hashes"*. Define where this index lives (`.palimpsest/index.json` or within `workspace.json`).

#### F6: Export / Interoperability
* **Strengths:** F6.1 (W3C Web Annotation JSON-LD + Markdown) and F6.2 (Flattened PDF copy) cover both modern knowledge management (Obsidian) and traditional peer review workflows.
* **Critique & Gaps:**
  * F6.3 (Importing existing PDF annotations into the log) introduces PDF parsing complexity (extracting standard Adobe PDF `/Annots` dictionaries).
  * **Recommendation:** Keep Markdown export in MVP; schedule W3C JSON-LD, flattened PDF generation, and PDF annotation import for v1.0.

#### F7: Side Notes Panel & Reading Log
* **Strengths:** F7.1–F7.5 (Collapsible panel, per-document markdown file, deep linking, 1-click quote insertion) provide immense utility without leaving the reading flow.
* **Critique & Gaps:**
  * F7.8/F7.9 (Automated session timing and timeline visualization) are sophisticated features that require tracking window focus, idle detection, and reading velocity thresholds to avoid logging hours when a user leaves their laptop open.
  * **Recommendation:**
    - *MVP:* Collapsible Notes panel, per-document markdown note (`.palimpsest/notes/<content-hash>.md`), bidirectional deep links, reading status dropdown (Unread, Reading, Read).
    - *v1.0:* Automated reading session duration tracking, idle filtering, and chronological reading timeline.

---

## 4. MoSCoW Release Prioritization Alignment

To ensure predictable velocity, `REQUIREMENTS.md` must clearly map requirements to release milestones:

```
+----------------------------------------------------------------------------------------------------+
|                                    REQUIREMENTS MoSCoW MATRIX                                      |
+--------------------------+--------------------------+-----------------------+----------------------+
|  MUST HAVE (MVP v0.1)    |  SHOULD HAVE (v1.0)      |  COULD HAVE (v1.1+)   |  WON'T HAVE (v1)     |
+--------------------------+--------------------------+-----------------------+----------------------+
| - F1.1-F1.5 Folder ws,   | - F1.6 Workspace search  | - F3.1 Freehand ink / | - Cloud sync service |
|   .palimpsest/ setup     | - F2.1 EPUB + arXiv HTML |   arrows / shapes     | - User accounts      |
| - F2.1-F2.3 PDF + MD     | - F3.1 Underline, strike | - F4.6 Named snapshot | - BibTeX citation DB |
|   fast rendering         |   sticky note, boxes     |   diff & scrub viewer | - AI summarization   |
| - F3.1 Text highlight    | - F6.1 W3C JSON-LD exp   | - F5.3 GUI 3-way merge| - In-place PDF text  |
|   (6 colours) + comments | - F6.2 Flattened PDF exp | - Mobile/tablet touch |   binary editing     |
| - F4.1-F4.5 Append-only  | - F6.3 PDF annot import  | - External PKM sync   | - DOCX / LaTeX native|
|   event log + undo/redo  | - F7.6-F7.10 Reading log |   plugins             | - Real-time network  |
| - F5.1-F5.4 JSONL sidecar|   timelines & durations  |                       |   collaboration      |
| - F7.1-F7.5 Notes panel  | - F7.11 Note versioning  |                       |                      |
|   + deep linking         |                          |                       |                      |
+--------------------------+--------------------------+-----------------------+----------------------+
```

---

## 5. Non-Functional Requirements (NFR) Assessment

| NFR Category | Draft Clause | PM Evaluation & Recommendation |
| :--- | :--- | :--- |
| **Platforms** | Linux (AppImage/deb/Flatpak), Win 10+, macOS 12+ | **Approved.** Standard cross-platform desktop matrix. Ensure CI builds test all three early. |
| **Performance** | New annotation < 100 ms; 1,000 docs < 3 s; 10k events < 1 s. | **Approved.** Realistic and measurable. 10k events in < 1 s mandates in-memory state projection. |
| **Reliability** | Atomic writes (write-then-rename / fsync). Crash loses < 1 s input. | **Approved.** Essential to prevent corrupted JSON Lines. |
| **Privacy & Security** | No network calls; sandboxed HTML (scripts disabled); no telemetry. | **Approved.** Core selling point against SaaS readers. |
| **Accessibility** | Full keyboard navigation; WCAG 2.1 AA target. | **Approved.** Ensure all annotation actions have keyboard shortcuts. |
| **i18n** | Externalized strings; English first, Indonesian second. | **Approved.** Strong signal of international community readiness. |

---

## 6. Resolution of Open Questions (PM Stance)

1. **Question 1 (License):**
   * *PM Recommendation:* **Apache License 2.0**. Provides explicit patent defense (§3) and trademark protection (§6), which are vital for open-source document rendering systems.
2. **Question 2 (Author Identity):**
   * *PM Recommendation:* **Single local display name per machine** (defaults to OS user, editable in preferences). Avoids cloud accounts while providing attribution in the event log.
3. **Question 3 (Sidecar Location):**
   * *PM Recommendation:* **Fixed `.palimpsest/` workspace root directory**. Prevents file clutter across research folders and keeps original document trees immaculate.
4. **Question 4 (Product Name):**
   * *PM Recommendation:* **Palimpsest**. Fully clear of blocking software trademarks; perfectly evokes our layered, append-only markup philosophy.
5. **Question 5 (Reading Progress Tracking Default):**
   * *PM Recommendation:* **On by default (local only)** with prominent Settings toggle. Zero network egress ensures no privacy violation.

---

## 7. Action Items for Finalizing REQUIREMENTS.md v1.0

When the Architect and QA reviews are submitted:
1. Update document header: `Status: v1.0, reviewed by PM, Architect, QA`.
2. Transition title and text from `clickme` / `.clickme/` to `Palimpsest` / `.palimpsest/`.
3. Incorporate MoSCoW release prioritization table directly into Section 4.
4. Refine F1.6, F2.1, F3.1, and F7 into clean, independently estimable user stories.
5. Append a comprehensive **Decision Log** tracking all review points from PM, Architect, and QA with explicit status (`accepted` / `rejected` + rationale).
6. Append a **Decisions for the User** section documenting all adopted assumptions.
