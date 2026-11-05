# ScribeTrace Prototype Spike Results (SPIKE-1 to SPIKE-4)

**Document:** `docs/spikes/RESULTS.md`  
**Author:** DevOps Engineer (`agent-1793867530011-yoc5o5`)  
**Date:** 2026-11-05  
**Review Status:** SPIKE-1 & SPIKE-2 verified by QA ([`docs/reviews/qa-spikes-1-2-verification.md`](../reviews/qa-spikes-1-2-verification.md)) and reviewed by Architect; SPIKE-3 desk-checked ([`docs/spikes/zotero-desk-check.md`](zotero-desk-check.md)); SPIKE-4 minimal Tauri v2 webview app and multi-OS CI workflow constructed.

---

## 1. Quality Gate Summary Table

| Spike | Invariant Under Test | QA Threshold | Measured Result | Status | Reproduce Command |
|---|---|---|---|---|---|
| **SPIKE-1** | Commutativity `merge(A, B) == merge(B, A)` | 10,000 / 10,000 runs | **10,000 / 10,000 PASS** | **PASS** | `cd spikes/log-merge && cargo test --release --test property_tests` |
| **SPIKE-1** | Idempotency `merge(merge(A,B), B) == merge(A,B)` | 10,000 / 10,000 runs | **10,000 / 10,000 PASS** | **PASS** | `cd spikes/log-merge && cargo test --release --test property_tests` |
| **SPIKE-1** | Same-ID Conflicting Payload Commutativity | Total order tie-break | **Deterministic match** | **PASS** | `cd spikes/log-merge && cargo test --release --test property_tests` |
| **SPIKE-1** | 10,000 Event Replay Latency | `< 500 ms` | **41 ms** (12x faster) | **PASS** | `cd spikes/log-merge && cargo test --release --test replay_benchmark` |
| **SPIKE-1** | Log File Immutability | Byte-identical reads | **Bytes identical** (SHA-256 match) | **PASS** | `cd spikes/log-merge && cargo test --test table_tests` |
| **SPIKE-1** | Cloned Device Collision Handling | Mismatch mints new UUID | **Fresh UUID minted, 0 file overwrites** | **PASS** | `cd spikes/log-merge && cargo test --test table_tests` |
| **SPIKE-1** | Backup Restore / Missing State | Detects gap, mints UUID | **Preserves files, mints UUID** | **PASS** | `cd spikes/log-merge && cargo test --test table_tests` |
| **SPIKE-1** | Crash Before State Update | Detects unindexed line | **Adopts safely, mints UUID** | **PASS** | `cd spikes/log-merge && cargo test --test table_tests` |
| **SPIKE-1** | Torn / Over-1MB Line Handling | Skips without mutating file | **Skipped & counted; 0 mutations** | **PASS** | `cd spikes/log-merge && cargo test --test table_tests` |
| **SPIKE-2** | Clock Independence (Wrong Clocks) | 100% covered events hidden | **100% hidden regardless of `ts`** | **PASS** | `cd spikes/purge && cargo test --release --test purge_cases` |
| **SPIKE-2** | Unseen Offline Event Visibility | Uncovered seq visible | **Uncovered visible; 2nd purge hides** | **PASS** | `cd spikes/purge && cargo test --release --test purge_cases` |
| **SPIKE-2** | Unrelated Device Protection | Devices not in covers not hidden | **Never hidden (seq >= 0 preserved)** | **PASS** | `cd spikes/purge && cargo test --release --test purge_cases` |
| **SPIKE-2** | Multiple Purge Order Invariance | Identical in 10,000 orders | **10,000 / 10,000 PASS** | **PASS** | `cd spikes/purge && cargo test --release --test random_orders_test` |
| **SPIKE-2** | Fresh History Post-Purge | Paper ID not sealed | **New seq visible; history restarts** | **PASS** | `cd spikes/purge && cargo test --release --test purge_cases` |
| **SPIKE-3** | Zotero 7 Local HTTP Interface Contract | Metadata + rects + colors | **All fields documented** | **documented, not live-verified** | Desk check: `docs/spikes/zotero-desk-check.md` |
| **SPIKE-4** | Cold First-Page Render (50 MB / 500p PDF) | `< 1500 ms` (ceiling 2000 ms)| *Node parse baseline: 158 ms* | **NOT RUN** (Webview CI open) | `cd spikes/tauri-webview && cargo build --release` |
| **SPIKE-4** | Idle Memory RSS (after 10 page turns) | `< 250 MB` | *Node parse baseline: 166.3 MB* | **NOT RUN** (Webview CI open) | `cd spikes/tauri-webview && ./measure.sh` |
| **SPIKE-4** | Ground-Truth Text Layer Selection | Match ground-truth text | *Node parse baseline: Matched* | **NOT RUN** (Webview CI open) | `cd spikes/tauri-webview && ./measure.sh` |

---

## 2. Spike Findings & Surprises

### SPIKE-1: Log Merge
- **Property Test Insight & Full Canonical Tie-Breaker:** In response to Architect review, `merge_events` was upgraded to compare the full canonical serialized JSON representation of conflicting records if identical UUIDs occur with divergent payloads. A dedicated property test (`prop_same_id_conflicting_payloads_commutativity`) was added and verified.
- **Sequence Validation:** Enforced `seq >= 1` across both domain and storage reader (rejecting `seq == 0`).
- **Throughput:** Replaying 10,000 JSONL events across 5 device files executed in **41 ms** in release mode (< 500 ms QA budget).

### SPIKE-2: Purge
- **Supremum Property & Device Scoping:** Fixed coverage evaluation so that events are hidden *only* if the event's device is present in `effective_covers` and `seq <= cover`. Unrelated devices not named in any purge are never hidden, even if sequence numbers are 0.
- **No Per-Event Uncovered Flag:** In alignment with Architect's amended ADR 0007, the misleading `is_uncovered_after_purge` flag was removed from the visibility result. Uncovered events simply appear chronologically in the mark stream.
- **Order Invariance:** 10,000 random permutations of 6 distinct purges produced identical visibility output.

### SPIKE-3: Zotero Desk Check
- **Local API Contract:** Documented that Zotero 7's built-in HTTP server on `127.0.0.1:23119` exposes annotations under `/api/users/0/items/<attachmentKey>/children?itemType=annotation`.
- **Field Completeness:** Bounding boxes are returned in `annotationPosition` (`pageIndex` and `rects`), alongside `annotationType`, `annotationColor`, and `annotationText`.
- **Honest Status:** Remains strictly **documented, not live-verified** until the maintainer executes live curl probes against an active Zotero 7 instance.

### SPIKE-4: Tauri WebView & PDF.js
- **Distinction Between Node Parse Baseline and Real Webview:** Node-based measurements (158 ms cold parse, 166.3 MB RSS) reflect only PDF.js JavaScript parsing overhead, not native WebKitGTK / WebView2 / WKWebView rendering.
- **Real Webview App Built:** A minimal Tauri v2 application (`spikes/tauri-webview`) has been initialized with native process RSS inspection (`/proc/self/statm`) and IPC metrics reporting.
- **Status:** Marked **NOT RUN** locally because the development environment is headless without graphical display (`$DISPLAY` / `$WAYLAND_DISPLAY`); real webview rendering measurements are deferred to the multi-OS CI workflow (`.github/workflows/spike4.yml`) across Ubuntu, macOS, and Windows runners.

---

## 3. Ambiguities & Resolution Log

1. **ADR 0007 Same-ID Conflict Resolution:** Addressed Architect review item 1: full canonical serialization tie-breaker guarantees strict mathematical commutativity across all inputs.
2. **ADR 0007 Unrelated Device Scoping:** Addressed Architect review item 2: missing device in covers map never triggers suppression.
3. **ADR 0007 Flag Simplification:** Addressed Architect review item 3: removed per-event uncovered flag.
4. **SPIKE-4 Status Transparency:** Addressed Chief-of-Staff honesty requirement: Node figures isolated as baseline; webview gate kept open awaiting 3-OS CI run.

---

## 4. Gate Conclusion
- **Gates 1 & 2 (Log-Merge & Purge):** **PASS** (verified by QA and Architect).
- **Gate 3 (Zotero Local API):** **DESK CHECK ONLY** (documented, not live-verified; live check stays with user).
- **Gate 4 (Lightness & Shell Webview):** **OPEN** (minimal Tauri v2 app built; awaiting multi-OS CI webview runs on Linux, Windows, and macOS).
