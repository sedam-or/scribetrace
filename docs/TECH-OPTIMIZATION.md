# ScribeTrace: Technical optimization review

**Date:** 2026-11-06 · **Author:** architect · **Inputs:** [ARCHITECTURE §Lightness budget](ARCHITECTURE.md), [RESULTS](spikes/RESULTS.md), [ADR 0006](adr/0006-staged-mvp-shell-and-stack.md), `spikes/`
**Rule used here:** a number is *measured* only if a command in this file produced it on 2026-11-06 on the dev machine (Linux, headless). Everything else says **UNMEASURED**.

## 1. Measured lightness

| Item | Direct | Transitive (normal deps, unique) | Budget | Verdict |
|---|---|---|---|---|
| `spikes/log-merge` (serde, serde_json, uuid, sha2 + thiserror in lib) | 4 | 17 (39 with dev-deps proptest etc.) | ≤15 direct | within |
| `spikes/purge` | 3 | 15 (40 with dev-deps) | ≤15 direct | within |
| `spikes/tauri-webview` (tauri, serde, serde_json; Linux) | 3 | 217 | ≤15 direct | direct within; transitive is Tauri's weight |
| PDF.js payload (`pdfjs-dist` 4.0.379 worker 305,665 B + main 1,029,136 B) | 1 JS pkg | n/a | ≤6 JS pkgs | **1.33 MB**, Stage 2 only |
| Projected Stage 1 core+CLI+shell (probe, no code): serde, serde_json, uuid(v4), sha2, thiserror, clap, ureq, tauri, tauri-build | 9 (+2 dev: proptest, wiremock) | 84 for core+CLI deps with `ureq` default-features **off**; 127 with them on | ≤15 direct | within; Tauri adds ~217 on top |
| Projected Stage 1 JS: react, react-dom, @tauri-apps/api, markdown-it, vite, typescript | 6 | UNMEASURED | ≤6 direct | **at the limit**; a test runner (vitest) would be the 7th and needs an ADR. Use `node --test` |

Commands: `cargo tree -e normal --prefix none | sort -u | wc -l` in each spike; the probe crate used `ureq` with and without `default-features=false` (127 − 84 = **43 crates saved**, mostly TLS).

**UNMEASURED, do not quote:** installer size (any OS), Linux release binary size (the release build was cut off in this session), idle RSS of the webview, cold start, first-page render. All of these wait for SPIKE-4 in CI (webview needs a display; the dev machine has none). Node numbers in RESULTS (158 ms, 166 MB) are a parse baseline, not a webview.

**Measured and good:** replay of 10,000 events across 5 device files = **41 ms** release (budget 500 ms; SPIKE-1).

**Gap in the budget:** it counts *direct* crates only, and Tauri alone brings ~217. Add a CI line that records the transitive count and fails on +20% growth (see cut 2).

## 2. Five cuts that keep it small and fast

| # | Cut / choice | Effect | Cost | Test that proves it |
|---|---|---|---|---|
| 1 | **No database.** Per-device JSONL, union in memory ([ADR 0007](adr/0007-per-device-history-log-merge-and-forget.md)); no SQLite, no index file. | No C library, no schema migrations, no sync conflicts; replay 41 ms for 10k events (measured). | Full replay on every paper open; no ad-hoc queries. Snapshots only if replay > 1 s. | Keep the SPIKE-1 replay test as a CI gate (≤500 ms, 10k events); add a 50,000-event run and record it. |
| 2 | **Trim crate features.** `ureq` with `default-features=false` (plain HTTP to loopback only, no TLS), `uuid` only `v4`, `clap` without colour/suggestions, `serde_json` default. | −43 crates measured in the probe; smaller binary and audit surface; no TLS stack in an app that never leaves 127.0.0.1. | If a later feature needs HTTPS, it needs an ADR (which the network rule requires anyway). | CI step: `cargo tree` unique count ≤ recorded baseline +20%; `cargo tree -i rustls` must find nothing. |
| 3 | **Smallest release profile, system webview.** `opt-level="s"`, `lto=true`, `codegen-units=1`, `panic="abort"`, `strip=true`; Tauri uses the OS webview, bundles no browser. | Binary size drops; installer target ≤20 MB is reachable only this way. Size effect is UNMEASURED. | Slower release builds; `panic="abort"` loses unwinding (fine: a crash never touches the append-only files, R-10). | CI builds installers on 3 OS and fails above 20 MB (Stage 1). Record actual sizes in RESULTS. |
| 4 | **Keep PDF.js and every PDF dependency out of Stage 1.** Stage 1 never opens a PDF. Markdown is parsed to a tree and rendered as React elements: no HTML sanitizer package. | −1.33 MB JS, 1 JS package and the whole text-layer attack surface stay out of the first release; JS count holds at 6. | Stage 2 pays for PDF.js later (pinned, [ADR 0011](adr/0011-stage-2-reader.md)). | CI: `dist/` size ≤ 400 KB (a target, UNMEASURED); a test that fails if `pdfjs-dist` appears in the Stage 1 lockfile. |
| 5 | **No watcher, no polling, no autosave loop; read only the open paper.** Everything runs on a click; the paper list reads file *names*, not contents. | 0 background work, idle CPU ≈ 0, list loads fast however much history exists; matches R-02. | "Refresh" is manual; an outside edit is noticed on the next open or save, not live. | Portable proof: `cargo tree -i notify` is empty and the UI has no timers (lint). Linux-only extra: window open 60 s → inotify count in `/proc/<pid>/fdinfo` is 0 and no sockets (skipped on macOS/Windows). List test: fixture of 500 real folders × 2,000-line files; the files hold invalid JSON on purpose, so the list can only pass if it never parses contents; ≤2 s (R-23). |

## 3. Tauri versus fallback: decision rule

SPIKE-4 measures two different things; do not mix them.
- **A. Empty Stage 1 shell** (window, React UI, one IPC call; no PDF): decides Stage 1.
- **B. PDF.js in the webview** (the existing spike: 50 MB / 500-page PDF, 10 page turns, text selection): decides only Stage 2 (R-30..).

Targets come from the [lightness budget](ARCHITECTURE.md): idle ≤120 MB (whole process tree, including the webview helper), cold start ≤2 s, installer ≤20 MB; B: first page ≤1.5 s (ceiling 2.0 s), RSS ≤250 MB, selected text equals ground truth.

| Measured on each of Linux (WebKitGTK), Windows (WebView2), macOS (WKWebView) | Decision |
|---|---|
| A passes on all 3 OS | **Stay on Tauri for Stage 1.** Build slices as planned. |
| A passes on all 3, B fails on one OS (first page > 2.0 s, RSS > 250 MB or text mismatch) | **Stay for Stage 1.** Stage 2 reader is not promised on that OS; ADR 0011 is re-opened for that OS only. |
| A fails on one OS by ≤50% (idle ≤180 MB, start ≤3 s, installer ≤30 MB) | **Stay**, apply cut 3 and 4 again, re-measure once. Still failing → treat as "fails by more". |
| A fails on one OS by >50%, or the webview is missing/broken there (e.g. no WebKitGTK 4.1 on a supported distro, WebView2 install blocked) | **Fallback on that OS only:** `scribetrace serve` (CLI, binds `127.0.0.1` on a random port with a per-run token, opens the system browser; same React UI). |
| A fails by >50% on two or more OS | **Switch for everyone** to CLI + local web; Tauri is dropped. |
| Electron or any bundled browser | **Rejected:** it cannot meet the 20 MB budget ([ADR 0006](adr/0006-staged-mvp-shell-and-stack.md)). |

**Make the fallback cheap now:** the UI calls the core only through one typed `Rpc` interface (slice 1). Tauri `invoke` is one adapter; an HTTP adapter would be the second. Without that seam the fallback is a rewrite. The fallback adds a loopback *listener*, which is a new SEC item (Host-header check, token, no CORS, DNS-rebinding test) and an ADR; it is a real cost, not a free escape.

**Rust maintenance, honestly.** The maintainer works in Laravel/Vue/TS/Python, so they will not be able to review Rust at borrow-checker depth. What keeps this safe: (1) the Rust surface is small, about 9 crates and a core with no UI; (2) the *tests are the review*: property tests for merge and purge (SPIKE-1/2 pattern), table tests per format rule, `clippy -D warnings`, `cargo deny`/`audit` in CI; (3) the shell is one thin command file, and the UI (React/TS) is where the maintainer's skills apply; (4) AI agents write most Rust, so bus factor on Rust is real but tolerable. Why not move the core to TS: the UI layer is forbidden raw filesystem access ([ARCHITECTURE](ARCHITECTURE.md) layering), so a TS core would need a Node sidecar, which breaks size and the 0-background-process rule. **Trigger to revisit:** after slices 1–6, if more than half of the agent-built Rust slices need more than two review rounds, or one Rust bug blocks work for over a week, hold a go/no-go on a Python or Node core behind the same `Rpc` seam (accepting the fallback's costs above).

**How A and B are measured (same on every OS).** Wait 10 s after launch, then sample the **whole process tree** (app plus webview helpers such as WebKitWebProcess or msedgewebview2) once a second for 5 s and report the median. Linux: sum `Pss` from `/proc/<pid>/smaps_rollup` over the tree; macOS: sum `ps -o rss` over the tree; Windows: sum `WorkingSetSize` of the tree (`Get-CimInstance Win32_Process`). Shared pages are counted differently per OS, so compare each OS only against its own result and report the method next to the number. A single-PID reading is invalid.

## 4. What I do not know yet
Everything under A and B above except the Node baseline. SPIKE-4 in CI is the single event that turns this document from rules into decisions; until then Stay is the default and no slice in [BUILD-PLAN](BUILD-PLAN.md) depends on the PDF numbers (only the empty-shell numbers, A, gate Stage 1).
