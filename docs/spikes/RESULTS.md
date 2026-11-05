# ScribeTrace Prototype Spike Results (SPIKE-1 to SPIKE-4)

**Document:** `docs/spikes/RESULTS.md`  
**Author:** DevOps Engineer (`agent-1793867530011-yoc5o5`)  
**Date:** 2026-11-05  
**Review Status:** SPIKE-1 & SPIKE-2 verified by QA ([`docs/reviews/qa-spikes-1-2-verification.md`](../reviews/qa-spikes-1-2-verification.md)); SPIKE-3 desk check documented ([`docs/spikes/zotero-desk-check.md`](zotero-desk-check.md)); SPIKE-4 measured against lightness budget.

---

## 1. Quality Gate Summary Table

| Spike | Invariant Under Test | QA Threshold | Measured Result | Status | Reproduce Command |
|---|---|---|---|---|---|
| **SPIKE-1** | Commutativity `merge(A, B) == merge(B, A)` | 10,000 / 10,000 runs | **10,000 / 10,000 PASS** | **PASS** | `cd spikes/log-merge && cargo test --release --test property_tests` |
| **SPIKE-1** | Idempotency `merge(merge(A,B), B) == merge(A,B)` | 10,000 / 10,000 runs | **10,000 / 10,000 PASS** | **PASS** | `cd spikes/log-merge && cargo test --release --test property_tests` |
| **SPIKE-1** | 10,000 Event Replay Latency | `< 500 ms` | **41 ms** (12x faster) | **PASS** | `cd spikes/log-merge && cargo test --release --test replay_benchmark` |
| **SPIKE-1** | Log File Immutability | Byte-identical reads | **Bytes identical** (SHA-256 match) | **PASS** | `cd spikes/log-merge && cargo test --test table_tests` |
| **SPIKE-1** | Cloned Device Collision Handling | Mismatch mints new UUID | **Fresh UUID minted, 0 file overwrites** | **PASS** | `cd spikes/log-merge && cargo test --test table_tests` |
| **SPIKE-1** | Backup Restore / Missing State | Detects gap, mints UUID | **Preserves files, mints UUID** | **PASS** | `cd spikes/log-merge && cargo test --test table_tests` |
| **SPIKE-1** | Crash Before State Update | Detects unindexed line | **Adopts safely, mints UUID** | **PASS** | `cd spikes/log-merge && cargo test --test table_tests` |
| **SPIKE-1** | Torn / Over-1MB Line Handling | Skips without mutating file | **Skipped & counted; 0 mutations** | **PASS** | `cd spikes/log-merge && cargo test --test table_tests` |
| **SPIKE-2** | Clock Independence (Wrong Clocks) | 100% covered events hidden | **100% hidden regardless of `ts`** | **PASS** | `cd spikes/purge && cargo test --release --test purge_cases` |
| **SPIKE-2** | Unseen Offline Event Visibility | Uncovered seq visible | **Uncovered visible; 2nd purge hides** | **PASS** | `cd spikes/purge && cargo test --release --test purge_cases` |
| **SPIKE-2** | Multiple Purge Order Invariance | Identical in 10,000 orders | **10,000 / 10,000 PASS** | **PASS** | `cd spikes/purge && cargo test --release --test random_orders_test` |
| **SPIKE-2** | Fresh History Post-Purge | Paper ID not sealed | **New seq visible; history restarts** | **PASS** | `cd spikes/purge && cargo test --release --test purge_cases` |
| **SPIKE-3** | Zotero 7 Local HTTP Interface Contract | Metadata + rects + colors | **All fields documented** | **PASS (Desk)** | Desk check: `docs/spikes/zotero-desk-check.md` |
| **SPIKE-4** | Cold First-Page Render (50 MB / 500p PDF) | `< 1500 ms` (ceiling 2000 ms)| **158 ms** (9.5x under budget) | **PASS** | `cd spikes/tauri-webview && ./measure.sh` |
| **SPIKE-4** | Idle Memory RSS (after 10 page turns) | `< 250 MB` | **166.3 MB** | **PASS** | `cd spikes/tauri-webview && ./measure.sh` |
| **SPIKE-4** | Ground-Truth Text Layer Selection | Match ground-truth text | **Matched exactly** | **PASS** | `cd spikes/tauri-webview && ./measure.sh` |

---

## 2. Spike Findings & Surprises

### SPIKE-1: Log Merge
- **Property Test Insight:** During early proptest fuzzing, when two synthetic events had the same UUID (`id`) but conflicting timestamps or payloads, simple `HashMap::or_insert` caused merge order asymmetry depending on whether set A or set B was iterated first. We resolved this by adding a canonical key tie-breaker in `merge_events`, guaranteeing that set union is 100% commutative and idempotent under all inputs, even when malformed duplicates occur.
- **Throughput:** Replaying 10,000 JSONL events across 5 device files took **41 ms** in release mode, far outperforming the 500 ms QA budget ceiling.

### SPIKE-2: Purge
- **Supremum Property:** Modeling multiple purges as an element-wise supremum (`max(seq)`) over per-device coverage maps mathematically guarantees that `visible(events, purges)` is commutative and associative. Shuffling 6 distinct purges across 10,000 random permutations produced zero deviations.
- **Clarification on ADR 0007:** As noted by the Architect during review, a purge event hides events with `seq <= covers[device]`. Offline events that were not covered simply remain visible and appear chronologically after the purge marker; no per-event flag is needed in storage.

### SPIKE-3: Zotero Desk Check
- **Local API Endpoint:** Zotero 7's built-in HTTP server on `127.0.0.1:23119` exposes annotations under `/api/users/0/items/<attachmentKey>/children?itemType=annotation`.
- **Field Completeness:** Bounding boxes are returned in the JSON-encoded `annotationPosition` field (`pageIndex` and `rects`), alongside `annotationType`, `annotationColor`, `annotationText`, and `annotationComment`.
- **Pre-requisite:** User must check *"Allow other applications on this computer to communicate with Zotero"* under Preferences → Advanced.

### SPIKE-4: Tauri WebView & PDF.js
- **Rendering Speed:** Pinned PDF.js v4 renders page 1 of a 52.9 MB / 500-page synthetic PDF in **158 ms** on a cold start, well within the 1.5s lightness budget.
- **Memory Footprint:** Peak RSS after 10 page navigations remained at **166.3 MB**, comfortably within the 250 MB budget ceiling.
- **Fixture Safety:** The 50 MB synthetic PDF is created on demand during measurement and automatically deleted; zero large binary fixtures are tracked in git.

---

## 3. Ambiguities & Resolution Log

1. **ADR 0007 Event Ordering:** While `(ts, device, seq)` is specified for UI display ordering, domain union merge relies strictly on `(device, seq, id)` to preserve strict clock independence.
2. **ADR 0007 Uncovered Event Flag:** ADR 0007 originally suggested marking unseen offline events with a flag. The Architect amended this: new events and late offline events cannot be distinguished by `seq` alone, so uncovered events are simply displayed in the timeline after the purge event without a storage-level flag.
3. **SPIKE-3 Live Execution:** Since Zotero 7 is not headless and cannot be installed in the CI / agent sandbox, SPIKE-3 is verified via desk research and a curl reproduction script for the user/maintainer.

---

## 4. Gate Conclusion
All prototype gates (SPIKE-1, SPIKE-2, SPIKE-3 desk check, SPIKE-4) pass their respective acceptance criteria and quality thresholds. Stage 1 engineering may proceed upon user confirmation.
