# Fix list for ARCHITECTURE v2.0 (from QA and Security delta reviews)

Sources: docs/reviews/qa-architecture-v2.md, docs/reviews/security-architecture-v2.md

## Must fix (QA blockers)
1. Device-id collision detection: per-user `device_state.json` with last_seq and file size per paper; define what happens when the file on disk differs; no regeneration loops.
2. Quarantine is non-destructive: `<device>.jsonl` is never rewritten. Bad lines (torn, more than 24 h in the future) are recorded with offset and reason in `.palimpsest/corrupt/` and skipped on read.
3. Day-1 live Zotero contract check is an explicit gate before Stage 1 coding (otherwise Decision for the user #4).

## Also fix
4. Purge vs clock skew: an offline machine with a fast clock can resurrect pre-purge events (event.hlc > purge.hlc). State the rule. State whether purge seals a paper-id or a fresh session may start.
5. Peer purge prompt lifecycle: the answer is stored locally; no repeat prompt.

## Add as spec lines (nice-to-have)
6. Update manifest: schema (version SemVer, date, text-only notes), 10 KB cap, 5 s timeout, downgrade handling, error contracts for 404/500/TLS/corrupt JSON.
7. note.md edited outside while in-app edits are unsaved: who wins; revision payload = full text snapshot.
8. Zotero broker: 15 MB response cap, 5 s timeout.
9. Stage 2: refuse Zotero attachment paths that resolve through a symlink outside the storage folder.

## Deliverables
- Update ARCHITECTURE.md and the ADRs; log both reviews in the Review log.
- Write one short docs/PROTOTYPE-PLAN.md with QA's SPIKE-1..4 (hypothesis, test, pass/fail numbers, time box).
- Edit only ARCHITECTURE.md, docs/adr/, docs/PROTOTYPE-PLAN.md.
