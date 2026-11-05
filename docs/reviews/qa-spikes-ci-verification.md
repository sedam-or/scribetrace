# QA Verification Report: Prototype Spikes & Multi-OS CI Results (Revision 3)

**Document:** `docs/reviews/qa-spikes-ci-verification.md`  
**Reviewer:** Senior QA Engineer (`agent-1793867530011-poqhdm`)  
**Verified Commits:** [`54ade4c`](https://github.com/sedam-or/scribetrace/commit/54ade4c) (Spikes 1 & 2), [`770ff7c`](https://github.com/sedam-or/scribetrace/commit/770ff7c) (Spike 4 & corrected lightness metrics), [`48fb643`](https://github.com/sedam-or/scribetrace/commit/48fb643) (Live Zotero 7 check & desktop template)  
**CI Evidence:** GitHub Actions [Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930), [Run 42078477255](https://github.com/sedam-or/scribetrace/actions/runs/42078477255), and [Run 42080470515](https://github.com/sedam-or/scribetrace/actions/runs/42080470515)  
**Target Specifications:** [`docs/PROTOTYPE-PLAN.md`](../PROTOTYPE-PLAN.md), [`docs/TECH-OPTIMIZATION.md`](../TECH-OPTIMIZATION.md) §3, [`docs/adr/0014-per-os-idle-memory-budget.md`](../adr/0014-per-os-idle-memory-budget.md)  
**Date:** 2026-11-06  
**Overall Verdict:** **VERIFIED & APPROVED (Aligned with ADR 0014 & Decision Rules)**

---

## 1. Prototype Spikes Gate Evaluation Matrix

| Gate / Spike | Invariant Under Test | PROTOTYPE-PLAN / ADR 0014 Budget | Measured Result | Rule-Table Verdict | Platform / Evidence |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **SPIKE-1** | Log Merge Commutativity & Idempotency | 10,000 / 10,000 runs | **10,000 / 10,000 PASS** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-1** | 10,000 Event Replay Throughput | `< 500 ms` | **41 ms** (Release), **94 ms** (Debug) | **PASS** | Local & CI Linux/Windows |
| **SPIKE-1** | Immutability & Collision Handling | Zero file overwrites, fresh UUID minted | **PASS (0 byte modifications)** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-2** | Sequence-Covered Purge Rule | 100% covered events hidden regardless of clocks | **100% hidden, 0 data leaks** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-2** | Multi-Purge Order Invariance | Identical in 10,000 random orders | **10,000 / 10,000 PASS** | **PASS** | Local & CI Linux/Windows |
| **SPIKE-3** | Zotero 7 Local HTTP Interface Contract | Live query against Zotero 7 on loopback | **Contract verified live; exact fixtures recorded** | **PASS** | [`docs/spikes/zotero-live-check.md`](../spikes/zotero-live-check.md) |
| **SPIKE-4A** | Binary & Bundle Size (Linux) | `≤ 20 MB` | **5.58 MB** (stripped), **2.47 MB** (.deb) | **PASS** | [Run 42080470515](https://github.com/sedam-or/scribetrace/actions/runs/42080470515) |
| **SPIKE-4A** | Binary & Bundle Size (macOS) | `≤ 20 MB` | **2.67 MB** (2,804,568 B) | **PASS** | [Run 42076856930](https://github.com/sedam-or/scribetrace/actions/runs/42076856930) |
| **SPIKE-4A** | Binary & Bundle Size (Windows) | `≤ 20 MB` | **1.81 MB** (stripped), **1.63 MB** (.exe) | **PASS** | [Run 42080470515](https://github.com/sedam-or/scribetrace/actions/runs/42080470515) |
| **SPIKE-4A** | Cold Start (Linux) | `≤ 2,000 ms` | **91–166 ms** (tuned desktop) | **PASS** | Local X11 / Wayland session |
| **SPIKE-4A** | Cold Start (Windows) | `≤ 2,000 ms` | **510 ms** | **PASS** | [Run 42080470515](https://github.com/sedam-or/scribetrace/actions/runs/42080470515) |
| **SPIKE-4A** | **Idle Memory (Linux PSS)** | Target $\le 120$ MB, Tier 2 $\le 160$ MB (ADR 0014) | **57.83 MB** (Headless CI) / **156.85 MB** (Tuned Desktop) | **TIER 2 STAY (PASS)** | Tuned via `WEBKIT_DISABLE_DMABUF_RENDERER=1` |
| **SPIKE-4A** | **Idle Memory (Windows WorkingSet)** | Target $\le 120$ MB, Tier 2 $\le 140$ MB (ADR 0014) | **136.08 MB** (median over 5s) | **TIER 2 STAY (PASS)** | [Run 42080470515](https://github.com/sedam-or/scribetrace/actions/runs/42080470515) |
| **SPIKE-4A** | **Idle Memory (macOS PSS/RSS)** | Target $\le 120$ MB, Tier 2 $\le 140$ MB (ADR 0014) | **UNMEASURED** (queue timeout) | **HONEST UNMEASURED** | Informative for Stage 1 |
| **SPIKE-4B** | Cold First Page Render (3 OS) | `< 1,500 ms` (ceiling 2,000 ms) | Linux 140 ms, macOS 117 ms, Windows 197 ms | **NODE BASELINE ONLY** (Not live webview) | Node.js harness on 500p PDF |
| **SPIKE-4B** | Idle Memory RSS (3 OS) | `< 250 MB` | Linux 161.7 MB, macOS 164.3 MB, Win 125.8 MB | **NODE BASELINE ONLY** (Not live webview) | Node.js harness on 500p PDF |
| **SPIKE-4B** | Ground-Truth Text Selection | Exact ground-truth string match | **`selection_check: true`** | **PASS (Node Parser)** | Node.js harness on 500p PDF |
| **Supply Chain** | Zero `rustls` Stack Verification | 0 crates | **0 rustls crates** | **PASS** | CI Linux ([Run 42078477255](https://github.com/sedam-or/scribetrace/actions/runs/42078477255)) |
| **Supply Chain** | Transitive Crate Ceilings | log-merge $\le 26$, purge $\le 23$, tauri $\le 375$ | **21 / 18 / 310 crates** | **PASS** | CI Linux ([Run 42078477255](https://github.com/sedam-or/scribetrace/actions/runs/42078477255)) |

---

## 2. Rigorous Scrutiny against `TECH-OPTIMIZATION.md` §3 & ADR 0014

### 2.1 The Linux Desktop Memory Resolution (ADR 0014)
- **Background:** The raw untuned WebKitGTK desktop memory was measured at 195.5–211.41 MB PSS ($> 50\%$ over the initial 120 MB target), which triggered scrutiny under TECH-OPTIMIZATION §3.
- **Tuned Mitigation:** DevOps and Architect identified that Mesa GPU buffer allocation (`DMABUF`) contributed ~54.5 MB of unnecessary compositor overhead for an empty window.
- **Ratified Rule (ADR 0014):**
  - Sets per-OS idle memory ceilings: Linux Tier 2 $\le 160$ MB; Windows Tier 2 $\le 140$ MB; macOS Tier 2 $\le 140$ MB.
  - Ships desktop launcher template with `WEBKIT_DISABLE_DMABUF_RENDERER=1` in `src-tauri/desktop.template`.
  - Re-measurement confirmed: **156.85 MB PSS** under the standardized 10 s settle + 5 s median sampling window.
  - **Verdict:** Conforms to **Tier 2 Stay** ($\le 160$ MB). Stage 1 on Linux remains on Tauri v2 without fallback.

### 2.2 SPIKE-4B Honest Clarification: Node.js Baseline vs Native Webview
- **Verdict:** Correctly relabeled in `RESULTS.md` Section 4 as **"Node baseline, NOT webview"**.
- In headless CI, SPIKE-4B measures the PDF.js parsing engine running under Node.js (117–197 ms parse latency, 125.8–164.3 MB RSS). Native WebKitGTK / WebView2 webview rendering of PDFs is deferred to Stage 2 graphical verification per ADR 0011.

---

## 3. Conclusion & Gate Sign-Off

The updated `docs/spikes/RESULTS.md` (Revision 3) accurately reflects the empirical measurements, adheres strictly to the decision table in `TECH-OPTIMIZATION.md` §3 and ADR 0014, and honestly reports unmeasured dimensions (macOS idle memory) without conflating Node.js benchmarks with live webviews.

**Verdict:** **APPROVED & VERIFIED.** All gates for SPIKE-1, SPIKE-2, SPIKE-3, and SPIKE-4A (Linux/Windows) are fully satisfied for Stage 1 development.
