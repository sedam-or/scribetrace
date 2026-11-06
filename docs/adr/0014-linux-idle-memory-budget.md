# ADR 0014: Linux idle-memory budget and WebKitGTK tuning

**Status:** Accepted, amended 2026-11-06 (Linux budget raised to 200 MB by CEO decision; see [Amendment](#amendment-2026-11-06-linux-budget-200-mb) and [DECISION-LOG](../DECISION-LOG.md) row y) · **Date:** 2026-11-06 · **Amends:** the idle line of the [lightness budget](../ARCHITECTURE.md); applies the rule in [TECH-OPTIMIZATION §3](../TECH-OPTIMIZATION.md)

## Context
SPIKE-4A measured the empty Stage 1 shell (devops, [RESULTS](../spikes/RESULTS.md), method = whole process tree, 10 s settle, 5 s sampling, median of 3). Budget: idle ≤120 MB; the rule allows "stay" up to 180 MB (+50%) after one tuning pass.

| Linux desktop, PSS | Idle | Cold start |
|---|---|---|
| Untuned | 195.5 MB (fails the 180 ceiling) | 489 ms |
| `WEBKIT_DISABLE_DMABUF_RENDERER=1` | **156.9 MB** | 91 ms |
| `WEBKIT_DISABLE_COMPOSITING_MODE=1` | 171.7 MB | 166 ms |
| both | 191.3 MB (fails) | 94 ms |

Other OS: Windows 136 MB (WorkingSet, tree); macOS idle UNMEASURED (runner timed out). Linux CI under xvfb: 57.8 MB, which is a headless buffer and is not comparable to a desktop. Sizes pass everywhere (5.6 / 1.8 / 2.7 MB binaries).

## Options
- **A. Stay on Tauri, ship the DMABUF tuning, and set per-OS idle budgets.**
- **B. Fall back to `scribetrace serve` on Linux.** Adds a loopback listener (new SEC items, DNS-rebinding tests, an ADR) and loses native dialogs; the memory it saves is a browser tab that the user already runs.
- **C. Keep 120 MB for everyone and call Linux a fail.** Forces B for a gain nobody can feel.

## Decision
**A.** Per-OS idle budget for the empty shell: **Linux ≤ 200 MB (amended from 160 MB, see Amendment), Windows ≤ 140 MB, macOS UNMEASURED until a run completes (provisional ≤ 140 MB)**. The 120 MB figure stays as the *target*, not the gate. Tuning ships in the Linux launcher, not in code: the packaged `.desktop` entry runs `env WEBKIT_DISABLE_DMABUF_RENDERER=1 scribetrace` (devops: Tauri deb/AppImage desktop template; documented in DEVELOPMENT for `cargo run`). It is not set from Rust, because `set_var` is `unsafe` and the workspace forbids `unsafe_code`; a self re-exec is the fallback if a package format has no launcher.

Why this is honest: the 120 MB number was a guess made before any measurement; the measured tuned figure is +30%, inside the +50% rule. Comparison with what the target users already run (Zotero and a browser, both several hundred MB) is context, not the justification; the justification is the rule written before the measurement.

## Consequences
- ARCHITECTURE lightness table and `docs/DEFINITION-OF-DONE.md` take the per-OS numbers; slice 14 gates on them, not on 120 MB.
- Risk: DMABUF off can cost rendering speed on some GPUs. The empty shell shows no cold-start loss (91 ms); re-check with the 2,000-line history view (slice 7) and revisit if frame time suffers.
- Revisit when: macOS is measured, a Linux run with the real UI exceeds 200 MB, or WebKitGTK changes its renderer defaults. Then the fallback (B) is evaluated with the same rule.
- Stage 2 (PDF) is judged separately by SPIKE-4B (Linux 161.7 MB idle with a 500-page PDF, budget 250 MB: passes).
- Phase-1 status: the Linux E2E memory gate in `e2e/test_walking_skeleton.mjs` still reads 160 MB until qa changes it (qa owns the harness). The current main build measures 190.62 MB (CI 42180952834), so that gate is red until it is updated; this ADR does not claim it passes.

## Amendment 2026-11-06: Linux budget 200 MB
**Decided by the CEO, relayed by chief-of-staff; recorded here by architect.**

- **Old budget:** Linux ≤ 160 MB. **New budget:** Linux ≤ 200 MB, idle empty shell, method unchanged (whole process tree, 10 s settle).
- **Measured basis:** 190.62 MB with `WEBKIT_DISABLE_DMABUF_RENDERER=1` (CI 42180952834); 197.55 MB in an earlier run. The tuned build measures above 160 MB on every run seen so far.
- **Why 200 MB:** it sits just above the worst measured run (about 1.3% over 197.55 MB, about 5% over 190.62 MB), so ordinary run-to-run variance does not flake the gate. It is a round number, not a tuned target.
- **Rule exceeded, stated plainly:** 200 MB is +67% over the 120 MB target, above the +50% ceiling (180 MB) in [TECH-OPTIMIZATION §3](../TECH-OPTIMIZATION.md). The CEO decision overrides that rule for Linux. This amendment records that override; it does not argue the rule is met.
- **Unchanged:** Windows ≤ 140 MB; macOS provisional ≤ 140 MB; the 120 MB target; the AppImage 120 MB size ceiling (`APPIMAGE_BUDGET_BYTES` in `release.yml`); DMABUF tuning in the launcher, not in code.
- **Not changed here:** the E2E gate (qa), and the reduction work. Reducing the Linux footprint is a follow-up. Tighten the budget when a reduction brings the empty shell under 160 MB.
