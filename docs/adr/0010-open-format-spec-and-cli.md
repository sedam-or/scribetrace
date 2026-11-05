# ADR 0010: Open format specification and command-line tool

**Status:** Accepted (slimmed in Round 8)  
**Date:** 2026-11-05

## Context
"Open annotation format" is a fixed user decision. It must be a deliverable people can check without the app, and it must not be promised stable before an outside tool uses it. W3C Web Annotation describes an annotation's *state*, not its *history*, so it cannot be the format.

## Decision
- Publish `spec/FORMAT.md` (the event lines of [ADR 0007](0007-per-device-history-log-merge-and-forget.md)), a schema per line type, and good/bad sample files (`spec/vectors/`) that every reader must treat the same way. Label it **unstable v0**.
- A CLI (`scribetrace-cli`; the name `scribetrace` is the app's own binary) built from the same core with three commands: `check` (validate a folder, list skipped lines), `timeline` (print milestones), `export` (Markdown). It never edits a document and never uses the network.
- Readers ignore and preserve unknown fields; newer apps read older files without migration. *Why:* no migration code to write or test.
- **Later:** W3C JSON-LD export of current state (add when a tool maintainer asks); `merge` dry run (the timeline already shows the union).

## Consequences
+ Sceptics and tool builders can verify claims; the vectors double as our conformance tests, so the spec costs little extra. − Early readers may break while v0 changes.

Security: the spec states size limits, id characters and that all text is untrusted; app and CLI share one validator (SEC-8, 12, 13, 14).
