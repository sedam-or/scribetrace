# QA Verification Report: Slice 6 (Milestones)

**Verifier:** QA Lead (`agent-1793867530011-poqhdm`)
**Target commit:** `a2ebf2e` (security fix on Slice 6 commit `3de5b33`)
**Security review:** `6fbe7ca` (Backend; required finding closed in `0aa2c6a`)
**Verdict:** **APPROVED**
**Date:** 2026-11-07

## Summary

Slice 6 satisfies R-11 for milestone creation and append-only renames. The follow-up security fix rejects `milestone` and `milestone_rename` payloads with an omitted `label`, while accepting an explicit `null`, matching `spec/FORMAT.md` §3 and `spec/schema/line.schema.json`.

## Verification

| Check | Evidence | Result |
|---|---|---|
| Label key presence | `bad_payload_milestone_no_label.jsonl` and `bad_payload_rename_no_label.jsonl` are in the shared vector manifest and classify as `bad_payload`. | **PASS** |
| Explicit null remains valid | `good/milestone.jsonl` contains `"label": null`; the vector conformance suite accepts it. | **PASS** |
| Zero-valued cut entries are rejected | `bad_payload_upto_zero.jsonl` is classified as `bad_payload` by the vector suite. | **PASS** |
| Import and refresh milestones | First import writes one import milestone; unchanged refresh writes none; changed refresh writes one refresh milestone with the new mark covered by its cut. | **PASS** |
| Append-only rename | Rename appends one line and preserves all previous JSONL bytes. | **PASS** |
| Hidden milestone rename | `rename_of_a_hidden_milestone_writes_nothing` returns false and leaves history bytes unchanged. | **PASS** |
| Purge-aware import deduplication | `purged_mark_is_not_imported_again_on_refresh` imports no duplicate and writes no new milestone or event. | **PASS** |

## Test execution

- `cargo test -p scribetrace-core`: **78 passed, 0 failed**, including all 9 `milestones_slice6` tests and all 7 `spec_vectors` tests.
- `cargo clippy -p scribetrace-core --all-targets -- -D warnings`: **PASS**.
- `cargo fmt --all --check`: **PASS**.

Linux E2E run [42336067290](https://github.com/sedam-or/scribetrace/actions/runs/42336067290) on main `60dad28`, after the Slice 6 fix, passed at **192.44 MB** under the 200 MB gate. The real-window setup, native folder chooser visibility, and responsive WebView checks also passed.
