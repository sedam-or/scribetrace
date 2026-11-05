# QA Verification Report: Prototype Spikes & Multi-OS CI Results (Revision 2)

**Document:** `docs/reviews/qa-spikes-ci-verification.md`  
**Reviewer:** Senior QA Engineer (`agent-1793867530011-poqhdm`)  
**Verified Commits:** [`54ade4c`](https://github.com/sedam-or/scribetrace/commit/54ade4c) (Spikes 1 & 2), [`49eb790`](https://github.com/sedam-or/scribetrace/commit/49eb790) (Spike 4 & corrected lightness metrics)  
**CI Evidence:** GitHub Actions [Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930), [Run 42078477255](https://github.com/sedam-or/scribetrace/actions/runs/42078477255), and [Run 42080470515](https://github.com/sedam-or/scribetrace/actions/runs/42080470515)  
**Target Specifications:** [`docs/PROTOTYPE-PLAN.md`](../PROTOTYPE-PLAN.md), [`docs/TECH-OPTIMIZATION.md`](../TECH-OPTIMIZATION.md) §3  
**Date:** 2026-11-06  
**Overall Verdict:** **CONDITIONAL GATE PASS (Strict Audit against Decision Rules)**

---

## 1. Prototype Spikes Gate Evaluation Matrix

| Gate / Spike | Invariant Under Test | PROTOTYPE-PLAN / TECH-OPT Rule | Measured Result | Rule-Table Verdict | CI Platform / Run Evidence |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **SPIKE-1** | Log Merge Commutativity & Idempotency | 10,000 / 10,000 runs | **10,000 / 10,000 PASS** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-1** | 10,000 Event Replay Throughput | `< 500 ms` | **41 ms** (Release), **94 ms** (Debug) | **PASS** | Local & CI Linux/Windows |
| **SPIKE-1** | Immutability & Collision Handling | Zero file overwrites, fresh UUID minted | **PASS (0 byte modifications)** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-2** | Sequence-Covered Purge Rule | 100% covered events hidden regardless of clocks | **100% hidden, 0 data leaks** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-2** | Multi-Purge Order Invariance | Identical in 10,000 random orders | **10,000 / 10,000 PASS** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-3** | Zotero 7 Local HTTP Interface Contract | Annotations, metadata, colors, rects | **All fields documented** | **DESK-CHECK ONLY** (Gate before Slice 4) | `docs/spikes/zotero-desk-check.md` |
| **SPIKE-4A** | Binary Size (Linux Ubuntu) | `≤ 20 MB` | **5.58 MB** (stripped), **2.47 MB** (.deb) | **PASS** | [Run 42080470515](https://github.com/sedam-or/scribetrace/actions/runs/42080470515) |
| **SPIKE-4A** | Binary Size (macOS 15 ARM64) | `≤ 20 MB` | **2.67 MB** (2,804,568 B) | **PASS** | [Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930) |
| **SPIKE-4A** | Binary Size (Windows 2025) | `≤ 20 MB` | **1.81 MB** (stripped), **1.63 MB** (.exe) | **PASS** | [Run 42080470515](https://github.com/sedam-or/scribetrace/actions/runs/42080470515) |
| **SPIKE-4A** | Cold Start (Linux) | `≤ 2,000 ms` | **122 ms** (live window) | **PASS** | Local display session |
| **SPIKE-4A** | Cold Start (Windows) | `≤ 2,000 ms` | **510 ms** | **PASS** | [Run 42080470515](https://github.com/sedam-or/scribetrace/actions/runs/42080470515) |
| **SPIKE-4A** | **Idle Memory (Windows WorkingSet)** | `≤ 120 MB` (Target) / `≤ 180 MB` (Tier 2 Stay) | **136.08 MB** (median over 5s) | **TIER 2 STAY (PASS)** | [Run 42080470515](https://github.com/sedam-or/scribetrace/actions/runs/42080470515) |
| **SPIKE-4A** | **Idle Memory (Linux PSS)** | `≤ 120 MB` (Target) / `≤ 180 MB` (Tier 2 Stay) | **57.83 MB** (Headless CI runner) vs **211.41 MB** (Desktop Display) | **PROVISIONAL / PENDING RE-MEASUREMENT** | [Run 42080470515](https://github.com/sedam-or/scribetrace/actions/runs/42080470515) |
| **SPIKE-4B** | Cold First Page Render (3 OS) | `< 1,500 ms` (ceiling 2,000 ms) | Linux 140 ms, macOS 117 ms, Windows 197 ms | **NODE BASELINE ONLY** (Not live webview) | Node.js harness on 500p PDF |
| **SPIKE-4B** | Idle Memory RSS (3 OS) | `< 250 MB` | Linux 161.7 MB, macOS 164.3 MB, Win 125.8 MB | **NODE BASELINE ONLY** (Not live webview) | Node.js harness on 500p PDF |
| **SPIKE-4B** | Ground-Truth Text Selection | Exact ground-truth string match | **`selection_check: true`** | **PASS (Node Parser)** | Node.js harness on 500p PDF |
| **Supply Chain** | Zero `rustls` Stack Verification | 0 crates | **0 rustls crates** | **PASS** | CI Linux ([Run 42078477255](https://github.com/sedam-or/scribetrace/actions/runs/42078477255)) |
| **Supply Chain** | Transitive Crate Ceilings | log-merge $\le 26$, purge $\le 23$, tauri $\le 375$ | **21 / 18 / 310 crates** | **PASS** | CI Linux ([Run 42078477255](https://github.com/sedam-or/scribetrace/actions/runs/42078477255)) |

---

## 2. Rigorous Scrutiny against `TECH-OPTIMIZATION.md` §3 Decision Rules

Applying the decision matrix verbatim:

### 2.1 The Linux Desktop Memory Divergence (211.41 MB vs 120 MB Budget)
- **Rule:**
  - "A passes on all 3 OS ($\le 120\text{ MB}$) $\implies$ Stay on Tauri for Stage 1."
  - "A fails on one OS by $\le 50\%$ ($\le 180\text{ MB}$) $\implies$ Stay, apply cut 3 and 4 again, re-measure once."
  - "A fails on one OS by $> 50\%$ ($> 180\text{ MB}$) $\implies$ Fallback on that OS only: `scribetrace serve` (CLI, binds `127.0.0.1` on random port with per-run token, opens system browser)."
- **Measured Reality:**
  - In a purely headless virtual buffer (Xvfb/CI runner), WebKitGTK process tree consumes **57.83 MB PSS** (**PASS**).
  - In a live desktop session with full OpenGL/Mesa GPU acceleration buffers allocated, WebKitGTK helper processes (`WebKitWebProcess` + `WebKitNetworkProcess`) inflate tree memory to **211.41 MB PSS**.
  - 211.41 MB is **76.2% over the 120 MB target** ($> 50\%$).
- **QA Finding & Ruling:**
  - If 211.41 MB represents the true unoptimized desktop baseline, the decision rule in §3 strictly mandates falling back to `scribetrace serve` on Linux.
  - However, DevOps is actively re-measuring using software rendering flags (`WEBKIT_DISABLE_DMABUF_RENDERER=1` / disabling compositor GPU overhead) and the standardized 10 s settle + 5 s median sampling window.
  - **Verdict:** Linux desktop idle memory is **PROVISIONAL**. Tauri Stage 1 stays unblocked for initial scaffolding (Slices 1–3), but the Linux shell decision will be re-evaluated as soon as DevOps resubmits the re-measured desktop numbers.

### 2.2 SPIKE-4B Clarification: Node.js Baseline vs Native Webview
- **QA Finding:**
  - The previous summary conflated the Node.js PDF.js worker parsing benchmark with native WebKitGTK / WebView2 webview rendering.
  - In headless CI, SPIKE-4B measures the JavaScript runtime parsing capability under Node.js (117–197 ms parse latency, 125.8–164.3 MB RSS). It **does not prove** that WebKitGTK or WebView2 will render a 500-page PDF within 250 MB in a graphical session.
  - **Verdict:** Correctly relabeled in `RESULTS.md` as **"Node baseline, NOT webview"**. Real webview rendering with PDF.js is deferred to Stage 2 graphical verification per ADR 0011.

---

## 3. Verified Supply Chain & Build Health

1. **Zero TLS Invariant:** Verified 0 `rustls` dependencies in CI.
2. **Binary & Bundle Footprint:**
   - Linux: 5.58 MB binary, 2.47 MB `.deb`
   - macOS: 2.67 MB binary
   - Windows: 1.81 MB binary, 1.63 MB `.exe`
   All installer packages easily beat the $\le 20\text{ MB}$ budget ceiling by a factor of 8x–12x.
3. **Deterministic Replay & Purge:** Spikes 1 and 2 remain rock-solid (10,000 property test iterations, 41 ms replay, immutable logs).

---

## 4. Conclusion & Next Steps

1. **Stage 1 Scaffolding:** Greenlit to proceed.
2. **Pending DevOps Re-measurement:** Awaiting DevOps' re-measured Linux desktop idle memory under software rendering / standardized 10 s settling protocol to finalize whether Linux uses Tauri v2 window or the `scribetrace serve` browser adapter.
3. **Linux E2E Test Harness:** QA is proceeding to build the real-window `tauri-driver` + Xvfb test harness in `e2e/`.
