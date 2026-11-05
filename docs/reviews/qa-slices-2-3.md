# QA Verification Report: Slices 2 & 3 (Safe Device ID, Crash-Proof Append, Multi-Device Merge, Purge Filter)

**Reviewer:** Senior QA Engineer (`agent-1793867530011-poqhdm`)  
**Commits Verified:** [`23cc932`](https://github.com/sedam-or/scribetrace/commit/23cc932) (Core implementation), [`697bd6c`](https://github.com/sedam-or/scribetrace/commit/697bd6c) (Spec vectors & line classifier), [`008465c`](https://github.com/sedam-or/scribetrace/commit/008465c) (FORMAT §4 tie-breaker alignment)  
**Target Specification:** [`docs/BUILD-PLAN.md`](../BUILD-PLAN.md) (Slices 2 & 3), [`docs/DEFINITION-OF-DONE.md`](../DEFINITION-OF-DONE.md), [`spec/FORMAT.md`](../../spec/FORMAT.md)  
**Date:** 2026-11-06  
**Overall Verdict:** **RE-OPENED (PENDING BACKEND FOLLOW-UP ON 3 STORE-LEVEL SPEC CASES)**

---

## 1. Acceptance Criteria Verification

### Slice 2: Safe Device ID and Crash-Proof Append (Req: R-08, R-10)
> *Specification Acceptance Test:* Cloned `device.json` mints a new id and leaves the old file untouched; an injectable writer (fault-injection `LogWriter`) is cut after byte k of N for every k, in memory → read finds no malformed line, at most the last action lost. No real process kills.

| Verification Item | Test File / Implementation | Scope & Results | Verdict |
|:---|:---|:---|:---:|
| **Cloned Machine Collision** | `src-core/tests/device_clone_test.rs::test_collision_cloned_machine` | Injected a conflicting external line into `dev1.jsonl`. When local `FsDeviceStore::ensure_safe_device()` detected mismatch between disk `last_event` and `device.json`, it minted a new random UUID `dev2`. Appending proceeded to `dev2.jsonl`, leaving `dev1.jsonl` untouched with its original events intact (SEC-34). | **PASS** |
| **Restored Backup / Missing State** | `src-core/tests/device_clone_test.rs::test_collision_restored_backup_or_missing_state` | Deleted `device.json` to simulate restored backup or state wipe. Fresh device ID safely minted on startup; existing history preserved across 2 device files. | **PASS** |
| **Crash Inconsistency Collision** | `src-core/tests/device_clone_test.rs::test_collision_crash_between_append_and_state_update` | Simulated crash occurring between file append and `device.json` record update. Collision handler detected mismatch and minted a fresh device ID without corrupting prior events. | **PASS** |
| **Non-Mutating Read Invariant** | `src-core/tests/device_clone_test.rs::test_reading_never_modifies_files_byte_identical` | Read history across 10 repeated calls; disk file verified 100% byte-identical before and after (SEC-19). | **PASS** |
| **Fault-Injection Cut at Byte $k \in [1..N]$** | `src-core/tests/crash_append_test.rs::test_fault_injection_cut_after_every_byte_k_of_n` | Used `FaultyWriter` to cut append of a serialized event after every single byte $k$ from 1 to $N$. Read report confirmed: (1) 0 malformed lines returned in `visible_events`; (2) at most the single cut action was lost (baseline event 1 preserved across all $k$ cuts); (3) `skipped_lines` accurately counted torn lines. | **PASS** |
| **Unterminated Line Repair on Append** | `src-core/tests/crash_append_test.rs::test_fault_injection_cut_after_every_byte_k_of_n` (Step 4) | After a crash cut left an unterminated trailing line without `\n`, subsequent append cleanly inserted newline separator and recovered full readability. | **PASS** |

### Slice 3: Multi-Device Merge, Purge Filter & Bad Line Counting (Req: R-07, R-09)
> *Specification Acceptance Test:* SPIKE-1 property tests moved in: any order and double merge give the same result; torn and over-1 MB lines skipped, counted in the window; files unchanged; 10k events ≤500 ms. Includes the pure `visible(events, purges)` filter from SPIKE-2.

| Verification Item | Test File / Implementation | Scope & Results | Verdict |
|:---|:---|:---|:---:|
| **Merge Commutativity & Idempotency** | `src-core/tests/merge_props.rs::prop_merge_commutativity_and_idempotency` | Proptest ran across **10,000 cases** with randomized pools across 5 virtual devices, random split masks, and duplicate injections. Proved: $merge(A, B) \equiv merge(B, A)$ and $merge(merge(A, B), B) \equiv merge(A, B)$. | **PASS** |
| **Conflicting Device Tie-Breaker** | `src-core/tests/merge_props.rs::prop_same_id_conflicting_devices_commutativity` & `test_conflicting_copies_both_rules` | Proptest ran across **10,000 cases** asserting determinism per `spec/FORMAT.md §4`: cross-device duplicate event IDs select smaller `device_id` in byte order; within-device duplicates preserve earlier file line. | **PASS** |
| **Purge Visibility Invariant (Order-Independence)** | `src-core/tests/purge_props.rs::prop_random_purge_orders_identical` | Proptest ran across **10,000 permutations** of purges covering varying sequence cuts across 5 virtual devices. Visibility output proved strictly identical under arbitrary purge permutation. | **PASS** |
| **Asymmetric Clock Skew Robustness** | `src-core/tests/purge_cases.rs::test_wrong_clocks_both_ways` | Case A: Purge with past clock (`2020`) still hides covered event with future clock (`2026`). Case B: Purge with future clock (`2035`) covering `seq: 5` cannot hide uncovered event `seq: 10`. | **PASS** |
| **Unseen Offline Events Preserved** | `src-core/tests/purge_cases.rs::test_unseen_offline_event_and_second_purge` | Device B writes events 1..5 offline; Device A purges up to seq 3; events 4 and 5 remain visible until second purge covering seq 5 arrives. | **PASS** |
| **Device Not in Purge Cut Unaffected** | `src-core/tests/purge_cases.rs::test_device_not_in_purge_never_hidden_even_if_seq_zero` | Purge covering other devices never hides events from unlisted devices, even if event seq is 0. | **PASS** |
| **Bounded Line Reading (1 MB Cap) & Bad Line Counting** | `src-core/tests/crash_append_test.rs::test_bad_and_torn_lines_skipped_and_counted` | Injected 4 corrupted lines: (1) invalid JSON; (2) over-1 MB line ($>1,048,576$ bytes); (3) seq == 0; (4) unterminated trailing junk. Valid events preserved; all 4 corrupted lines skipped and counted in `ReadReport.skipped_lines`. | **PASS** |
| **10,000 Event Replay Benchmark** | `src-core/tests/benchmark_test.rs::test_replay_10_000_events_benchmark` | Full pipeline benchmark across 5 device files (2,000 events each = 10,000 total): file reading, line parsing, union merge, purge filtering, and display sorting. Measured: **320 ms** (comfortably under the $\le 500\text{ ms}$ budget). | **PASS** |
| **Format Spec Vectors Conformance** | `src-core/tests/spec_vectors.rs` | 7 automated tests validating all 24 sample vectors in `spec/vectors/` against `classify_line` and JSON Schema definitions. | **PASS** |

---

## 2. Definition of Done (15 Rows Audit)

| # | Rule | Enforced By | Audit Status & Evidence | Verdict |
|:---:|:---|:---|:---|:---:|
| **1** | `cargo fmt --check` clean | CI | `cargo fmt --all --check` clean, 0 diffs. | **PASS** |
| **2** | `cargo clippy --workspace --all-targets -D warnings` | CI | Clean across all targets with 0 warnings. | **PASS** |
| **3** | No `unwrap`/`expect`/`panic!` outside tests; `unsafe` forbidden | Workspace lints | Workspace lints enforce `deny` on `clippy::unwrap_used` and `clippy::expect_used`, and `forbid` on `unsafe_code` in `Cargo.toml`. | **PASS** |
| **4** | Typed errors (`thiserror`) in core; shell converts to plain text | Clippy + Review | `StoreError` (`Io`, `Encode`, `InvalidId`) uses `thiserror`. Shell `commands.rs` maps errors cleanly to user strings. | **PASS** |
| **5** | Unit tests + property tests for parser/id/merge/purge | CI / proptest | 10,000-case proptests in `merge_props.rs` and `purge_props.rs`, plus unit suites in `device_clone_test.rs`, `crash_append_test.rs`, `spec_vectors.rs`. | **PASS** |
| **6** | Core line coverage $\ge 80\%$ | `cargo llvm-cov` | **92.27%** line coverage across `scribetrace-core` (776 / 836 lines covered; regions 91.24%, functions 93.81%). Exceeds the $\ge 80\%$ budget. | **PASS** |
| **7** | UI: `typecheck`, `lint`, `test`, `build` clean | CI / pnpm | All 19 tests in `tests-ui/` pass (127 ms). `vite build` produces clean bundles: HTML (0.39 kB), CSS (14.23 kB), JS (235.06 kB). | **PASS** |
| **8** | UI boundary: no Node built-ins, Rpc seam | `lint-ui-boundaries.mjs` | Verified clean across all 20 UI files. | **PASS** |
| **9** | Core boundary: no tauri/TLS/notify/pdfjs | `tools/check-gates.sh` | Clean: 0 tauri in core, 0 TLS, 0 notify, 0 `pdfjs-dist`. | **PASS** |
| **10** | Budgets: crates, packages, transitive | `check-gates.sh` | Direct Rust crates: 7 ($\le 15$). Direct JS packages: 6 ($\le 6$). Transitive crates: 315 ($\le 378$). | **PASS** |
| **11** | `cargo deny` & `cargo audit` clean | CI | `cargo deny check` clean; `cargo audit` clean with documented GTK3 ignores. | **PASS** |
| **12** | Accessibility WCAG 2.2 AA | Review | Tokens, contrast ratios, and component roles maintained across all 8 UI foundation components. | **PASS** |
| **13** | Docs updated in same change | Review | Updated: `spec/FORMAT.md`, `docs/USER-GUIDE.md`, `docs/BUILD-PLAN.md`. | **PASS** |
| **14** | Security review & tests (SEC-1..34) | Review | SEC-10 (atomic write/fsync), SEC-19 (non-mutating reads), and SEC-34 (isolated `device.json` in user app config directory) verified by automated tests. | **PASS** |
| **15** | Independent review, signed-off commit | Review | Independently verified by QA (`agent-1793867530011-poqhdm`). | **PASS** |

---

## 3. Nuances and Invariants for Subsequent Slices

1. **Device ID Location (`SEC-34`):**
   `FsDeviceStore` resolves `$XDG_CONFIG_HOME/scribetrace` or `~/.config/scribetrace`. It is strictly separated from the shared workspace folder `.scribetrace/`, ensuring device IDs are never synced between machines via Git or cloud sync.
2. **`ReadReport` Integration in UI (Slice 5 / 7 / 13):**
   `FsHistoryStore::read_report` returns `ReadReport` containing `skipped_lines` and `hidden_count`. While Tauri's current `read_history` command returns `Vec<Event>` for backward compatibility with Slice 1 UI, subsequent slices (notably Slice 5 setup/error messaging and Slice 13 CLI check) should surface `skipped_lines` to the UI banner as intended by R-09.
3. **Recovery After Crashed Write:**
   The `append()` method inspects existing file endings and automatically writes a newline delimiter if a prior crash left an unterminated line. This guarantees that crash-truncated events do not contaminate subsequent valid event appends.

4. **Tech Lead Review Findings & Re-Opened Action Items (Follow-up for Backend & QA):**
   - **(a) Unknown/Future Types Kept:** In `FsHistoryStore::read_report` / `scan_file`, a well-formed line with an unknown or forward-compatible `type` (e.g. `milestone`, `mark_update`, `milestone_rename`, or external extension) must be kept and **not** counted in `skipped_lines` (`spec/FORMAT.md §3`). Currently, direct serde deserialization to fixed `Event` fails and treats unknown types as bad lines.
   - **(b) Exact Boundary Handling:** A line of exactly 1,048,576 bytes without `\n` must be judged on content (not `too_long`); only lines > 1,048,576 bytes are `too_long`.
   - **(c) Monotonic Sequence Tracking with Unknown Types:** If a device file's last line is an unknown type with `seq = N`, subsequent appends must receive `seq = N + 1`, never `N`.
   - **(d) Device ID Mismatch Validation:** If an event line's `device` field does not match the filename `<device>.jsonl`, it must be classified as `bad_value` and counted as skipped.
   - **(e) Store-Level Vector Conformance Suite:** Add an automated test feeding every vector file in `spec/vectors/` through `FsHistoryStore::read_report` and assert against `manifest.json`.

---

## 4. Final Sign-Off Recommendation

While the core multi-device merge, proptests, crash simulation, and collision logic are thoroughly verified, the store reader requires alignment with `spec/FORMAT.md` for unknown event types, byte boundaries, and filename/device matching.

**Verdict:** **RE-OPENED (PENDING BACKEND FOLLOW-UP).** Once Backend delivers the store reader updates integrating `classify_line` and device-filename validation, QA will execute the comprehensive store-level vector suite and re-evaluate for final approval.
