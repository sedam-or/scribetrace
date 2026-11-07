# Security Review: Slice 6 Milestones (R-11)

**Reviewed commit:** `3de5b33` · **Reviewer:** Backend · **Review verdict:** Changes requested; finding closed by `a2ebf2e`.

This review is against commit `3de5b33` only. No source changes were made as part of the review.

## Required finding

### Missing `label` fields are accepted as valid payloads

`spec/schema/line.schema.json` requires `label` in both the `milestone` and `milestone_rename` payloads (§3). The Rust payloads represent those fields as `Option<String>` in `src-core/src/domain/event.rs`; Serde interprets an omitted `Option` field as `None`. The typed parser in `src-core/src/domain/line.rs` deserializes directly into `Event` and validates the label's length and the target UUID, but never checks that the `label` key was present. Therefore a sidecar line omitting `label` is classified as valid, although the schema requires it.

This is a validation and cross-reader consistency defect for untrusted, syncable sidecar data: this reader may accept a line another schema-conforming reader rejects.

**Resolution:** `a2ebf2e` requires the key while keeping explicit `null` valid, adds `bad_payload` vectors for both omissions, and adds hidden-rename and purge-dedup regression tests. QA verification is tracked separately; this note does not claim a QA result.

## Checks that passed

- **Import deduplication:** `ImportMarks` builds its known-key set from `ReadReport.raw_events` before purge filtering. The store populates that field from valid events in all scanned device files, including events hidden by purges. The import flow appends only unseen keys.
- **Milestone creation:** a milestone is appended only when `changed_since_last` finds a `mark_import` beyond the latest visible milestone cut. An unchanged refresh produces no milestone.
- **Rename behavior:** the rename use case reads visibility-filtered events and checks that the target is a visible milestone before appending. Missing or hidden targets therefore write nothing; existing lines are not edited. The label limit is enforced in both the use case and the line validator.
- **Bad payload vectors:** `bad_payload_upto_zero.jsonl`, `bad_payload_label_too_long.jsonl`, and `bad_payload_rename_not_uuid.jsonl` are listed as `bad_payload`; `spec_vectors::every_vector_gets_the_promised_verdict` checks the manifest.
- **Network and IPC:** the commit adds no network call, IPC command, or capability. The `src-tauri` change only updates an existing IPC test's expected event count.

## Coverage notes

- There is no focused test for renaming a purge-hidden milestone. The implementation currently guards this correctly. A regression test should assert that the call returns `false` and leaves the JSONL bytes unchanged; that protects the no-write and privacy invariant for syncable labels. This is a test gap, not a current security flaw.
- The import tests cover a mark removed from Zotero and later returned, but not a mark hidden by a purge and returned. The current `raw_events` dedup path handles that case; a purge-specific regression test would protect the forget semantics.
- The zero-`upto` validator case is covered by the bad-payload vector. A separate application-level test of generated cuts is not necessary to establish the parser's rejection of zero.

**Verification method:** read-only review of the target commit's source, tests, schema, vectors, and `FORMAT.md` §§3, 7, and 8. Tests were not run for this review.
