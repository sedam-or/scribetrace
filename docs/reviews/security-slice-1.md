# Security Review: Slice 1 (Walking Skeleton)

**Reviewer:** Security Lead / Backend  
**Commit:** `2aa5c56`  
**Target:** `src-core/`, `src-tauri/`, `src/rpc/`, capabilities and layering gates  
**Status:** APPROVED (0 Blockers)  
**Date:** 2026-11-06  

---

## 1. Scope & Verification

Slice 1 establishes the structural scaffold and walking skeleton for ScribeTrace. This review validates alignment with `docs/SECURITY-REQUIREMENTS.md` (v3.0, Lean) and the architectural invariants in `docs/ARCHITECTURE.md`.

---

## 2. Security Invariants Audit

| Security Property | Requirement | Implementation in Slice 1 | Audit Verdict |
| :--- | :--- | :--- | :---: |
| **Single Loopback Destination & Broker Defenses** | **SEC-1**, **SEC-33** | `ZoteroHttp` client in `src-core/src/adapters/zotero_http.rs` connects strictly to `http://127.0.0.1:<port>/api/users/0/items/...` via `ureq`. Zero outbound internet sockets. Direct TLS stacks (`rustls`, `native-tls`) are strictly excluded from core (`tools/check-gates.sh` passes). Defenses verified: `max_redirects(0)` (no SSRF redirects), 5s global timeout, 15 MB body cap, strict `Zotero-API-Version: 3` header check, and attachment key alphanumeric validation (`[A-Za-z0-9]`) preventing path/query injection before URL construction. | **PASS** |
| **Zero Shell / IPC Confinement** | **SEC-15**, **SEC-16** | Tauri shell plugin is absent. The IPC gateway in `src-tauri/` defines a strict allowlist in `build.rs` and `capabilities/default.json` (`allow-open-workspace`, `allow-import-marks`, `allow-read-history`). Unregistered commands are rejected by Tauri v2. | **PASS** |
| **Path Jail & Traversal Resistance** | **SEC-14**, **SEC-17**, **SEC-21** | `PaperId` and `DeviceId` domain types (`src-core/src/domain/ids.rs`) enforce strict ASCII alphanumeric + `_` + `-` character bounds (max 128 chars). Path traversal tokens (`..`, `/`, `\`, `\0`) are rejected at the domain boundary. `open_workspace` canonicalizes directory paths. | **PASS** |
| **Sacred Source Documents** | **SEC-19** | Zero file write handles are opened on original source documents. History is written strictly into `.scribetrace/papers/<paper-id>/`. | **PASS** |
| **Clean Seam & UI Boundary** | **SEC-2**, **SEC-8** | `src/` talks to core exclusively through typed `Rpc` interface (`src/rpc/rpc.ts`). `@tauri-apps/api` is isolated in `tauri-adapter.ts`. Verified by `tools/lint-ui-boundaries.mjs`. | **PASS** |
| **Memory Safety & Robustness** | **DoD Rule 3** | Workspace lints enforce `deny` on `unwrap_used`, `expect_used`, `panic`, `todo`, `dbg_macro`, and `forbid` on `unsafe_code`. | **PASS** |

---

## 3. Findings & Notes for Subsequent Slices

* **Temporary Placeholder Device ID (Slice 1):** `src-tauri/src/commands.rs` uses `"local-1"` as a placeholder device id. Slice 2 will replace this with persistent per-user app storage (`device.json`) and UUIDv4 generation per **SEC-34**.
* **Bad Line Counting (Slice 1):** `jsonl_store.rs` currently silently drops malformed lines. Slice 3 will implement the full `ReadResult` counter and 1 MB line guard per **SEC-12** and **SEC-13**.

**Verdict:** **APPROVED**. Slice 1 meets all architectural and security gates. Ready for Slices 2 and 3 implementation.
