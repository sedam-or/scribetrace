# Security Review & Triage: Dependabot Alerts (16 Alerts)

**Reviewer:** Security Lead / Backend  
**Date:** 2026-11-06  
**Status:** RESOLVED (0 Open Alerts)  
**Ticket:** `c3cf814e-591a-46f1-8007-9516358fdb47` (`#25`)

---

## 1. Executive Summary

A security audit triaged all 16 open Dependabot alerts on `main`:
* **14 npm alerts (1 critical, 10 high, 3 medium)** in `spikes/tauri-webview` (vulnerable `pdfjs-dist` 4.0.379 and its transitive dependency `tar` 6.2.1 via `canvas` 2.11.2).
* **2 Rust alerts (medium)** in `Cargo.lock` (product workspace) and `spikes/tauri-webview/Cargo.lock` for crate `glib` 0.18.5 (`GHSA-wrw7-89jp-8q8g` / `RUSTSEC-2024-0429`).

**Result:**
* `spikes/tauri-webview` updated to `pdfjs-dist` 4.10.38, which completely eliminates `canvas` and `tar`. `pnpm audit` now reports **0 vulnerabilities**.
* All 14 npm alerts (Alerts 2–15) resolved and dismissed on GitHub with reason `not_used` / patched.
* The 2 Rust `glib` alerts (Alerts 1 & 16) were analyzed for reachability: the vulnerable `VariantStrIter` symbol is never invoked by ScribeTrace or the Tauri shell, and no backport exists for GTK3 / glib 0.18 on Linux. Both alerts dismissed on GitHub with documented justification and explicitly configured in `deny.toml` and `.github/workflows/ci.yml`.
* **Total open alerts remaining on GitHub:** **0**.

---

## 2. Detailed Alert Breakdown & Remediation

| Alert # | Ecosystem | Package | Severity | Manifest Path | Advisory ID | Status | Resolution / Justification |
| :---: | :---: | :---: | :---: | :---: | :---: | :---: | :--- |
| **#1** | Rust | `glib` | Medium | `spikes/tauri-webview/Cargo.lock` | `GHSA-wrw7-89jp-8q8g` / `RUSTSEC-2024-0429` | **Dismissed** (`not_used`) | Tauri Linux GTK3 stack pins `glib` 0.18.5; `VariantStrIter` is never called. |
| **#2** | npm | `pdfjs-dist` | High | `spikes/tauri-webview/package.json` | `GHSA-wgrm-67xf-hhpq` | **Dismissed** (`not_used` / patched) | Upgraded to `pdfjs-dist` 4.10.38 (patched >= 4.2.67). |
| **#3** | npm | `pdfjs-dist` | High | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-wgrm-67xf-hhpq` | **Dismissed** (`not_used` / patched) | Upgraded lockfile to `pdfjs-dist` 4.10.38. |
| **#4** | npm | `tar` | High | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-8qq5-rm4j-mr97` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated by upgrading `pdfjs-dist`. |
| **#5** | npm | `tar` | High | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-r6q2-hw4h-h46w` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated. |
| **#6** | npm | `tar` | High | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-34x7-hfp2-rc4v` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated. |
| **#7** | npm | `tar` | High | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-83g3-92jg-28cx` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated. |
| **#8** | npm | `tar` | High | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-qffp-2rhf-9h96` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated. |
| **#9** | npm | `tar` | High | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-9ppj-qmqm-q256` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated. |
| **#10** | npm | `tar` | Medium | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-vmf3-w455-68vh` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated. |
| **#11** | npm | `tar` | Medium | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-gvwx-54wh-qm9j` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated. |
| **#12** | npm | `tar` | High | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-8x88-c5mf-7j5w` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated. |
| **#13** | npm | `tar` | Critical | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-23hp-3jrh-7fpw` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated. |
| **#14** | npm | `tar` | Medium | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-w8wr-v893-vjvp` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated. |
| **#15** | npm | `tar` | High | `spikes/tauri-webview/pnpm-lock.yaml` | `GHSA-r292-9mhp-454m` | **Dismissed** (`not_used` / patched) | Transitive dependency `tar` eliminated. |
| **#16** | Rust | `glib` | Medium | `Cargo.lock` | `GHSA-wrw7-89jp-8q8g` / `RUSTSEC-2024-0429` | **Dismissed** (`not_used`) | Product workspace: `glib::VariantStrIter` unreachable. Documented in `deny.toml`. |

---

## 3. Product Workspace Rust Audit: `glib` Reachability Analysis

### Advisory Details
* **ID:** `GHSA-wrw7-89jp-8q8g` / `RUSTSEC-2024-0429`
* **Title:** Unsoundness in `Iterator` and `DoubleEndedIterator` impls for `glib::VariantStrIter`
* **Affected Versions:** `>= 0.15.0, < 0.20.0`
* **First Patched Version:** `0.20.0`
* **Upstream Advisory URL:** https://rustsec.org/advisories/RUSTSEC-2024-0429.html

### Reachability and Risk Assessment
1. **Dependency Path:** `scribetrace-shell` -> `tauri` 2.12.5 -> `tauri-runtime-wry` -> `wry` -> `webkit2gtk` / `gtk` -> `glib` 0.18.5.
2. **Core Boundary (SEC-1, SEC-2, SEC-15):** `scribetrace-core` has zero UI/GTK/Tauri dependencies (`cargo tree -p scribetrace-core -i glib` is empty).
3. **Application Shell:** `scribetrace-shell` uses Tauri strictly as an IPC broker (`open_workspace`, `import_marks`, `read_history`). Neither our code nor Tauri's window initialization invokes `VariantStrIter::impl_get` or iterates over GVariant string arrays.
4. **Upstream Fix Feasibility:** `gtk-rs` introduced the fix in version 0.20.0, which contains breaking API changes and is not adopted by the current Tauri v2 Linux GTK3 backend. No 0.18 backport exists.
5. **CI Policy & Revisit Trigger:** Ignored strictly in `deny.toml` under `[advisories.ignore]` and in `ci.yml` (`cargo audit --ignore RUSTSEC-2024-0370 --ignore RUSTSEC-2024-0429`). Revisit trigger: when Tauri's Linux backend moves to `gtk-rs >= 0.20` or when ScribeTrace calls any `glib::Variant` API. Any new unpatched high/critical vulnerability will immediately fail CI.
6. **Stage 2 Note:** When Stage 2 introduces PDF rendering, `pdfjs-dist` must be pinned to `>= 4.2.67` (or latest) with `isEvalSupported: false` per CVE-2024-4367 mitigation guidelines.

---

## 4. Verification

* `gh api repos/sedam-or/scribetrace/dependabot/alerts?state=open` returns `[]` (0 open alerts).
* `pnpm audit` in `spikes/tauri-webview`: 0 vulnerabilities found.
* `pnpm audit` in repo root: 0 vulnerabilities found.
* `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok`.
* `cargo audit --ignore RUSTSEC-2024-0370 --ignore RUSTSEC-2024-0429`: clean exit 0.
* `./tools/check-gates.sh`: all gates passed.
