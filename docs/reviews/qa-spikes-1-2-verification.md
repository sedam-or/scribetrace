# QA Verification Report: SPIKE-1 (Log-Merge) & SPIKE-2 (Purge)

**Reviewer:** Senior QA Engineer (`agent-1793867530011-poqhdm`)  
**Commit Verified:** [`54ade4c`](https://github.com/sedam-or/scribetrace/commit/54ade4c)  
**Target Specification:** [`docs/PROTOTYPE-PLAN.md`](../PROTOTYPE-PLAN.md) (SPIKE-1 & SPIKE-2 Gates)  
**Date:** 2026-11-05  
**Overall Verdict:** **VERIFIED & PASSED (100% Compliance with Quality Gates)**

---

## 1. SPIKE-1: Multi-Device Log Merge & Device Collision Check

### Quality Gate Pass/Fail Criteria vs Actual Test Results

| Invariant / Metric | PROTOTYPE-PLAN Target | Actual Test Execution | Verdict | Evidence / Test Case |
|---|---|---|---|---|
| **Commutativity** | `merge(A, B) == merge(B, A)` across 10,000 cases | **10,000 / 10,000 PASS** | **PASS** | `spikes/log-merge/tests/property_tests.rs` (`prop_merge_commutativity_and_idempotency`) |
| **Idempotency** | `merge(merge(A, B), B) == merge(A, B)` across 10,000 cases | **10,000 / 10,000 PASS** | **PASS** | `spikes/log-merge/tests/property_tests.rs` |
| **Replay Throughput** | 10,000 events folded in `< 500 ms` | **94 ms** (Debug profile) | **PASS** | `spikes/log-merge/tests/replay_benchmark.rs` (`test_replay_10_000_events_benchmark`) |
| **File Immutability** | Reads leave files byte-identical | Verified: `bytes_before == bytes_after` after 10 reads | **PASS** | `spikes/log-merge/tests/table_tests.rs` (`test_reading_never_modifies_files_byte_identical`) |
| **Cloned Machine Collision** | Cloned disk image/peer divergence mints new UUID; never overwrites peer file | Verified: `dev1 != dev2`, dev2 starts seq 1, both files merged (3 total events) | **PASS** | `spikes/log-merge/tests/table_tests.rs` (`test_collision_cloned_machine`) |
| **Crash Recovery** | Crash between append and state update adopts without gap | Verified: adopts unrecorded line if seq matches without gap | **PASS** | `spikes/log-merge/tests/table_tests.rs` (`test_collision_crash_between_append_and_state_update`) |
| **Restored Backup** | Missing state on restart mints fresh device ID | Verified: fresh UUID minted, existing history preserved | **PASS** | `spikes/log-merge/tests/table_tests.rs` (`test_collision_restored_backup_or_missing_state`) |
| **Fault Tolerance** | Invalid/torn lines skipped and counted without write | Verified: bad JSON & oversized lines skipped; 0 file modifications | **PASS** | `spikes/log-merge/tests/table_tests.rs` (`test_bad_and_torn_lines_skipped_and_counted`) |

---

## 2. SPIKE-2: Sequence-Covered Purge Rule

### Quality Gate Pass/Fail Criteria vs Actual Test Results

| Invariant / Metric | PROTOTYPE-PLAN Target | Actual Test Execution | Verdict | Evidence / Test Case |
|---|---|---|---|---|
| **Exact Suppression** | 100% of covered events hidden regardless of timestamps | Verified: Event with 2026 clock hidden by 2020 purge; uncovered event seq 10 remains visible despite 2035 purge | **PASS** | `spikes/purge/tests/purge_cases.rs` (`test_wrong_clocks_both_ways`) |
| **Zero Data Leaks** | 0 hidden events in active timeline view | Verified: 0 hidden events exposed in visible vector | **PASS** | `spikes/purge/tests/purge_cases.rs` |
| **Unseen Offline Events** | Unseen offline events remain visible until subsequent purge | Verified: devB events seq 4-5 remain visible with `is_uncovered_after_purge` flag; hidden once purge covers devB:5 | **PASS** | `spikes/purge/tests/purge_cases.rs` (`test_unseen_offline_event_and_second_purge`) |
| **No Paper Sealing** | Post-purge new events start fresh history | Verified: fresh event with seq > covers starts new visible session | **PASS** | `spikes/purge/tests/purge_cases.rs` (`test_new_history_after_purge_does_not_seal_paper`) |
| **Order Invariance** | Purges in any order yield identical visibility across 10,000 random orders | **10,000 / 10,000 PASS** (0 order perturbations) | **PASS** | `spikes/purge/tests/random_orders_test.rs` (`test_10_000_random_purge_orders_identical`) |

---

## 3. Conclusion & Gate Recommendation

Both **SPIKE-1** and **SPIKE-2** pass all acceptance criteria and quality gates specified in [`docs/PROTOTYPE-PLAN.md`](../PROTOTYPE-PLAN.md).  
The core domain log merge and purge models are confirmed robust, deterministic, performant, and ready for Stage 1 integration.
