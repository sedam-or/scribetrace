# ADR 0008: Notes and one-way Obsidian export

**Status:** Accepted (slimmed in Round 8)  
**Date:** 2026-11-05 · **Supersedes:** [archived ADR 0005](archive/0005-dual-layer-notes-and-reading-log.md)

## Context
Users want notes readable without the app and usable in Obsidian. A live mirror or file watcher adds background work and race conditions.

## Decision
- **`note.md` is the current note; every save appends a `note_revision` holding the full text** (no diffs; text capped at 512 KB to stay under the line limit). *Why:* a full snapshot restores without replaying anything.
- **Outside edits:** before every save (and when a paper opens) the app compares the file with the last revision. If it differs, the outside text is first logged as a revision, then our text is written. Last writer wins on the file, **both versions stay in history** and can be restored. No dialog, no watcher.
- **Export on click** to a folder the user picks, into a subfolder `ScribeTrace/` that we create. Each generated file starts with a marker line; we overwrite only files that carry it, and report the others as skipped. Edits made to exported copies are regenerated away (stated in the UI). Nothing flows back from Obsidian.
- No `scribetrace://` handler in Stage 1.

## Consequences
+ Plain files, no background process, few races. − Export is not live; Obsidian edits are not imported (Later: only if users ask).

Security: Markdown is parsed to a tree and sanitized (SEC-7, 9); export paths come from a native dialog and stay in the marked subfolder (SEC-18, 36); file names from titles are sanitized (SEC-21).
