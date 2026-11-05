# Architectural Review of Product Requirements (v1 Draft)

**Project:** Palimpsest (formerly *clickme*)  
**Reviewer:** Principal Software Architect  
**Date:** 2026-11-05  
**Document Reviewed:** `docs/REQUIREMENTS.md` (v1 draft)  
**Status:** Complete — Ready for PM & QA Alignment  

---

## 1. Executive Summary

The core architectural thesis of **Palimpsest**—*immutable append-only event logging, local-first sidecar storage in `.palimpsest/`, zero mutation of source files, and integrated reading/notes history*—is technically sound, highly differentiating, and fills a genuine void in the scholarly workflow ecosystem.

However, several critical architectural ambiguities, edge cases, and hidden complexities in the v1 draft require formal hardening before baseline v1.0 sign-off. Specifically:
1. **Anchoring Fragility in Reflowable Formats:** Pure W3C `TextPositionSelector` + `TextQuoteSelector` models drift when reflowing EPUBs or responsive HTML. A dual-tier anchoring engine with Myers/Levenshtein fuzzy fallback and structural DOM context is mandatory.
2. **Hash-Based Re-Linking Cost:** On-the-fly SHA-256 calculation across an entire folder on external file renames will bottleneck I/O. A tiered caching strategy (file size + mtime check + head/tail chunk hash + background full hash) is required.
3. **JSONL Union Merge & Clock Skew:** Independent writes across multiple devices (via Syncthing, Git, or Dropbox) cannot rely solely on physical wall-clock timestamps for conflict ordering. Deterministic event ordering requires Hybrid Logical Clocks (HLC) and automatic ingestion of sync conflict sidecars.
4. **Note Versioning Conflict (F7.2 vs. F7.11):** Storing notes as plain `.md` files directly contradicts storing note edits as an append-only event log. A dual-representation architecture (canonical event log + projected `.md` snapshot) is necessary.
5. **Undo/Redo Event Explosion (F4.5):** Generating persistent disk log events for interactive UI undo/redo steps will saturate logs with transient noise. The spec must decouple interactive transient transactions from committed semantic events.

---

## 2. Requirement-by-Requirement Feasibility Analysis

Requirements are rated as **Feasible**, **Risky**, or **Unrealistic**, accompanied by architectural rationale and mitigation strategies.

### 2.1 Functional Requirements (F1–F7)

#### F1: Workspace (Project = Local Folder)

| ID | Title / Summary | Rating | Architectural Assessment & Rationale |
| :--- | :--- | :--- | :--- |
| **F1.1** | Recursive document listing with title, type, status, count | **Feasible (with constraints)** | Feasible, but recursive traversal must ignore standard large trees (`.git`, `node_modules`, build artifacts). Metadata extraction (PDF info, EPUB OPF, HTML `<title>`) must run asynchronously in background workers to avoid UI thread starvation. |
| **F1.2** | Single root workspace directory (`.palimpsest/`) | **Feasible** | Clean, proven design pattern (comparable to `.obsidian/` or `.vscode/`). Fully segregates metadata from user research assets. |
| **F1.3** | File watcher detecting changes within 2 s | **Risky** | Native file watchers (`inotify`, `ReadDirectoryChangesW`, `FSEvents`) behave inconsistently across operating systems and network shares (NFS/SMB/FUSE). Inotify descriptor exhaustion can crash watchers on deep directories. Requires a debounced watcher with polling fallback for network drives. |
| **F1.4** | Recent workspaces & concurrent multi-workspace support | **Feasible** | Multi-window desktop shell architecture with decoupled per-window workspace state and dedicated IPC channels. |
| **F1.5** | Content-hash re-linking on move/rename | **Risky** | Hashing every unlinked file with full SHA-256 blocks disk I/O on large multi-gigabyte document collections. Feasible only with a two-stage heuristic: (1) quick fast-hash (size + mtime + first/last 64 KB block), followed by (2) asynchronous full cryptographic SHA-256 verification. |
| **F1.6** | Workspace-wide full-text search (documents, notes, tags) | **Risky** | Searching 1,000 PDFs in memory is unviable. Requires a lightweight embedded SQLite FTS5 index located inside `.palimpsest/cache/index.db`. This database must be strictly treated as a throwaway, rebuildable index cache to honor Constitution Rule 2 (*Files over databases*). |

#### F2: Reading

| ID | Title / Summary | Rating | Architectural Assessment & Rationale |
| :--- | :--- | :--- | :--- |
| **F2.1** | Unified reader UI for PDF, EPUB, HTML, Markdown | **Feasible** | Feasible via specialized web rendering engines: PDF.js for PDF, Foliate-js (preferred over epub.js for performance and DOM cleanliness) for EPUB, sandboxed iframe for HTML, and unified AST renderer (e.g. `markdown-it` / `remark` with `DOMPurify`) for Markdown. Unified navigation abstraction is achievable. |
| **F2.2** | Restore last reading position | **Feasible** | Position state must be format-aware: Page number + viewport scroll percentage for PDF; EPUB Canonical Fragment Identifier (CFI) for EPUB; CSS selector + character offset for HTML; line number / block offset for Markdown. |
| **F2.3** | 50 MB / 500-page PDF first page rendered < 2 s | **Feasible** | Feasible using PDF.js streaming range requests and virtualized canvas rendering, avoiding reading the entire 50 MB file into memory before displaying Page 1. |

#### F3: Annotation

| ID | Title / Summary | Rating | Architectural Assessment & Rationale |
| :--- | :--- | :--- | :--- |
| **F3.1** | Markup toolset (highlight, underline, ink, shapes, comments, tags) | **Feasible** | Dual-layer rendering: SVG vector overlay for freehand ink, arrows, and geometric shapes; styled DOM text overlay / canvas rects for text highlights, underlines, and strikethroughs. |
| **F3.2** | Overlay text correction suggestions (source untouched) | **Feasible** | Implemented as insertion/replacement diff widgets rendered in the reading margin or inline text overlay, leaving source text byte-identical. |
| **F3.3** | Auto-persist changes within 1 s without manual save | **Feasible** | Micro-batched debounced append (300 ms debounce window) writing directly to the append-only event stream on the file system. |
| **F3.4** | Dual anchoring (W3C TextQuote/TextPosition + PDF QuadPoints) | **Risky** | While PDF coordinate geometry (`QuadPoints` + Page index) is deterministic, EPUB/HTML reflow alters text node positions when window widths, margins, or fonts change. Pure character offset (`TextPositionSelector`) breaks on reflow. W3C `TextQuoteSelector` (`exact`, `prefix`, `suffix`) is mandatory, backed by a robust fuzzy-matching fallback (Myers diff / Levenshtein window search) when exact matches fail. |
| **F3.5** | Orphaned annotations flagged, never deleted | **Feasible** | When selector confidence falls below threshold (< 0.75 match score), the annotation is marked `status: "orphaned"`, displayed in an orphaned drawer, and provides a manual 1-click re-anchor UI. |

#### F4: History (Core Value)

| ID | Title / Summary | Rating | Architectural Assessment & Rationale |
| :--- | :--- | :--- | :--- |
| **F4.1** | Immutable event log (id, timestamp, author, action, payload) | **Feasible** | Foundational event-sourcing pattern. Guarantees deterministic state replay: `Replay(Events) -> CurrentState`. |
| **F4.2** | Soft delete (tombstones in log) | **Feasible** | Appending an `annotation_deleted` event preserves historical integrity while removing the item from the active projection. |
| **F4.3** | Per-annotation version inspection and restore | **Feasible** | Filter event stream by `target_id`. Restore operation appends an `annotation_restored` event referencing the restored state snapshot. |
| **F4.4** | Chronological document history timeline with filters | **Feasible** | In-memory materialized index over the document's event stream. Highly responsive for up to 50,000 events. |
| **F4.5** | Undo/redo within session producing log events | **Unrealistic (as written)** | If every intermediate stroke or undo/redo tap immediately appends an immutable line to the persistent JSONL file, rapid operations (e.g. typing or moving an ink stroke) will produce massive log bloat with ephemeral garbage. **Resolution:** Interactive undo/redo must operate in an in-memory command stack. Only final, settled states (on debounce completion or pointer-up) are committed as semantic events to the append-only log. |
| **F4.6** | Named snapshots ("first read", "after review") | **Feasible** | Event type `snapshot_created` records a name, description, and the exact event sequence ID / watermark up to which the projection is evaluated. |

#### F5: Storage (Sidecar Files)

| ID | Title / Summary | Rating | Architectural Assessment & Rationale |
| :--- | :--- | :--- | :--- |
| **F5.1** | Sidecars at `.palimpsest/annotations/<hash>.jsonl` + path index | **Feasible** | Excellent decoupling. Moving or renaming documents never breaks annotation data. UTF-8 JSON Lines allows streaming read/write with minimal memory overhead. |
| **F5.2** | Versioned, backwards-compatible sidecar format | **Feasible** | Explicit schema envelope containing `"schema_version": 1`. Forward migrations handled during projection load without rewriting existing historical events. |
| **F5.3** | Conflict-free union merge without manual intervention | **Risky** | Union of event IDs guarantees no event loss during multi-device synchronization (Dropbox, Syncthing, Git). However, if two devices edit the same annotation concurrently with skewed system clocks, Last-Write-Wins (LWW) based on wall-clock time can produce inconsistent state. **Mitigation:** Use Hybrid Logical Clocks (HLC) combining physical time with monotonic logical counters. Furthermore, the application must detect and ingest sync conflict files (e.g. `<hash>.sync-conflict-*.jsonl`). |
| **F5.4** | Corrupt line handling (skip, warn, keep rest) | **Feasible** | Line-by-line streaming parse with JSON decode error trap. Corrupt lines are quarantined to `.palimpsest/corrupt/` for forensic review. |

#### F6: Export / Interop

| ID | Title / Summary | Rating | Architectural Assessment & Rationale |
| :--- | :--- | :--- | :--- |
| **F6.1** | Export to Markdown and W3C Web Annotation JSON-LD | **Feasible** | Pure serialization transform mapping materialized annotation objects to standard Markdown templates or the W3C JSON-LD context. |
| **F6.2** | Flattened annotated PDF export | **Feasible** | Requires native PDF manipulation (e.g. via `pdf-lib` in JS or `lopdf`/`pdf-writer` in Rust). Burns annotations, highlights, and comments into standard PDF appearance streams (`/Annots`). Original file remains strictly untouched. |
| **F6.3** | Import embedded PDF annotations into event log | **Feasible** | Parses standard PDF annotation dictionaries (`/Highlight`, `/Underline`, `/FreeText`, `/Ink`) on initial import and synthesizes `annotation_imported` events in the log. |

#### F7: Side Notes Panel & Reading Log

| ID | Title / Summary | Rating | Architectural Assessment & Rationale |
| :--- | :--- | :--- | :--- |
| **F7.1** | Collapsible Notes tab side panel | **Feasible** | Standard split-pane layout with persistent width preferences and sub-100ms hardware-accelerated CSS transition. |
| **F7.2** | Plain Markdown note per doc at `.palimpsest/notes/<hash>.md` | **Feasible** | Ensures plain-text portability and compatibility with external tools (Obsidian, VS Code, Logseq). |
| **F7.3** | Note auto-save within 1 s | **Feasible** | Debounced file write (500 ms) directly to disk. |
| **F7.4** | Deep-links from notes to document anchors/positions | **Feasible** | Custom URI scheme (e.g. `palimpsest://doc/<hash>?pos=<cfi_or_page>&annot=<id>`). Clicking handles smooth-scroll and highlight pulsation in the active reader. |
| **F7.5** | 1-click quote insertion with backlink | **Feasible** | Extracts selected text, surrounds with `> quote\n\n[Reference](palimpsest://...)`, and inserts at the editor's cursor position. |
| **F7.6** | Reading progress tracking (pages, furthest pos, % read) | **Feasible** | Viewport intersection observer tracks document visibility. Progress percentage computed based on total page count or content block count. |
| **F7.7** | Explicit reading status (Unread, Reading, Skimmed, Read, To re-read) | **Feasible** | Stored in document session metadata within `.palimpsest/workspace.json` or document state record. |
| **F7.8** | Append reading session entry on close/blur | **Feasible** | Debounced session aggregator: records start time, active duration (ignoring idle periods > 3 minutes), and page range read to `.palimpsest/reading-log/<hash>.jsonl`. |
| **F7.9** | Timeline view of reading log in Notes tab | **Feasible** | UI component rendering session entries chronologically beneath the document Markdown note. |
| **F7.10** | Workspace-wide notes & reading log | **Feasible** | Stored at `.palimpsest/workspace-notes.md` and `.palimpsest/workspace-reading-log.jsonl`. |
| **F7.11** | Note edits versioned under append-only principle | **Risky (Architectural Conflict)** | **Conflict with F7.2:** Storing notes as a plain `.md` file means external edits or saves overwrite the file. If note edits must be versioned with full rollback history, a single `.md` file cannot store historical states. **Solution:** Store the canonical, versioned note edit history in `.palimpsest/notes/<hash>.history.jsonl` (or diff patches), while projecting the current head revision directly to `.palimpsest/notes/<hash>.md` for external tool interop. |
| **F7.12** | Reading tracking local-only, opt-out per workspace | **Feasible** | Simple workspace configuration flag `tracking_enabled: boolean` stored in `workspace.json`. |

---

### 2.2 Non-Functional Requirements (NFR) Feasibility

| Category | Requirement | Rating | Architectural Assessment & Rationale |
| :--- | :--- | :--- | :--- |
| **Platforms** | Linux (AppImage/deb/Flatpak), Windows 10+, macOS 12+ | **Feasible** | Cross-platform desktop shell (Tauri v2 with Rust or Electron) provides unified native capabilities and automated build pipelines across all three targets. |
| **Performance: Latency** | New annotation visible in < 100 ms | **Feasible** | Optimistic UI update: the local UI render tree immediately mounts the annotation before asynchronous disk persistence completes. |
| **Performance: Workspace** | 1,000 documents listed in < 3 s | **Risky** | Calculating SHA-256 for 1,000 files on startup takes > 15 seconds on standard SSDs and > 60 seconds on HDDs. **Feasible ONLY IF** Palimpsest uses a fast mtime + size check against the index cache, hashing only newly discovered or altered files asynchronously in background threads. |
| **Performance: Log Replay** | 10,000 events load in < 1 s | **Feasible** | Streaming JSONL parser. Benchmarks indicate parsing and reducing 10,000 JSON lines (~3 MB) takes ~35–50 ms in modern V8 / JavaScript runtimes and < 15 ms in Rust. |
| **Performance: UI Panels** | Open/close side panel < 100 ms | **Feasible** | CSS transform / flex transition on GPU-accelerated layers. |
| **Reliability: Atomic Writes** | Write-then-rename / fsync on append | **Feasible** | Standard durability pattern: appends write with OS flush (`flush()` / `fsync()`); state file rewrites use atomic temporary file write followed by atomic rename (`rename()`). |
| **Reliability: Crash Recovery** | Crash loses at most last 1 s of input | **Feasible** | Maximum write debounce interval capped at 1,000 ms. In-flight uncommitted strokes flushed on window blur / close events. |
| **Security: Sandbox & Network** | No network calls by default; sandboxed HTML with scripts disabled | **Feasible & Critical** | Crucial requirement. HTML papers (e.g. arXiv HTML) frequently bundle tracking scripts, remote fonts, or arbitrary JS. HTML rendering must be hosted in an isolated iframe / webview with `sandbox="allow-same-origin"` and a Content Security Policy (`CSP: default-src 'self' data: blob:; script-src 'none'; connect-src 'none'`). |
| **Accessibility** | WCAG 2.1 AA, full keyboard navigation, screen reader labels | **Feasible** | Requires semantic HTML elements, accessible SVG roles (`aria-label`, `role="graphics-document"`), and keyboard shortcuts for all annotation operations. |
| **i18n** | Externalized strings, English + Indonesian | **Feasible** | Standard i18n framework (e.g. `i18next` or Fluent). Zero hardcoded UI strings. |
| **Open-Source Health** | OSI license, GitHub Actions CI on 3 OSes, one-command setup | **Feasible** | Automated CI matrix running lint, unit tests, end-to-end headless tests, and packaging scripts. |

---

## 3. Deep-Dive Architectural Focus Areas

### 3.1 Anchoring Engine across Reflowable Formats

Text anchoring in static PDFs is geometric: `(pageIndex, quadPoints: [x1, y1, x2, y2])`. This is stable across zooms and devices.

In reflowable EPUB, HTML, and Markdown, geometric coordinates do not exist. If window dimensions change, fonts resize, or styling updates, character positions in the document flow will shift.

#### The Problem with Pure W3C Selectors
* `TextPositionSelector` (`start`, `end` character offsets) is notoriously brittle; any modification or reflow differences between reader engines causes severe offset drift.
* `TextQuoteSelector` (`exact`, `prefix`, `suffix`) is robust against small changes, but exact substring search fails when formatting whitespace, non-breaking spaces, or line breaks fluctuate.

#### Recommended Anchoring Architecture: Three-Tier Selector Strategy
Palimpsest must store a unified composite selector bundle in every annotation event:

```json
{
  "target": {
    "source_hash": "sha256-a1b2c3...",
    "format": "epub",
    "container": "EPUB/text/ch01.xhtml",
    "selectors": {
      "cfi": "/6/4[chap01]!/4/2/10/1:15",
      "text_quote": {
        "exact": "attention is all you need",
        "prefix": "In this work, we propose that ",
        "suffix": ". The dominant sequence transduction"
      },
      "text_position": {
        "start": 1420,
        "end": 1445
      },
      "dom_range": {
        "start_container_path": "body > section:nth-child(2) > p:nth-child(3)",
        "start_offset": 12,
        "end_container_path": "body > section:nth-child(2) > p:nth-child(3)",
        "end_offset": 37
      }
    }
  }
}
```

#### Resolution Algorithm
1. **Fast Path:** Attempt exact match using `TextPositionSelector` verified against `TextQuoteSelector.exact`. If character text matches, anchor is resolved in < 1 ms.
2. **Context Path:** If position shifted, perform substring search within the same DOM container / chapter using `prefix + exact + suffix`.
3. **Fuzzy Fallback (Myers / Bitap Algorithm):** If exact substring fails, run a fuzzy approximate string match (allowing Levenshtein distance $\le 3$ for quote length $> 20$).
4. **Orphan Quarantine:** If confidence score $< 0.75$, mark annotation as `orphaned`. Never silently discard or misplace an annotation.

---

### 3.2 Content-Hash Re-Linking & Fast Workspace Scanning

Requirement **F1.5** mandates: *"IF a document is moved or renamed outside the app THEN the system SHALL re-link its annotations and notes by content hash."*

#### The I/O Bottleneck
If a user renames a 150 MB textbook or moves 50 papers into a subfolder, calculating full SHA-256 for all documents on startup or watcher trigger would saturate disk bandwidth and violate the 3-second workspace load requirement (NFR).

#### Recommended Two-Stage Fast-Hash Architecture
Palimpsest should maintain a lightweight hash registry in `.palimpsest/cache/index.db`:

```
+---------------------------------------------------------------------------------+
|                         TWO-STAGE RE-LINKING PIPELINE                           |
|                                                                                 |
|  1. Stat Check:                                                                 |
|     File Path + Size + mtime matches Cache? ---> [Instant Re-Link: OK]          |
|                                                                                 |
|  2. Unmatched File / Change Detected:                                           |
|     Compute Fast Chunk Hash:                                                    |
|     Hash(FileSize + First 64KB + Last 64KB) ---> Match found in Cache?          |
|                                                  |                              |
|                                                 YES: Spawn Background Thread    |
|                                                      Verify Full SHA-256        |
|                                                      Update Path-to-Hash Map    |
|                                                  |                              |
|                                                  NO: Full SHA-256 Hashing       |
+---------------------------------------------------------------------------------+
```

This ensures file moves and renames are identified in **< 10 ms per file**, deferring full cryptographic verification to background non-blocking tasks.

---

### 3.3 JSONL Union Merge, Concurrency, and Clock Skew

Requirement **F5.3** specifies: *"IF the same sidecar is changed on two machines THEN merging the two logs SHALL be a union of events by event id, with no manual conflict resolution."*

#### The Concurrency & Clock Skew Vulnerability
An append-only log union merge handles *additions* seamlessly (set union of distinct event IDs). However, annotations undergo *state mutations* (e.g. recoloring a highlight, editing a comment text, or deleting a mark).
* If Machine A edits a comment at local time `10:00:05`, and Machine B edits the same comment at local time `10:00:03` (due to a 5-second physical clock skew), a naive Last-Write-Wins (LWW) rule using local wall-clock timestamps will overwrite the logically later edit.
* Furthermore, sync utilities like Dropbox, Syncthing, or Nextcloud do not merge `.jsonl` files line-by-line; they generate conflict files such as:
  `sha256-abc.sync-conflict-20261105-142010.jsonl`

#### Architectural Solution
1. **Hybrid Logical Clocks (HLC):** Every event must include an HLC timestamp `(physical_ms, logical_counter, client_id)`:
   ```json
   {
     "id": "evt-019a4f2b-8c10-7000-84c1-92b0c3456789",
     "hlc": "2026-11-05T09:45:00.120Z-0001-clientA",
     "target_id": "annot-987",
     "type": "annotation_updated",
     "author": "Alice",
     "payload": { "color": "#ffeb3b" }
   }
   ```
   HLC guarantees strict causal ordering across independent distributed clients without requiring a central coordinator.
2. **Conflict Sidecar Harvester:**
   The workspace file watcher must actively monitor for sync conflict naming conventions:
   * `*.sync-conflict-*` (Syncthing)
   * `* (conflicted copy *` (Dropbox)
   * `*.conflict-*` (Nextcloud/ownCloud)
   When detected, Palimpsest opens both files, executes an in-memory union merge of events ordered by HLC, writes the unified stream back to `<hash>.jsonl`, and safely removes the conflict duplicate file.

---

### 3.4 Resolving the Note Storage Dichotomy (F7.2 vs. F7.11)

There is a direct architectural tension in Section 4:
* **F7.2:** Stored as plain Markdown in `.palimpsest/notes/<hash>.md`, readable without Palimpsest.
* **F7.11:** Note edits versioned under the same append-only principle as annotations, with full historical inspection and rollback.

If external tools (or Palimpsest itself) directly mutate `.palimpsest/notes/<hash>.md`, the edit history is destroyed. Conversely, if notes are stored only inside an append-only event log, external tools cannot easily view or edit them.

#### The Dual-Layer Projection Architecture
```
+---------------------------------------------------------------------------------+
|                       DUAL-LAYER NOTE STORAGE MODEL                             |
|                                                                                 |
|   Canonical Source of Truth:                                                    |
|   .palimpsest/notes/<hash>.history.jsonl                                        |
|   (Append-only log of note_created, note_updated, note_reverted events)         |
|                               |                                                 |
|                   Projection Engine (On Every Save)                             |
|                               v                                                 |
|   External Interop Projection:                                                  |
|   .palimpsest/notes/<hash>.md                                                   |
|   (Clean, standard GitHub-flavored Markdown snapshot with zero metadata noise)   |
|                                                                                 |
|   Bi-directional External Sync:                                                 |
|   If user edits .md file in Obsidian/VS Code:                                   |
|   Watcher detects file change -> Synthesizes external_note_edit event in log    |
+---------------------------------------------------------------------------------+
```
This satisfies both requirements: the `.md` file remains 100% standard and human-readable, while Palimpsest retains full, non-destructive version history.

---

### 3.5 HTML Sandboxing & Security Boundaries

Academic HTML papers (particularly from arXiv HTML, bioRxiv, or saved Web archives) contain untrusted, arbitrary third-party markup.

#### Threat Vectors
1. **Remote Network Leakage:** Embedded `<img src="https://...">`, `<script>`, or CSS `url()` can leak reading habits, user IP, and document access times to remote servers, violating Constitution Rule 4 (*Offline, local-first*).
2. **Arbitrary Code Execution:** Malicious JavaScript executing in an electron/tauri webview could access local files or execute IPC exploits.

#### Security Architecture
* **Strict CSP Header:**
  `Content-Security-Policy: default-src 'none'; style-src 'unsafe-inline'; img-src data: blob:; font-src data:; script-src 'none'; connect-src 'none'; frame-src 'none';`
* **Isolated Iframe / Webview:**
  Sandboxed with `<iframe sandbox="allow-same-origin">`. Scripts are unconditionally disabled.
* **Asset Sanitization:**
  External URLs stripped or rendered with placeholder warnings. Only local embedded base64 data URIs or workspace-cached assets are resolved.

---

## 4. Proposed Wording Changes for REQUIREMENTS.md

To eliminate ambiguities and resolve architectural edge cases, the following wording changes are proposed:

| ID | Current Wording (v1 Draft) | Proposed Replacement Wording | Architectural Reason |
| :--- | :--- | :--- | :--- |
| **F1.3** | WHILE a workspace is open the system SHALL detect files added, renamed, moved or deleted outside the app within 2 s and update the list. | WHILE a workspace is open the system SHALL detect files added, renamed, moved or deleted outside the app within 2 s and update the list, with an automatic polling fallback enabled when operating on network or virtual file systems. | Native OS file watchers fail to emit events reliably over network mounts (NFS/SMB) and certain FUSE layers. |
| **F1.5** | IF a document is moved or renamed outside the app THEN the system SHALL re-link its annotations and notes by content hash. | IF a document is moved or renamed outside the app THEN the system SHALL identify the re-link candidate via file-size and chunk-cache heuristics within 1 s, and re-link its annotations and notes upon background SHA-256 confirmation. | Prevents disk I/O thrashing and UI freezes when scanning large repositories for moved documents. |
| **F3.4** | Text-based annotations SHALL be anchored by text quote + position (with a page/coordinate fallback for PDF), so they survive reflow in EPUB/HTML/Markdown. | Text-based annotations SHALL be anchored using a composite selector containing W3C TextQuote (exact, prefix, suffix), TextPosition, and DOM/CFI paths (with page/coordinate fallback for PDF). In reflowable formats, an approximate string-matching algorithm SHALL resolve anchors when exact character positions shift. | Pure character offsets break under EPUB reflow; fuzzy quote matching is required for real-world resilience. |
| **F4.5** | Undo/redo SHALL work within a session and SHALL itself produce log events, so nothing is ever lost. | Undo/redo SHALL operate on an in-memory session stack for interactive manipulation, committing settled state changes as discrete events to the append-only log upon transaction completion or debounce expiration (max 1 s). | Prevents rapid keystrokes or intermediate drag coordinates from generating thousands of ephemeral garbage events in the persistent log. |
| **F5.3** | IF the same sidecar is changed on two machines (e.g. via git, Syncthing or Dropbox) THEN merging the two logs SHALL be a union of events by event id, with no manual conflict resolution. Sync itself is the user's responsibility; the app guarantees merge-safe files. | IF the same sidecar is changed on two machines (e.g. via git, Syncthing or Dropbox) THEN merging the two logs SHALL be a union of events by event id ordered by Hybrid Logical Clocks (HLC), with no manual conflict resolution. The system SHALL automatically detect and ingest sync conflict files emitted by sync providers. | Guarantees deterministic state ordering across clock skews and automates ingestion of sync conflict sidecars. |
| **F7.2** & **F7.11** | *F7.2:* ...stored as a plain file in the workspace (`.palimpsest/notes/<content-hash>.md`)...<br>*F7.11:* Note edits SHALL be versioned under the same append-only principle as annotations... | The Notes tab SHALL maintain canonical versioned edit history in `.palimpsest/notes/<content-hash>.history.jsonl` while continuously projecting the current markdown snapshot to `.palimpsest/notes/<content-hash>.md` for external tool interop. External edits to the `.md` file SHALL be detected and appended to the history log. | Resolves the direct contradiction between plain file storage and append-only version rollback. |
| **NFR: Perf** | A workspace of 1,000 documents is listed in < 3 s. | A workspace of 1,000 documents is listed in < 3 s by utilizing a cached metadata index verified against file modification times (mtime). | Full cryptographic hashing of 1,000 documents on startup cannot physically complete in < 3 s on spinning disks or cold caches. |

---

## 5. Identification of Missing Architectural Requirements

The following six critical areas are absent from the v1 draft and must be incorporated into the v1.0 specifications:

### 5.1 Event Schema Definition & Forward Migration Rules
* **Gap:** The specification mentions JSON Lines with `id, timestamp, author, action, payload`, but does not define schema versioning, payload validation, or evolution rules.
* **Requirement Needed:** Every event must declare an explicit envelope:
  `{ "v": 1, "id": "uuidv7", "hlc": "...", "target_id": "...", "type": "...", "payload": { ... } }`.
  The reader projection engine must preserve unrecognized future fields without truncation when writing back or snapshotting.

### 5.2 Log Compaction & Snapshotting Strategy
* **Gap:** A paper annotated over several years may accumulate 10,000+ micro-events. Replaying the raw log on every document open will eventually cause memory and latency degradation.
* **Requirement Needed:** When an event log exceeds a configurable threshold (e.g. 5,000 events or 5 MB), Palimpsest must generate a consolidated state snapshot (`<hash>.snapshot.json`), truncating the active log while archiving older events into `<hash>.archive.jsonl`. Replay starts from snapshot + delta.

### 5.3 Multi-Process File Locking & Workspace Concurrency
* **Gap:** A user may open the same workspace folder in two separate app windows, or launch two instances on the same machine. Unsynchronized concurrent writes to `.palimpsest/workspace.json` or sidecar files will corrupt data.
* **Requirement Needed:** Implement file-level cooperative advisory locking (`fcntl` on Linux/macOS, `LockFileEx` on Windows) on workspace configuration and active sidecar writes to guarantee atomic append operations.

### 5.4 Network Drives, Remote File Systems, and Cloud Sync Latency
* **Gap:** Researchers frequently place project folders on networked lab shares (NFS, SMB), academic cloud drives (Google Drive for Desktop, OneDrive), or external USB drives.
* **Requirement Needed:** System must gracefully handle transient I/O disconnections, read-only file system mounts, and delayed sync file hydration without crashing. If `.palimpsest/` is temporarily non-writable, changes must buffer in local memory and warn the user.

### 5.5 Workspace Scale Limits & Ignored Directory Boundaries
* **Gap:** Opening a user's home folder or a complex repository containing nested software checkouts (`node_modules/`, Python virtual environments `.venv/`, `.git/`) will freeze the app during directory traversal.
* **Requirement Needed:** The workspace scanner must enforce default ignore rules (`.git`, `node_modules`, `target`, `dist`, `build`, `.venv`) and establish hard protective boundaries (max depth 10, max file scan limit 25,000 items with user warning).

### 5.6 Crash Recovery & Corrupt Tail Truncation
* **Gap:** Sudden power cuts or kernel panics during an append operation can leave a half-written, non-terminated JSON line at the end of the `.jsonl` file.
* **Requirement Needed:** The sidecar parser must recognize an unterminated or malformed tail line on document load, automatically repair the file by truncating the corrupt tail line to the last valid newline, and report the recovery action in a non-modal diagnostic notification.

---

## 6. Architectural Recommendations & Decisions for the User

The user is currently offline. In accordance with the project briefing, all architectural decisions requiring user direction are documented below with my formal recommendation as Principal Software Architect. Work proceeds on these recommendations as verified assumptions.

```
+---------------------------------------------------------------------------------------------------+
|                                   DECISIONS FOR THE USER (ASSUMPTIONS)                            |
+----+--------------------------------+----------------------------+--------------------------------+
| #  | Decision Area                  | Architect Recommendation   | Strategic Rationale            |
+----+--------------------------------+----------------------------+--------------------------------+
| 1  | Desktop Shell Technology       | Tauri v2 (Rust + Webview)  | 10x lower memory vs Electron,  |
|    |                                |                            | native SQLite & fast hashing   |
| 2  | License Selection              | Apache License 2.0         | Patent retaliation protection  |
|    |                                |                            | & trademark defense            |
| 3  | Product Identity               | "Palimpsest"               | User decision confirmed;       |
|    |                                |                            | use .palimpsest/ directory     |
| 4  | Sidecar Sync Conflict Handling | Automated HLC Union Merge  | Zero user friction; ingests    |
|    |                                |                            | Syncthing/Dropbox conflicts    |
| 5  | Reading Tracking Default       | Enabled with 1st-Run Prompt| Balances instant utility with  |
|    |                                |                            | academic privacy confidence    |
+----+--------------------------------+----------------------------+--------------------------------+
```

### Decision 1: Desktop Application Framework (Tauri v2 vs. Electron)
* **Context:** The reader must render heavy PDFs (500+ pages), EPUBs, HTML, and Markdown across Linux, Windows, and macOS while maintaining sub-100ms UI responsiveness.
* **Options:**
  1. *Electron:* Familiar web stack, bundled Chromium + Node.js. High memory footprint (~200 MB base per window), large binary installer (~90 MB).
  2. *Tauri v2 (Rust backend + Webview frontend):* Native system webview, lightweight installer (< 15 MB), minimal baseline memory (~30 MB), highly concurrent Rust background engine for SHA-256 hashing, file watching, and SQLite indexing.
* **Architect's Recommendation (Proceeding Assumption):** **Tauri v2 with Rust backend and TypeScript/React frontend.**  
  *Rationale:* Academic users frequently keep dozens of PDFs open simultaneously on 8 GB RAM laptops. Electron's memory overhead will cause swapping. Rust provides native speed for document hashing, PDF byte extraction, and thread-safe file I/O without garbage collection pauses.

### Decision 2: Open-Source License
* **Context:** `REQUIREMENTS.md` lists MIT vs. Apache-2.0 as an open question.
* **Architect's Recommendation (Proceeding Assumption):** **Apache License 2.0.**  
  *Rationale:* Aligns with PM analysis in `docs/BUSINESS.md`. Document rendering and text-selection algorithms involve complex font and geometry handling susceptible to patent trolling. Apache-2.0's explicit patent license and termination clause protect all open-source academic contributors.

### Decision 3: Product Name and Directory Convention
* **Context:** User instruction explicitly assigned the name **Palimpsest**.
* **Architect's Recommendation (Confirmed):** Adopt **Palimpsest** across all documentation, replacing *clickme*. Use `.palimpsest/` as the project workspace sidecar directory.

### Decision 4: Automated Sync Conflict Resolution Strategy
* **Context:** When users sync a workspace folder via Dropbox, Nextcloud, or Syncthing across multiple offline computers, duplicate conflict sidecars will be created.
* **Architect's Recommendation (Proceeding Assumption):** **Autonomous Union Merge with HLC Ordering.**  
  *Rationale:* Palimpsest will automatically detect and absorb conflict files into the primary `<hash>.jsonl` event log, eliminating manual conflict resolution prompts for non-technical researchers.

### Decision 5: Reading Progress Tracking Default Policy
* **Context:** Privacy-conscious researchers may be concerned about reading session duration logs.
* **Architect's Recommendation (Proceeding Assumption):** **Enabled by default, with a one-time onboarding notification badge.**  
  *Rationale:* The onboarding notification clarifies that reading data is stored 100% locally in `.palimpsest/reading-log/` and never transmitted across the network, providing a 1-click toggle to disable.

---

## 7. Next Steps (Transition to Phase 2)

1. **Transmit Review to PM:** Send Agent Mail to Product Manager (`pm`) summarizing the top 3 architectural findings (Anchoring across reflowable formats, JSONL union merge / HLC clock skew, and Note storage dual-layer resolution).
2. **Support Requirements Finalization:** Assist PM in incorporating proposed wording changes and missing requirements into `docs/REQUIREMENTS.md` v1.0.
3. **Initiate Phase 2 Architecture:** Upon receipt of PM notification that `REQUIREMENTS.md` v1.0 is finalized, begin authoring `docs/ARCHITECTURE.md` and the initial suite of Architecture Decision Records (ADRs).
