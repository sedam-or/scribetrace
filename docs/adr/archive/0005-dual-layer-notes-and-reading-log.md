# ADR 0005: Dual-Layer Note Storage Architecture and Reading Session Tracking

**Status:** Archived, superseded by [ADR 0008](../0008-notes-and-obsidian-export.md) (notes and Obsidian); reading-session log deferred  
**Date:** 2026-11-05  
**Deciders:** Principal Software Architect, Product Management  
**Consulted:** QA Lead  

---

## 1. Context and Problem Statement

Requirement **F7** specifies a collapsible side notes panel and reading progress tracker integrated directly into the document reader.

This feature encompasses two distinct user needs that create potential architectural friction:
1. **External Interoperability (F7.2):** Researchers demand that notes be stored as plain, unadorned Markdown files (`.scribetrace/notes/<hash>.md`) so they can open, search, and edit them directly using tools like Obsidian, VS Code, Logseq, or Typora without needing ScribeTrace open.
2. **Historical Versioning & Rollback (F7.11):** Note edits must follow the same append-only versioning principle as annotations, enabling readers to inspect earlier revisions, review what thoughts were written during specific reading phases, and restore previous drafts.
3. **Reading Session Tracking (F7.6–F7.10):** Automatically tracking reading progress, pages/sections visited, active duration, and historical timelines without recording false idle time when the user walks away from their desk.

If ScribeTrace merely writes a plain `.md` file, every edit overwrites previous history. Conversely, if notes are stored only inside an opaque event log, external tools cannot access or edit them.

---

## 2. Decision: Dual-Layer Note Storage Architecture

ScribeTrace implements a **Dual-Layer Projection Model**:

```
+---------------------------------------------------------------------------------+
|                       DUAL-LAYER NOTE STORAGE PIPELINE                          |
|                                                                                 |
|   [User Types in ScribeTrace Notes Tab]                                          |
|                │                                                                |
|                ▼                                                                |
|   1. Canonical Event Stream (Authoritative Source of Truth):                    |
|      Append to `.scribetrace/notes/<hash>.history.jsonl`                         |
|      { "event_id": "...", "hlc": "...", "action": "note_edit",                  |
|        "patch": "...diff patch...", "content": "# Synthesis..." }               |
|                │                                                                |
|                ▼ (Asynchronous Projection Worker)                               |
|   2. External Interoperability File (Plain Markdown Mirror):                    |
|      Write atomically to `.scribetrace/notes/<hash>.md`                          |
|      (Clean, standard GitHub-Flavored Markdown with zero proprietary headers)    |
|                                                                                 |
|   ───────────────────────────────────────────────────────────────────────────   |
|   [External Edit Flow: User Edits .md in Obsidian / VS Code]                    |
|                │                                                                |
|                ▼                                                                |
|   3. Filesystem Watcher detects `.md` mtime change                              |
|   4. Compute diff against last projected head                                   |
|   5. Append `action: "external_note_edit"` to `.history.jsonl`                  |
|   6. Update in-memory reader note buffer seamlessly                             |
+---------------------------------------------------------------------------------+
```

### 2.1 Deep Linking & Quote Capture Syntax
To allow frictionless navigation between notes and document positions (F7.4, F7.5):
* **Deep Link URI:**
  `scribetrace://doc/<content-hash>?pos=<cfi_or_page_or_offset>&annot=<annot-id>`
* **One-Click Quote Capture:**
  Selecting text or an annotation and clicking "Quote to Note" generates a clean Markdown quote block:
  ```markdown
  > "Attention mechanisms have become an integral part of sequence modeling..."
  > — *Vaswani et al. (p. 2)* [Jump to Text](scribetrace://doc/sha256-a1b2...?pos=page:2&annot=019a4f20...)
  ```
  Clicking the URI inside ScribeTrace navigates the reader viewport and triggers a gentle visual pulse on the referenced target markup.

---

## 3. Reading Session Tracking & Inactivity Detection

### 3.1 Session Quantification (F7.6)
* **Paginated Documents (PDF):**
  $$\text{Progress} = \left(\frac{\text{Highest Valid Page Viewed}}{\text{Total Document Pages}}\right) \times 100$$
* **Reflowable Documents (EPUB, HTML, Markdown):**
  $$\text{Progress} = \left(\frac{\text{Highest Scrolled Character Offset}}{\text{Total Document Character Count}}\right) \times 100$$

### 3.2 Inactivity Cutoff Algorithm (F7.8)
To prevent overnight idle sessions from distorting reading statistics:
1. An active heartbeat timer monitors DOM user interactions (mouse movement, key presses, wheel scroll, touch gestures).
2. If zero interaction occurs for $> 5.0\text{ minutes}$, the timer enters `idle` state.
3. Upon document close, navigation away, or window minimize, a reading session entry is appended to `.scribetrace/reading-log/<content-hash>.jsonl`:
   ```json
   {
     "session_id": "019a4f2b-8c10-7000-84c1-92b0c3456789",
     "start_time": "2026-11-05T09:15:00.000Z",
     "end_time": "2026-11-05T09:47:00.000Z",
     "active_seconds": 1540,
     "range_read": {
       "format": "pdf",
       "start_page": 1,
       "end_page": 8,
       "max_page_reached": 8
     },
     "summary": "Completed initial pass through methodology and attention architecture."
   }
   ```
4. Entries are aggregated across documents into `.scribetrace/workspace-reading-log.jsonl` for high-level project dashboards.

---

## 4. Consequences

### Positive:
* Maximum user freedom: notes can be edited in Obsidian, VS Code, or ScribeTrace interchangeably.
* 100% versioned rollback capability without risking file corruption.
* Objective, realistic reading session durations unaffected by idle abandonment.
* Pure local storage with zero cloud dependencies or telemetry leakage.

### Negative & Mitigations:
* *External edit race conditions:* If the user edits the note in both Obsidian and ScribeTrace simultaneously.
  * *Mitigation:* The file watcher detects external mtime changes; if the in-memory buffer has uncommitted changes, ScribeTrace presents a non-destructive 2-way merge dialog showing both versions.

---

## 5. Security Impact

* **Markdown XSS Neutralization:** The Markdown renderer uses an AST sanitization pipeline powered by `DOMPurify`. Raw `<script>`, `<iframe>`, `javascript:`, and `data:text/html` payloads are stripped before rendering.
* **Deep Link URI Protocol Security:** The custom `scribetrace://` URI scheme is strictly validated: target document hashes must match the 64-character SHA-256 regex, and parameter offsets are strictly cast to safe numerical or CFI strings. Navigation to arbitrary external file paths or web URLs via deep links is blocked.
* **Reading Progress Privacy:** Reading logs contain only local document hashes and relative reading durations. No system usernames, IP addresses, machine identifiers, or external telemetry are included.

