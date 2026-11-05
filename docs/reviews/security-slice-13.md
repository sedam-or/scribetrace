# Security Review: Slice 13 (CLI `scribetrace-cli`)

**Reviewer:** Security Lead / Backend  
**Commit:** `1277908`  
**Target:** `src-cli/`, `src-core/src/adapters/jsonl_store.rs` (`list_papers`), golden tests  
**Status:** APPROVED (0 Blockers)  
**Date:** 2026-11-06  

---

## 1. Scope & Verification

Slice 13 introduces the `scribetrace-cli` standalone binary crate (ADR 0010) supporting:
- `scribetrace-cli check <folder>`: Non-mutating validation of paper history stores, reporting per-paper counts, unknown lines, and malformed lines categorized by reason code (exit `0` on clean, `1` on bad lines, `2` on usage/IO errors).
- `scribetrace-cli export <folder> [paper-id]`: Non-mutating export of paper history as sanitized Markdown to `stdout`.

This review evaluates `src-cli` and `FsHistoryStore::list_papers` against the security requirements (SEC-14, SEC-15, SEC-18, SEC-19, SEC-20, SEC-21) and safe parsing of untrusted text.

---

## 2. Security Invariants Audit

| Security Property | Requirement | Implementation in Slice 13 | Audit Verdict |
| :--- | :--- | :--- | :---: |
| **Zero Mutation Invariant** | **SEC-19** | Zero file modification, truncation, or creation within `<folder>`. CLI operates purely over `open_store`, calling `read_report` and writing output exclusively to `stdout`/`stderr`. Tested by golden tests verifying zero file system alterations. | **PASS** |
| **Zero Network Invariant** | **SEC-1** | Zero network libraries or dependencies included in `src-cli/Cargo.toml`. No sockets, DNS, or HTTP connections opened. Completely offline. | **PASS** |
| **Path Traversal & Injection Defense** | **SEC-14**, **SEC-17**, **SEC-21** | `paper-id` argument is strictly validated via `PaperId::new(id)` before use, rejecting `..`, `/`, `\`, null bytes, and characters outside `^[a-zA-Z0-9_-]{1,64}$`. In `list_papers()`, `fs::read_dir` skips non-directories (and symlinks via `entry.file_type()?.is_dir()`), and silently ignores any directory entry that does not validate as a valid `PaperId`. | **PASS** |
| **Terminal Injection / ANSI Escape Defense** | **SEC-20** | `clean()` filters out ASCII/Unicode control characters (`c.is_control()`) except newline, normalizes tabs to spaces, and strips Unicode bidirectional override characters (`\u{202A}`..=`\u{202E}`, `\u{2066}`..=`\u{2069}`, `\u{200E}`, `\u{200F}`). Prevents terminal hijacking, hidden execution, or visual spoofing of CLI logs. | **PASS** |
| **Markdown Injection Defense** | **SEC-20** | `md_escape()` backslash-escapes all Markdown syntax characters (`\`, `` ` ``, `*`, `_`, `[`, `]`, `<`, `>`, `&`, `#`, `|`, `~`, `!`), neutralizing raw HTML tags and link injection from untrusted annotation text. | **PASS** |
| **Crash-Proof Reading & Memory Limits** | **SEC-12**, **SEC-13** | Built directly on `FsHistoryStore::read_report` using the bounded line reader (max 1 MB per line) and `classify_line` parser. Malformed or oversized lines are counted and isolated under `bad_lines` without memory exhaustion or crash. | **PASS** |

---

## 3. Findings & Recommendation

1. **`list_papers` symlink handling:** `entry.file_type()?.is_dir()` on Unix does not follow symlinks (it returns `false` for directory symlinks), ensuring rogue symlinks to system folders are not traversed.
2. **Deterministic Output:** `BTreeMap<&String, &usize>` ensures reason codes are printed in stable lexicographical order across runs.

**Verdict:** **APPROVED**. Commit `1277908` is secure, robust, and adheres strictly to all ScribeTrace security constraints.
