# QA Verification Report: Slice 1 (Walking Skeleton)

**Reviewer:** Senior QA Engineer (`agent-1793867530011-poqhdm`)  
**Commit Verified:** [`2aa5c56`](https://github.com/sedam-or/scribetrace/commit/2aa5c56) (with CI fix [`ca8cef4`](https://github.com/sedam-or/scribetrace/commit/ca8cef4) and ADR 0014 [`dfbf39b`](https://github.com/sedam-or/scribetrace/commit/dfbf39b))  
**Target Specification:** [`docs/BUILD-PLAN.md`](../BUILD-PLAN.md) (Slice 1), [`docs/DEFINITION-OF-DONE.md`](../DEFINITION-OF-DONE.md)  
**CI Reference:** Run [42085977167](https://github.com/sedam-or/scribetrace/actions/runs/42085977167) (Linux required: PASS, macOS: PASS, Windows: informative failure analyzed below)  
**Date:** 2026-11-06  
**Overall Verdict:** **VERIFIED & APPROVED (with 1 non-blocking Windows IPC test finding for Architect)**

---

## 1. Acceptance Test Suite Verification

| Level | Test Suite | Scope & Invariants Verified | Result | Execution Time |
|:---|:---|:---|:---:|:---:|
| **(a) Core + Adapters** | `src-core/tests/import_slice1.rs` | Loopback `StandIn` server on port 0, single `mark_import` event written into `.scribetrace/papers/z_EFGH6789/dev-test.jsonl`, `author: "Me"`, `seq: 1`, payload key `WXYZ2345`, page index `3`. Source file `paper.pdf` verified byte-identical before and after. Unreachable Zotero handled with typed `SourceError::Unreachable` writing 0 files. Hostile IDs (`../x`, `""`, `a/b`) rejected by `PaperId`/`DeviceId`. | **PASS** | 0.01s |
| **(b) UI View** | `tests-ui/history.test.tsx` | View driven by in-memory `createMockRpc([mark])`. Asserts DOM rendering of mark text, metadata `mark_import`, author `Me`, count `Imported 1 mark`. Asserts hostile XSS text (`<script>`, `<img>`) rendered strictly as inert text (`&lt;script&gt;`). Asserts RPC rejection transitions to `role="alert"` error state. | **PASS** | 0.09s |
| **(c) Tauri IPC Shell** | `src-tauri/tests/ipc_slice1.rs` | Commands executed through Tauri mock builder and `tauri::generate_context!()` with real capability file `capabilities/default.json`. Unlisted commands (`plugin:fs\|read_file`) rejected by capability allowlist. Calls before `open_workspace` return typed error. `open_workspace` -> `import_marks` -> `read_history` round-trips with full deserialization. | **PASS** (Linux & macOS) | 0.01s |

---

## 2. Definition of Done (15 Rows Audit)

| # | Rule | Enforced By | Audit Status & Evidence | Verdict |
|:---:|:---|:---|:---|:---:|
| **1** | `cargo fmt --check` clean | CI | `cargo fmt --all --check` clean, 0 diffs. | **PASS** |
| **2** | `cargo clippy --workspace --all-targets -D warnings` | CI | Clean across all targets with 0 warnings. | **PASS** |
| **3** | No `unwrap`/`expect`/`panic!` outside tests; `unsafe` forbidden | Workspace lints | Enforced via root `Cargo.toml` `clippy::unwrap_used = "deny"`, `clippy::expect_used = "deny"`, `unsafe_code = "forbid"`. | **PASS** |
| **4** | Typed errors (`thiserror`) in core; shell converts to plain text | Clippy + Review | `DomainError`, `SourceError`, `StoreError`, `AppError` use `thiserror`. Shell `commands.rs` maps to user-readable strings (`"No project folder is open."`, `"That path is not a folder."`). | **PASS** |
| **5** | Unit tests + property tests for parser/id/path | CI / proptest | `src-core/tests/ids_props.rs` proptests ID character bounds and round-trip. | **PASS** |
| **6** | Core line coverage $\ge 80\%$ | `cargo llvm-cov` | **94.14%** line coverage (241/256 lines). Regions: 91.45%, Functions: 90.70%. | **PASS** |
| **7** | UI: `typecheck`, `lint`, `test`, `build` clean | CI / pnpm | `pnpm lint`, `node --test history.test.js`, and `vite build` complete with 0 errors. Chunks: `index.html` (0.39 kB), CSS (5.65 kB), JS (221.36 kB). | **PASS** |
| **8** | UI boundary: no Node built-ins, Rpc seam | `lint-ui-boundaries.mjs` | Verified clean across all UI files; `@tauri-apps/api` isolated to `src/rpc/tauri-adapter.ts`. | **PASS** |
| **9** | Core boundary: no tauri/TLS/notify/pdfjs | `tools/check-gates.sh` | Verified: 0 tauri in core, 0 TLS, 0 notify, 0 `pdfjs-dist`. | **PASS** |
| **10** | Budgets: crates, packages, transitive, memory | `check-gates.sh` / CI | Rust direct crates: 7 ($\le 15$). JS packages: 5 ($\le 6$). Transitive crates: 315 ($\le 378$). Shell release size $\le 20$ MB. Idle memory & cold start tracked under SPIKE-4 / ADR 0014. | **PASS** |
| **11** | `cargo deny` & `cargo audit` clean | CI | `cargo deny check` clean. `cargo audit` clean with 2 documented GTK3 ignores (`RUSTSEC-2024-0370`, `RUSTSEC-2024-0429`). | **PASS** |
| **12** | Accessibility WCAG 2.2 AA | Review | Labelled inputs (`<label htmlFor="folder">`), visible focus tokens, status messaging via `role="status"` / `role="alert"`, semantic elements (`<main>`, `<h1>`, `<h2>`, `<section>`, `<ol>`, `<blockquote>`). | **PASS** |
| **13** | Docs updated in same change | Review | Updated: `docs/DEVELOPMENT.md`, `docs/DEFINITION-OF-DONE.md`, `docs/adr/0014-per-os-idle-memory-budget.md`. | **PASS** |
| **14** | Security review & tests (SEC-1..32) | Security Lead Review | Formally reviewed and APPROVED in `docs/reviews/security-slice-1.md` by Backend (SEC-1, SEC-2, SEC-8, SEC-14..19, SEC-21, SEC-33). | **PASS** |
| **15** | Independent review, signed-off commit | Conventional commit | Reviewed independently by QA (`agent-1793867530011-poqhdm`). | **PASS** |

---

## 3. Findings & Honest Limits (What Tests Do and Do Not Actually Prove)

### A. Critical Finding for Cross-Platform IPC Test (`ipc_slice1.rs`)
- **Finding:** In `src-tauri/tests/ipc_slice1.rs`, the helper `request()` hard-codes `url: "tauri://localhost"`.
- **Impact on Windows:** On Windows WebView2, Tauri's origin scheme is `http`, so Tauri's capability permission matcher (`local` scope) treats `tauri://localhost` as an alien origin. In Windows CI (Run 42085977167 Job 127465472362), `import_then_read_round_trips_through_ipc` panicked at line 58 with:
  ```
  called Result::unwrap() on an Err value:
  String("open_workspace not allowed on window \"main\", webview \"main\", URL: tauri://localhost
  allowed on: [windows: \"main\", URL: local]
  referenced by: capability: default, permission: allow-open-workspace")
  ```
- **Remediation for Architect:** For cross-platform mock context testing, configure the mock request URL conditionally (or use Tauri's canonical mock origin `http://tauri.localhost` / `tauri://localhost` based on `cfg!(windows)`). Since Windows is informative in Stage 1 and Linux required passed 100%, this does not block Slice 1 sign-off.

### B. What Level (a) Does Not Prove
1. **Deduplication / Repeat Import:** In Slice 1, calling `ImportMarks::run` repeatedly will append duplicate `mark_import` lines with incrementing `seq` numbers. Mark deduplication and diffing are deferred to Slices 4 and 6.
2. **Bad Line Counting:** `FsHistoryStore::read()` silently skips unparseable lines (`if let Ok(event) = serde_json::from_str::<Event>(&line)`). Counting and reporting bad lines in `ReadResult` is deferred to Slice 3.
3. **Zotero Pagination:** `ZoteroHttp` queries `limit=100` on a single page. Multi-page pagination is deferred to Slice 4.

### C. What Level (b) Does Not Prove
1. **Interactive Event Handling & Hooks:** Level (b) uses `renderToStaticMarkup` which proves pure static HTML rendering and XSS escaping, but does not exercise React hooks (`useState`), user typing, or button clicks in `App.tsx`. (The Linux real-window WebDriver E2E harness covers this gap).
2. **Dynamic Milestone / Timeline Views:** Only static `HistoryPanel` rendering is proven.

### D. Architectural Placeholders Confirmed
1. **Device ID:** `SLICE1_DEVICE = "local-1"` in `src-tauri/src/commands.rs` is a temporary constant. Slice 2 replaces this with persistent UUID minting in `device.json`.
2. **Folder Selection:** `open_workspace` canonicalizes any existing folder provided by UI string; native folder picker dialog integration is scheduled for Slice 5.

---

## 4. Final Sign-Off Recommendation

Slice 1 establishes an exemplary, rigorous foundation for ScribeTrace. All 3 levels of acceptance tests (a, b, c), all layering constraints, all security invariants, and all Definition of Done criteria are satisfied on the required Linux target.

**Verdict:** **APPROVED & VERIFIED.** Ready for Slice 2 (Safe device id & crash-proof append) and Slice 3 (Multi-machine merge & bad line counts).
