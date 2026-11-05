# ADR 0006: Staged MVP, desktop shell and stack

**Status:** Accepted (shell choice conditional on SPIKE-4; Rust skill is user decision 1)  
**Date:** 2026-11-05 · **Merges:** [archived ADR 0001](archive/0001-desktop-shell-tauri-v2.md)

## Context
The MVP is PDF + Markdown, history as the core, companion to Zotero and Obsidian. The reader (rendering, anchoring, three operating systems) is the most expensive part and is not needed to record and show history. Lightness is a selling point, so the shell and libraries are chosen by measured weight and by being the boring, widely used option.

## Decision
- **Stages.** Stage 1 = recorder and history viewer over Zotero marks and Markdown notes; it never opens a PDF. Stage 2 = own PDF/Markdown reader, built only after the product gate. *Why:* cuts the largest cost and risk until users show the history is wanted.
- **One Rust core library** (domain + use cases) used by the desktop app and the CLI. *Why:* one implementation of every rule, testable without a window.
- **Shell: Tauri v2**, UI in TypeScript with **React + Vite** (boring choice: largest contributor pool; the web view, not the UI library, dominates weight). Rust crates: `serde`/`serde_json`, `uuid`, `sha2`, `clap`, `thiserror`, `ureq` (small blocking HTTP client, no async runtime in the core); tests: `proptest`, `wiremock`.
- **Rejected:** *Electron* ships its own browser (runtime alone 121–158 MB zipped). *CLI + local web UI* is tiny (a std-only Rust server binary measured 0.35 MB) but needs a listening socket (attack surface, breaks the one-network-destination story), and a browser page cannot open folders or files, so the server would need its own native dialogs.
- **Gate:** SPIKE-4 ([PROTOTYPE-PLAN](../PROTOTYPE-PLAN.md)) renders a 50 MB PDF on WebKitGTK, WebView2 and WKWebView before Stage 1 code. Pass → no later shell switch. Fail → reopen this ADR while nothing is built.

## Sourced numbers (all vendor or single-run; our own SPIKE-4 measurement replaces them)
| Item | Value | Source, date |
|---|---|---|
| Tauri minimal app | "as little as 600 KB" (vendor claim) | tauri.app, read 2026-11-05 |
| Electron runtime zip | 121.2 MB (macOS arm64), 124.1 MB (Linux x64), 158.2 MB (Windows x64) | GitHub API `electron/electron` v45.2.3, 2026-11-05 |
| Same app, bundle | Tauri 8.6 MiB vs Electron 244 MiB | [gethopp, dev.to](https://dev.to/gethopp/tauri-vs-electron-performance-bundle-size-and-the-real-trade-offs-1el4), 2025-07-07, macOS, N=1 |
| Same app, memory (6 windows) | Tauri ~172 MB vs Electron ~409 MB | same |
| Startup | difference negligible (< 1.5 s both) | same |
| Idle memory, older test | ~80 MB vs ~120 MB | [levminer.com](https://www.levminer.com/blog/tauri-vs-electron), 2022 |
| Std-only Rust local server | 0.35 MB | measured here 2026-11-05, rustc 1.99.1, `opt-level=s`, stripped |
Other blog roundups give 3× to 10× different memory figures, which is why the budget is checked by measurement, not by quoting.

## Consequences
+ Smallest realistic installer and memory on mainstream tech; core reusable by CLI and a future plug-in front end. − Three system web views behave differently (that is SPIKE-4). − If nobody writes Rust, flip to Electron with a JavaScript core.

Security: the window has no file, socket or OS access; every action is a typed, allow-listed command (SEC-2, 15, 16).
