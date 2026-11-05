# Security Review: Technical Architecture v2.0 & ADRs 0006–0012

**Document Version:** 1.0  
**Date:** 2026-11-05  
**Reviewer:** Security Lead (agent-1793872772619-82tneo)  
**Target:** `docs/ARCHITECTURE.md` (v2.0) and `docs/adr/0006-` through `0012-`  
**Status:** Approved (Zero Blockers; Recommendations for Implementation)  

---

## 1. Executive Summary & Verdict

The Technical Architecture Specification v2.0 and ADRs 0006–0012 successfully realign Palimpsest with the scoped PDF + Markdown MVP. The architecture cleanly separates Stage 1 (Zotero companion, history recorder, Markdown notes, export) from Stage 2 (in-app PDF/Markdown reader behind a product gate), dramatically reducing the initial attack surface.

**Verdict: APPROVED.**  
All five security objections raised in Round 6 have been either resolved in full or resolved with sound, defensible architectural trade-offs. The newly introduced mechanisms—the isolated Zotero broker, the deterministic purge epoch, app-data device IDs, notify-only update checks, and bounded Obsidian exports—adhere strictly to our core principles of least privilege, defense in depth, and honest trust boundaries.

---

## 2. Evaluation of Round 6 Security Objections

| # | Round 6 Security Objection | Disposition in v2.0 / ADRs | Architectural Rationale & Evaluation |
|---|---|:---:|---|
| **1** | **The Erase Marker Fallacy (Decision 4):** Marking an "erase requested" tombstone leaves plain-text history on disk, violating confidentiality for researchers under NDAs. | **Partly Resolved (Defensible)** | Architect accepted local deletion and the purge event rule ([ADR 0007](../adr/0007-per-device-history-log-merge-and-forget.md), [Memo §1 row 1](../debate/07-tech-decision-memo.md)). "Forget" deletes this machine's files immediately, appends an ordinary `purge` event, ignores events $\le \text{purge time}$ on read, and prompts other machines on open. Rejection of overwrite-shredding and signed purge files is accepted: overwriting is unreliable on modern journaling/copy-on-write filesystems (ext4, APFS, NTFS SSDs), and signing requires key infrastructure not available in v1. The transparent UI disclosure stating that copies in Git/cloud sync may remain satisfies honesty. |
| **2** | **The Loopback Hole (Decision 6):** Unbounded access to `127.0.0.1` opens SSRF and local port scanning vulnerabilities against host services (databases, dev servers). | **Resolved in Full** | Fully adopted in [ADR 0009](../adr/0009-zotero-integration.md) and §2.2 / §4. The webview retains `connect-src 'none'`. All Zotero calls are handled exclusively by `crates/palimpsest-zotero/` with a pinned loopback address (`127.0.0.1:23119`), no DNS lookup, no HTTP redirects, strict GET-only path allowlisting (`/api/users/0/items*`), required `X-Zotero-Version` response headers, and invocation only upon explicit user click. |
| **3** | **The "No Updates" Trap (Decision 9):** Dropping updates completely strands users on vulnerable PDF/document parsers when security flaws are patched. | **Partly Resolved (Defensible)** | Adopted manual notify-only update check ([ADR 0012](../adr/0012-distribution-updates-and-network-rule.md)). User click triggers a single HTTPS request fetching latest version metadata. The download URL is permanently hardcoded inside the binary, preventing malicious redirect poisoning. Rejection of in-place auto-updaters and certificate pinning is sound: avoids updater RCE complexity for MVP and prevents broken client connections during TLS renewal. |
| **4** | **Device ID Identity Leak (Decision 4):** Deriving `<device>.jsonl` from hostnames or usernames leaks researcher identity in public Git repos. | **Resolved in Full** | Fully adopted in [ADR 0007](../adr/0007-per-device-history-log-merge-and-forget.md). Device IDs are random anonymous UUIDs. The architect improved security by placing the ID file in **per-user OS app storage** (`appdata`), preventing two machines from sharing an ID across synced project folders. |
| **5** | **Shell Architecture (Decision 2):** Defending Tauri v2 over Electron to maintain memory safety and narrow IPC permissions. | **Resolved in Full** | Adopted in [ADR 0006](../adr/0006-staged-mvp-and-delivery-shape.md). Tauri v2 retained with a week-1 webview rendering spike, keeping the Rust core and strict typed IPC allowlist. |

---

## 3. Analysis of New Mechanisms in Architecture v2.0

### 3.1 Zotero Broker (`crates/palimpsest-zotero/`, ADR 0009)
* **Control:** Strict loopback confinement. Webview has zero socket capabilities (`connect-src 'none'`).
* **Verdict:** Secure. Enforcing a path allowlist (`/api/users/0/items*`), forbidding redirects (`allow_redirects = false`), and inspecting response headers effectively eliminates SSRF risks.
* **Recommendation:** Enforce a hard body size cap (e.g. 15 MB) and 5.0 s timeout on the HTTP client to guard against memory exhaustion if Zotero returns massive payloads.

### 3.2 Purge Semantics & Forget Rule (ADR 0007, §2.3)
* **Control:** Local file deletion + `purge` event marker in the local device log. Read operations ignore all events where `hlc <= purge.hlc`. Other machines prompt the user before unlinking historical files.
* **Verdict:** Robust. Treating `purge` as a pure function during merge ensures deterministic replay across machines without requiring complex distributed consensus. The explicit UI disclosure prevents false security assumptions regarding external sync providers.

### 3.3 Device ID in Per-User App Storage (ADR 0007)
* **Control:** `device_id` generated via random UUIDv4 and saved to local OS app configuration paths (`~/.config/palimpsest/` or `%APPDATA%/palimpsest/`).
* **Verdict:** Superior to workspace storage. It guarantees that syncing the `.palimpsest/` folder across devices via Syncthing or Git never results in ID collisions.

### 3.4 Notify-Only Update Check (ADR 0012, §2.2)
* **Control:** Manual click button executing an HTTPS GET to a static JSON manifest. The application displays a non-modal update banner with a hardcoded download URL.
* **Verdict:** Safe. By disallowing in-place execution of replacement binaries and ignoring remote download URLs, the updater surface is stripped of remote code execution (RCE) vectors.

### 3.5 Obsidian Export Overwrite Rule (ADR 0008, §2.3)
* **Control:** Writes Markdown into a user-selected folder within a dedicated, marked subfolder. Overwrites only files previously created by Palimpsest whose contents remain unchanged.
* **Verdict:** Safe. Protects against arbitrary file overwrite and sync loops within external Obsidian vaults.

### 3.6 Path Jail (ADR 0008, 0011, §4)
* **Control:** Core validates all paths using canonicalization (`dunce::canonicalize`).
* **Verdict:** Secure. Confinement extends cleanly from `<workspace>/.palimpsest/` to user-granted read folders (Zotero storage paths in Stage 2) and user-selected native export paths.

---

## 4. Ranked Findings & Recommendations

### Blockers
* **None.** The technical specification v2.0 is safe to proceed to implementation planning.

### Nice-to-Have Enhancements (Stage 1 & Stage 2 Implementation)
1. **[Nice-to-Have - Stage 1] Response Size and Timeout Bounds on Zotero Broker (ADR 0009):**  
   Configure the internal `reqwest` / `ureq` client in `crates/palimpsest-zotero/` with an explicit timeout of $5.0\text{ s}$ and a response stream limit of $15\text{ MB}$ to guarantee deterministic memory usage.
2. **[Nice-to-Have - Stage 1] Strict Schema Parsing on Update Version Manifest (ADR 0012):**  
   The update version JSON payload should be parsed with strict field types (`version`: SemVer string, `release_date`: ISO date) and text-only release notes to ensure no unexpected formatting passes into the UI.
3. **[Nice-to-Have - Stage 2] Symlink Traversal Assertion on Granted Zotero Storage Folders (ADR 0011):**  
   When resolving PDF paths returned by Zotero in Stage 2, assert that the target PDF is an ordinary file and does not resolve via a symlink to outside the user's storage hierarchy.
