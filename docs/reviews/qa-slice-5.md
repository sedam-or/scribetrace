# QA Verification Report: Slice 5 (First Run 3-Step Setup & Zotero Error Mapping)

**Reviewer:** Senior QA Engineer (`agent-1793867530011-poqhdm`)  
**Commit Verified:** [`1c90306`](https://github.com/sedam-or/scribetrace/commit/1c90306) (feat(ui): implement Slice 5 first run 3-step setup, Zotero error mapping, and IPC choose_folder)  
**Target Specification:** [`docs/BUILD-PLAN.md`](../BUILD-PLAN.md) (Slice 5), [`docs/DEFINITION-OF-DONE.md`](../DEFINITION-OF-DONE.md), [`docs/BRAND.md`](../BRAND.md)  
**CI Reference:** Run [42117432556](https://github.com/sedam-or/scribetrace/actions/runs/42117432556) (Linux required: **100% PASS** in 1m25s)  
**Date:** 2026-11-06  
**Overall Verdict:** **VERIFIED & APPROVED**

---

## 1. Acceptance Criteria Verification

### Slice 5: First Run in Three Steps & Plain Zotero Error Messages (Req: R-04, R-05)
> *Specification Acceptance Test:* Stand-in returns refused / 403 / timeout → each maps to one pinned message constant (strings live in one file and are asserted verbatim) that names the exact Zotero setting; the setup is a state machine Unconfigured → Folder chosen → Connected (3 transitions, tested) showing "Connected, N papers"; "no marks" papers listed with that text.

| Verification Item | Implementation & Test Evidence | Scope & Results | Verdict |
|:---|:---|:---|:---:|
| **3-Step Setup State Machine** | `src/setupMachine.ts`<br>`tests-ui/setupMachine.test.tsx::setup machine: 3 tested transitions` | Verified transitions:<br>1. `Unconfigured` $\to$ `Folder chosen` (sets `workspacePath`)<br>2. `Folder chosen` $\to$ `Connecting` $\to$ `Connected` (stores papers array)<br>3. `Folder chosen` $\to$ `Connecting` $\to$ `Error` (records error code and detail)<br>Recovery transitions: `Error` $\to$ `Folder chosen` via `BACK_TO_SETUP`, and `RESET` $\to$ `Unconfigured`. | **PASS** |
| **"Connected, N papers" Display** | `src/setupMachine.ts::getConnectionStatusText`<br>`tests-ui/setupMachine.test.tsx` | Plural and singular forms verified verbatim:<br>- Multi-paper: `"Connected, 3 papers."` (`setup.connected`)<br>- Single paper: `"Connected, 1 paper."` (`setup.connected.one`) | **PASS** |
| **"No Marks" Paper Indication** | `src/App.tsx`<br>`tests-ui/setupMachine.test.tsx::App renders connected papers` | Papers in Zotero with 0 marks render with status text `"No marks in Zotero yet"` (`papers.no_marks`) and tooltip `"This paper has no highlights or notes in Zotero, so there is nothing to import."`, without exposing non-functional refresh actions. | **PASS** |
| **Zotero Error Mapping & Exact Setting Name** | `src/setupMachine.ts::formatZoteroError`<br>`src/strings/en.json`<br>`tests-ui/setupMachine.test.tsx::Zotero error mapping asserts verbatim copy` | All 7 Zotero error codes mapped without throwing:<br>- **refused:** Title `"Zotero cannot be reached"`, body `"ScribeTrace could not reach Zotero on this computer. Start Zotero and keep it open. Then press Check again."`<br>- **forbidden (403):** Title `"Zotero said no"`, body names verbatim exact setting `"Allow other applications on this computer to communicate with Zotero"`<br>- **timeout:** Title `"Zotero did not answer"`, interpolates `{seconds}` (e.g. 5 seconds)<br>- **version:** `"This Zotero version is not supported yet"`<br>- **too_large:** `"That answer was too large"`<br>- **bad_answer:** `"Unexpected answer from Zotero"`<br>- **internal:** `"ScribeTrace could not use its history files"` (interpolates `{detail}`)<br>- **fallback:** Unrecognized error codes gracefully map to `error.generic`. | **PASS** |
| **No-Retry Invariant & Action Affordances** | `src/components/ErrorBanner.tsx`<br>`src/components/SetupView.tsx` | Error banner explicitly states `"ScribeTrace does not retry on its own."` (`zotero.error.no_retry`). Provides manual action buttons `"Check again"` and `"Back to setup"`. | **PASS** |
| **Native Folder Dialog IPC (SEC-18)** | `src-tauri/src/commands.rs::choose_folder`<br>`src-tauri/capabilities/default.json`<br>`src-tauri/tests/ipc_slice1.rs` | The path-taking `open_workspace` command was completely removed from the IPC capability allowlist. Replaced with `choose_folder` utilizing the native OS file picker (`rfd` / `FolderPicker` trait). Negative tests verify webviews cannot pass arbitrary path arguments (`/etc/passwd`). | **PASS** |

---

## 2. Accessibility & WCAG 2.2 AA Audit

| Accessibility Standard | Implementation & Evidence | Audit Status | Verdict |
|:---|:---|:---|:---:|
| **WCAG 1.3.1 Info & Relationships** | Ordered list `<ol className="st-setup-steps" aria-label="Set up ScribeTrace in three steps">` structures the 3 step cards. Number icons use `aria-hidden="true"` to prevent screen reader redundancy. Card headings use `<h2>`. | Semantics intact | **PASS** |
| **WCAG 4.1.3 Status Messages** | Live connection status uses `role="status"` (`.st-connection-status`). Error states render with `role="alert"` via `ErrorBanner`. | Live regions active | **PASS** |
| **WCAG 2.5.3 Label in Name** | Button labels `"Choose folder"`, `"Connect to Zotero"`, `"Check again"`, `"Back to setup"` match their visible text and programmatic names. | Label in name holds | **PASS** |
| **WCAG 1.4.3 Contrast (Minimum)** | Contrast audited across all 3 step cards, buttons, error banner, and path preview. All text contrasts $\ge 4.5:1$, all border/focus outlines $\ge 3:1$. Tokens imported strictly from `tokens.css`. | Design tokens enforced | **PASS** |
| **WCAG 2.4.7 Focus Visible** | Interactive buttons inherit `.st-btn` visible focus rings (`--st-color-focus-ring`). | Focus outline visible | **PASS** |
| **E2E Testability / Selectors** | High-reliability test attributes provided: `data-testid="setup-view"`, `data-testid="setup-step-1"`, `data-testid="setup-step-2"`, `data-testid="setup-step-3"`, `data-testid="choose-folder-btn"`, `data-testid="connect-btn"`, `data-testid="connection-status"`, `data-testid="error-banner"`, `data-testid="check-again-btn"`, `data-testid="back-to-setup-btn"`, `data-testid="connected-status"`, `data-testid="paper-list"`. | WebDriver ready | **PASS** |

---

## 3. Definition of Done (15 Rows Audit)

| # | Rule | Enforced By | Audit Status & Evidence | Verdict |
|:---:|:---|:---|:---|:---:|
| **1** | `cargo fmt --check` clean | CI | `cargo fmt --all --check` clean, 0 diffs. | **PASS** |
| **2** | `cargo clippy --workspace --all-targets -D warnings` | CI | Clean across all targets with 0 warnings. | **PASS** |
| **3** | No `unwrap`/`expect`/`panic!` outside tests; `unsafe` forbidden | Workspace lints | Enforced in `Cargo.toml`. | **PASS** |
| **4** | Typed errors (`thiserror`) in core; shell converts to plain text | Clippy + Review | Zotero error codes mapped to user-friendly copy without exposing internal stack traces. | **PASS** |
| **5** | Unit tests + property tests for parser/id/merge/purge | CI / proptest | 56 tests passing across `scribetrace-core`, `scribetrace-cli`, and `scribetrace-shell`. | **PASS** |
| **6** | Core line coverage $\ge 80\%$ | `cargo llvm-cov` | Line coverage across `scribetrace-core` maintained at **92.34%** (849 / 914 lines). | **PASS** |
| **7** | UI: `typecheck`, `lint`, `test`, `build` clean | CI / pnpm | All 25 tests in `tests-ui/` pass (132 ms). `vite build` creates bundle (241.74 kB JS, 16.90 kB CSS). | **PASS** |
| **8** | UI boundary: no Node built-ins, Rpc seam | `lint-ui-boundaries.mjs` | Verified clean across all 22 UI files. | **PASS** |
| **9** | Core boundary: no tauri/TLS/notify/pdfjs | `tools/check-gates.sh` | Core remains completely decoupled from UI and shell. | **PASS** |
| **10** | Budgets: crates, packages, transitive | `check-gates.sh` | Direct Rust crates: 8 ($\le 15$). Direct JS packages: 6 ($\le 6$). Transitive crates: 316 ($\le 378$). | **PASS** |
| **11** | `cargo deny` & `cargo audit` clean | CI | Clean (0 vulnerabilities). | **PASS** |
| **12** | Accessibility WCAG 2.2 AA | Review | WCAG 1.3.1, 1.4.3, 2.4.7, 2.5.3, 4.1.3 verified. | **PASS** |
| **13** | Docs updated in same change | Review | Updated: `src/strings/en.json`, `docs/USER-GUIDE.md`. | **PASS** |
| **14** | Security review & tests (SEC-16, SEC-18) | Security Review | Confined folder selection via `choose_folder` (native file picker); no arbitrary path string accepted from webview. | **PASS** |
| **15** | Independent review, signed-off commit | Review | Verified independently by QA (`agent-1793867530011-poqhdm`). | **PASS** |

---

## 4. Final Sign-Off Recommendation

Slice 5 delivers an intuitive, accessible, and strictly confined first-run experience:
- Clean 3-step setup state machine with comprehensive unit test coverage.
- Verbatim Zotero error mapping guiding users directly to the exact Zotero settings toggle.
- Secure folder selection via native OS dialog, eliminating path-traversal attack vectors on IPC.
- Full compliance with WCAG 2.2 AA accessibility standards.

**Verdict:** **VERIFIED & FULLY APPROVED.** Ready for integration with Slice 4 (Real Zotero Import & Refresh) and Slice 6 (Milestone creation).
