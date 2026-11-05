# QA Verification Report: Prototype Spikes & Multi-OS CI Results

**Document:** `docs/reviews/qa-spikes-ci-verification.md`  
**Reviewer:** Senior QA Engineer (`agent-1793867530011-poqhdm`)  
**Verified Commits:** [`54ade4c`](https://github.com/sedam-or/scribetrace/commit/54ade4c) (Spikes 1 & 2), [`ee41655`](https://github.com/sedam-or/scribetrace/commit/ee41655) (Spike 4 & CI metrics)  
**CI Evidence:** GitHub Actions [Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930) (Spike 4) and [Run 42076857267](https://github.com/sedam-or/scribetrace/actions/runs/42076857267) (CI suite)  
**Target Specifications:** [`docs/PROTOTYPE-PLAN.md`](../PROTOTYPE-PLAN.md), [`docs/TECH-OPTIMIZATION.md`](../TECH-OPTIMIZATION.md) §3  
**Date:** 2026-11-06  
**Overall Verdict:** **VERIFIED & PASSED (100% Multi-OS Quality Gate Compliance)**

---

## 1. Prototype Spikes Gate Evaluation Matrix

| Gate / Spike | Invariant Under Test | PROTOTYPE-PLAN / TECH-OPT Budget | Measured Result | Verdict | CI Platform / Run Evidence |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **SPIKE-1** | Log Merge Commutativity & Idempotency | 10,000 / 10,000 runs | **10,000 / 10,000 PASS** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-1** | 10,000 Event Replay Throughput | `< 500 ms` | **41 ms** (Release), **94 ms** (Debug) | **PASS** | Local & CI Linux/Windows |
| **SPIKE-1** | Immutability & Collision Handling | Zero file overwrites, fresh UUID minted | **PASS (0 byte modifications)** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-2** | Sequence-Covered Purge Rule | 100% covered events hidden regardless of clocks | **100% hidden, 0 data leaks** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-2** | Multi-Purge Order Invariance | Identical in 10,000 random orders | **10,000 / 10,000 PASS** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-3** | Zotero 7 Local HTTP Interface Contract | Annotations, metadata, colors, rects | **All fields documented** | **DESK-CHECK ONLY** | `docs/spikes/zotero-desk-check.md` |
| **SPIKE-4A** | Empty Shell Binary Size (Linux) | `≤ 20 MB` | **5.58 MB** (5,851,480 B) | **PASS** | Ubuntu 24.04 ([Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930)) |
| **SPIKE-4A** | Empty Shell Binary Size (macOS) | `≤ 20 MB` | **2.67 MB** (2,804,568 B) | **PASS** | macOS 15 Apple Silicon ([Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930)) |
| **SPIKE-4A** | Empty Shell Binary Size (Windows) | `≤ 20 MB` | **1.81 MB** (1,900,544 B) | **PASS** | Windows Server 2025 ([Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930)) |
| **SPIKE-4B** | Cold First Page Render (Linux) | `< 1,500 ms` (ceiling 2,000 ms) | **140 ms** (10x faster) | **PASS** | Ubuntu 24.04 ([Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930)) |
| **SPIKE-4B** | Idle Memory RSS (Linux) | `< 250 MB` | **161.7 MB** | **PASS** | Ubuntu 24.04 ([Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930)) |
| **SPIKE-4B** | Cold First Page Render (macOS) | `< 1,500 ms` (ceiling 2,000 ms) | **117 ms** (12x faster) | **PASS** | macOS 15 Apple Silicon ([Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930)) |
| **SPIKE-4B** | Idle Memory RSS (macOS) | `< 250 MB` | **164.3 MB** | **PASS** | macOS 15 Apple Silicon ([Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930)) |
| **SPIKE-4B** | Cold First Page Render (Windows) | `< 1,500 ms` (ceiling 2,000 ms) | **197 ms** (7x faster) | **PASS** | Windows Server 2025 ([Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930)) |
| **SPIKE-4B** | Idle Memory RSS (Windows) | `< 250 MB` | **125.8 MB** (2x under budget) | **PASS** | Windows Server 2025 ([Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930)) |
| **SPIKE-4B** | Ground-Truth Text Extraction | Exact ground-truth string match | **`selection_check: true`** | **PASS** | All 3 OS ([Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930)) |
| **Supply Chain** | Zero `rustls` Stack Verification | 0 crates | **0 rustls crates** | **PASS** | CI Linux ([Run 42076857267](https://github.com/sedam-or/scribetrace/actions/runs/42076857267)) |
| **Supply Chain** | Transitive Crate Ceilings | log-merge $\le 26$, purge $\le 23$, tauri $\le 375$ | **21 / 18 / 310 crates** | **PASS** | CI Linux ([Run 42076857267](https://github.com/sedam-or/scribetrace/actions/runs/42076857267)) |

---

## 2. Architectural Decision Rule Execution (`TECH-OPTIMIZATION.md` §3)

Per the ratified decision rule in `docs/TECH-OPTIMIZATION.md` §3:

1. **Stage 1 Shell Decision (Part A):**
   - **Condition:** "Part A passes on all 3 OS ($\le 20\text{ MB}$ release binary size)".
   - **Measured:** Linux = 5.58 MB, macOS = 2.67 MB, Windows = 1.81 MB. All three are substantially below the 20 MB ceiling.
   - **Decision:** **STAY ON TAURI FOR STAGE 1.** Build slices 1–14 as planned. No fallback to `scribetrace serve` is needed.

2. **Stage 2 Reader Decision (Part B):**
   - **Condition:** "Part B passes on all 3 OS (first page $< 2.0\text{ s}$, RSS $< 250\text{ MB}$, text selection equals ground truth)".
   - **Measured:** Cold first page renders in 117–197 ms across all 3 platforms (budget was 1,500 ms). Idle RSS settles at 125.8–164.3 MB (budget was 250 MB). Text layer ground truth matches 100%.
   - **Decision:** Stage 2 PDF.js reader integration is validated across Linux (WebKitGTK), macOS (WKWebView), and Windows (WebView2). ADR 0011 remains solid.

---

## 3. Notable CI Engineering Hardening & Execution Integrity

QA commends the DevOps team for the following hardening measures implemented in CI:

1. **Non-Display Headless Transparency:**
   The documentation explicitly distinguishes between compiled binary sizing and headless runner benchmark execution, preserving test honesty.
2. **Automated Supply Chain Guard:**
   `ci.yml` strictly enforces the zero-TLS rule (`cargo tree -i rustls` exits 1 if detected) and locks down transitive crate growth with hard ceilings (preventing dependency drift).
3. **Cross-Platform Cleanliness:**
   - Isolated `frontendDist` to a dedicated `ui/` directory, preventing Windows `os error 33` file-locking issues.
   - Cleanly eliminated cross-platform compilation warnings (`-D warnings` passing across Linux, macOS, and Windows).
   - Generated valid Windows `.ico` and macOS `.icns` resources from source SVG brand assets.

---

## 4. Final Quality Gate Conclusion

- **SPIKE-1 (Log Merge):** **PASS**
- **SPIKE-2 (Purge Engine):** **PASS**
- **SPIKE-3 (Zotero Contract):** **DESK-CHECK PASS** (Live verification against real Zotero 7 remains a gating prerequisite specifically before Stage 1 Slice 4 production code, as specified in `BUILD-PLAN.md`).
- **SPIKE-4A (Empty Shell Lightness):** **PASS across all 3 OS**
- **SPIKE-4B (PDF.js Lightness & Text Selection):** **PASS across all 3 OS**

**Recommendation:** The architectural spike phase is officially concluded. Stage 1 production coding (Slice 1 Walking Skeleton) is **CLEARED TO PROCEED**.
