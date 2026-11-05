# Round 5: Technical options for the staged MVP

Builds on [03-decision-memo](03-decision-memo.md) and [04-high-level-design](04-high-level-design.md). Old choices live in [ARCHITECTURE.md](../ARCHITECTURE.md) and ADRs 0001–0005, written for a wider scope (4 formats, folder-of-papers workspace, live notes mirror). Each decision: options, pick, and what happens to the old choice. Stage 1 = importer + history + notes + quote + export (no reader). Stage 2 = own PDF/Markdown reader.

## What the new scope changes (read first)

1. **Stage 1 renders no PDF.** Zotero marks arrive with their page, position and text. PDF rendering and text anchoring only matter from Stage 2.
2. **A paper is identified by Zotero's key, not by living in the folder.** Old design: papers sit in the project folder. New: the workspace holds history for papers that live in Zotero's own storage. We never write there.
3. **Only one network-like thing exists:** talking to Zotero on this same computer. The old "zero network" rule must say "no outbound traffic; loopback to Zotero only, and only on click".
4. **No background watching.** The old file watcher, conflict-file harvester and external-edit tracking go.

## Decisions

| # | Decision | Options | Pick | Old choice |
|---|---|---|---|---|
| 1 | Delivery | A. Stand-alone app. B. Zotero plug-in. C. Obsidian plug-in. | **A**, with the core built as a separate library plus the command-line tool, so a plug-in front end stays possible. | New. |
| 2 | Desktop shell | A. Tauri v2 (Rust core, system web view). B. Electron. C. Native toolkit. | **A, with a checkpoint at Stage 2 start.** | Keep (ADR 0001). |
| 3 | PDF rendering + anchoring (Stage 2) | A. PDF.js + page/box + quote repair. B. Native PDF engine. C. Text offsets only. | **A.** | Keep rendering, simplify anchoring (ADR 0002, 0003). |
| 4 | History log and merge | A. One append-only line file per paper **per device**. B. Embedded database. C. Content-addressed object store. | **A.** | Keep format, change merge (ADR 0004). |
| 5 | Notes storage | A. Note text as plain file + revisions in the log. B. Notes only inside the log. C. Notes only as user-visible files. | **A.** | Change (ADR 0005). |
| 6 | Zotero integration | A. Zotero's local interface. B. Read Zotero's database file. C. User-exported files. D. Zotero plug-in. | **A**, B only as a fallback if the check fails. | New. |
| 7 | Obsidian | A. One-way export on click. B. Live mirror. C. Obsidian plug-in. | **A.** | Drop live projection (ADR 0005). |
| 8 | Open format + tool | A. Own documented event format, W3C export for state. B. Adopt W3C Web Annotation as the format. C. Spec later. | **A**, spec as its own versioned folder with schema and test files. | New. |
| 9 | Packaging, updates | A. Installers + signed auto-update. B. Installers, manual update, package managers. C. Store only. | **B** for the MVP. | Change (SEC-28 updater deferred). |

## Reasoning, per decision

**1. Delivery.** A Zotero plug-in could read marks directly, with no setting to switch on, but it is tied to Zotero's internal changes, cannot grow into a reader for Markdown, and the plug-in language cannot be shared with the command-line tool. An Obsidian plug-in lives inside a closed-source host and cannot read Zotero's marks as cleanly. Stand-alone costs us an installer and a window, but it is the only shape that carries Stage 2. *Risk:* we own packaging on three systems.

**2. Shell.** Tauri keeps the core in Rust, shared by the app and the command-line tool (one reader and validator of the format, not two). It is small and has a narrow permission model. *Objection I expect:* system web views differ per OS; PDF.js behaviour on Linux's web view is the known weak spot. Electron avoids that at ~10× the size and a JavaScript core. Stage 1 is not affected; **at Stage 2 start we run a one-day PDF rendering test on all three web views and switch if it fails.** Skill fit matters more than benchmarks: if nobody on the team writes Rust, pick Electron now. That is a decision for the user.

**3. PDF and anchoring.** Keep PDF.js (Apache-2.0). Drop the old four-tier cascade built for EPUB/HTML (CFI, DOM paths). New rule: a mark stores **page + boxes + quote with a little text before and after**. On open: boxes first; if the text there no longer matches, search the quote on that page; if still not found, a bounded fuzzy search on that page; else mark the entry **orphaned**, never delete. *Never trust text offsets across tools:* Zotero's and PDF.js's text extraction differ, so imported marks keep Zotero's page and box and only use the quote for repair. Markdown marks anchor against the **source text** (quote + context + line), not the rendered page, so renderer changes cannot move them.

**4. History log.** The old design had one file per paper, written from every device, plus a sync-conflict harvester. Better: **each device writes only its own file** (`<paper>/<device>.jsonl`), so two writers never touch the same file and sync tools cannot create conflicts. Reading = union of all device files, ordered by (logical time, device, counter). Each line has a unique id, a format version, an author label. This is also friendlier to Git (append-only, clean diffs, no merge driver). **Dropped from the MVP:** hash chain (no verification story yet), snapshots/compaction (add only after measurement shows replay is slow), the file watcher. **Kept:** crash-safe append, tolerant reader (bad line skipped and set aside), size limit per line, bound on how far in the future a timestamp may be. *Open problem:* "forget this paper's history" deletes this device's files, but a synced copy on another machine would bring it back. Proposal: write a small "erase requested" marker that other devices offer to honour on next open. Security and QA, please attack this.

**5. Notes.** Current note text is a plain Markdown file in the workspace (readable without the app, satisfies "files you own"). Every save of that note appends a revision to the log. If the file changed outside the app, we notice **on open** by comparing with the last revision and record "edited outside", with no watcher. Obsidian never edits this file: it works on the exported copy (decision 7), so nothing flows back.

**6. Zotero.** Facts checked 2026-11-05: Zotero's local interface listens on this computer only, must be enabled by the user under Settings → Advanced, answers with "forbidden" if not, needs no key for reading, and hands out the PDF's location on disk ([Zotero docs](https://www.zotero.org/support/dev/web_api/v3/local_api)). **Not documented, must be tested in the one-day check:** whether marks (annotation items) come back with type, text, comment, colour, page position and added/modified dates; whether paging beyond 100 works; whether marks in the trash are visible; group libraries use another path. Links back use `zotero://open-pdf/library/items/<key>?page=<n>&annotation=<key>`; this form works but is only known from forum threads, and Zotero staff plan to rename it while keeping the old name as an alias ([forum](https://forums.zotero.org/discussion/comment/456903/)), so treat as best effort. Deleted marks cannot be dated: refresh shows "gone between milestone X and Y". *Plan B* (reading Zotero's database file) is fragile, locked while Zotero runs, and not a public interface; use only for a one-off read of a copy.

**7. Obsidian.** Export writes Markdown into a folder the user picks, inside a subfolder we create and mark. We overwrite only files we wrote and that are unchanged since; otherwise we report and skip. No `palimpsest://` link scheme in Stage 1: registering a system-wide link handler adds attack surface and has no use without a reader. Links point to Zotero until Stage 2.

**8. Format and tool.** W3C Web Annotation describes **an annotation's state**, not its history, so it cannot be the format; we offer it as an export of the current state. The spec ships as its own folder: written description, schema for every line, sample files that good and bad readers must handle the same way. Marked **unstable v0**. The tool commands: check, show timeline, export, merge. Package names from pm's memo (`palimpsest-core`, `palimpsest-format`).

**9. Packaging.** Auto-update means an outbound connection, a signing key to protect, and an attack path into every user's machine, against our "no network" pledge. For the MVP: installers on the release page with published checksums and build proof, plus package managers. Cost to check with devops: Apple developer membership, a Windows signing certificate (some open-source programmes exist; not verified here).

## Cost picture (what is expensive)

Cheap: history log, notes, export, tool (all small file handling in one core). Medium: Zotero import (external, undocumented parts). Expensive: installers on 3 systems, Stage 2 reader and anchoring. Staging keeps the expensive part behind the gate.

## Where I expect objections

Security: loopback exception, erase marker, no update channel (I think it helps). QA: how to test import without a live Zotero (answer: recorded sample responses), and how to test merge (answer: two device files, shuffled order, same result). Devops: three-OS installers and signing cost.
