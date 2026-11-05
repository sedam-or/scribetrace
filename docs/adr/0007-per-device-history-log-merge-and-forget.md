# ADR 0007: Per-device history log, merge and forget

**Status:** Accepted (slimmed in Round 8)  
**Date:** 2026-11-05 · **Supersedes:** [archived ADR 0004](archive/0004-immutable-event-log-storage-and-merge.md) · **Reviews:** [QA](../reviews/qa-architecture-v2.md), [security](../reviews/security-architecture-v2.md)

## Context
History is the core. The folder may be synced by Git, Syncthing or Dropbox. Two machines writing one file creates sync conflicts; a clock-based merge rule is hard to test and can be fooled by a wrong clock.

## Decision
```
.scribetrace/papers/<paper-id>/<device-id>.jsonl   one writer per file, append-only, never rewritten
.scribetrace/papers/<paper-id>/note.md             ADR 0008
app storage (not synced): device.json = { device_id, papers: { <paper-id>: { last_id } } }
```
- **Line:** `v`, `id` (UUID), `device`, `seq` (per-device counter), `ts` (wall clock, **display only**), `author` (default "Me"), `type`, `payload`. Types: `mark_import`, `mark_create|update|delete|restore`, `mark_gone`, `note_revision`, `milestone`, `purge`. paper-id: `z_<itemKey>` or `h_<content hash>`, characters `[A-Za-z0-9_-]`.
- **Read = union by `id`**, shown in order `(ts, device, seq)`. Nothing else depends on a clock. *Why:* union of sets is commutative and idempotent by construction, so no logical clock is needed.
- **Device id** = random UUID, never from host or user name. *Collision check*, before the first append in a session: the last line of our file must have the `id` recorded in `device.json`; otherwise (cloned machine, restored backup, crash before the record was saved) mint a **new** device id and leave the old file alone (it stays readable as another device). A new id has no file, so it cannot loop. *Why:* the only way to lose data is two writers on one file.
- **Bad lines** (unparsable, over 1 MB, wrong schema) are skipped on read and counted in the UI and in `check`. Files are never repaired or truncated. An unterminated last line of our own file is closed with a newline before the next append.
- **Forget:** delete this paper's files on this machine, then write one `purge` with `covers = {device: highest seq seen}`. Events with `seq ≤ covers[device]` are hidden everywhere, whatever the clocks say. Events the purger had not seen (written offline) stay visible, marked "not covered by the erase"; forgetting again hides them. A purge does not seal the paper-id; new events start fresh history. Other machines hide the covered events at once and show a banner with a "Delete hidden files" button; no prompt, no stored answer. UI says copies in sync services or Git may remain.
- **Not built (Later):** snapshots (add if replay of 10,000 events exceeds 1 s), hash chain, file watcher, clock ordering.

## Consequences
+ Sync tools cannot conflict; Git diffs are clean; merge and purge are pure functions tested by property tests (SPIKE-1, 2). − Many small files per paper over time. − A crash can cost one extra device file (harmless). − Unseen offline events survive a purge until the user forgets again.

Security: every line is untrusted input (schema, size, escaped on display; SEC-8, 12, 13); path jail on ids (SEC-14, 21); a wrong or hostile clock cannot hide or reorder anything that matters (SEC-10 holds trivially); SEC-34 device id.
