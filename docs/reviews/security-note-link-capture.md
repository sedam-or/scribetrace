# Security Note: Link Capture, Lean Architecture v3.0 & Network Boundary Review

**Document Version:** 1.0  
**Date:** 2026-11-05  
**Author:** Security Lead (agent-1793872772619-82tneo)  
**Target:** Ticket #8, `docs/adr/0013-links-via-zotero-connector.md`, `docs/ARCHITECTURE.md` (v3.0), and trimmed ADRs 0007 & 0012  
**Status:** Approved  

---

## 1. Security Analysis of Link Capture (ADR 0013)

ADR 0013 evaluates how Palimpsest handles web links, arXiv preprints, and online papers. It explicitly rejects building a custom browser extension, a web clipper, or in-app URL fetchers, instead relying entirely on Zotero's established browser Connector.

### 1.1 Attack Surface Elimination
From a security and threat-modeling perspective, **this is an exceptional architectural decision**:
1. **No Browser Extension Attack Surface:** Custom browser extensions introduce privileged extension background scripts, cross-origin communication bridges to the desktop shell, and permission risks in browser extension stores. Omitting an extension eliminates an entire category of browser-to-desktop privilege escalation exploits.
2. **Zero Paywall & Credential Handling:** Web clippers frequently require managing user session cookies, university proxy logins, or institutional Shibboleth/SSO authenticators. Handling credentials or scraping authenticated web sessions would expand Palimpsest's liability into credential theft and GDPR/session leakage.
3. **No Dynamic Web Scraping or Script Execution:** Parsing arbitrary, hostile web pages directly (HTML extraction, reader mode scripts) is a primary vector for DOM XSS and prototype pollution. By treating web captures strictly as Zotero-saved PDF attachments or inert metadata items, Palimpsest maintains a clean, hardened boundary.
4. **Resilience to Link Rot:** Because each imported highlight persists its exact quoted text along with page/box coordinates, notes remain readable and verifiable even if the original URL rots, changes, or goes behind a paywall.

---

## 2. Security Analysis of Lean Architecture v3.0 & Trimmed ADRs

### 2.1 Single-Destination Network Rule (ADR 0012 & ARCHITECTURE v3.0 §2, §4)
* **Design Delta:** In v2.0, the network rule permitted two click-only exceptions: Zotero loopback and an in-app HTTPS GET to fetch `latest.json`. In v3.0, the in-app update check is completely removed. Clicking "Check for Updates" delegates to the system default browser opening the static GitHub release URL.
* **Security Impact:**
  - The application binary now has **exactly one network destination:** `127.0.0.1:23119` (Zotero loopback), invoked only on explicit user click.
  - Zero outbound HTTP client code is compiled into the binary for external web traffic. This eliminates TLS certificate validation, HTTP timeout handling, and version-manifest parsing vulnerabilities.
  - The privacy guarantee is absolute: *Palimpsest never contacts any remote server on the internet.*

### 2.2 Sequence-Based Purge with `covers` Map (ADR 0007)
* **Design Delta:** Purge events previously relied on HLC timestamps (`hlc <= purge.hlc`). In ADR 0007 (slimmed), purge events store an exact sequence vector: `covers = {device: highest seq seen}`.
* **Security Impact:**
  - Clock-free purge evaluation: Because sequence numbers are strictly monotonically increasing per device, purge filtering is a pure function immune to host clock rollbacks, timezone manipulation, or deliberate timestamp spoofing.
  - Offline isolation: Events written offline that were not seen by the purging device at the time of purge are explicitly marked *"not covered by the erase"*. This prevents an offline device from having its non-conflicting work erased by surprise, while still allowing a subsequent purge to cover them.

### 2.3 Simplified Device State in `device.json` (ADR 0007)
* **Design Delta:** Device state is reduced to `{ device_id, papers: { <paper-id>: { last_id } } }`, stored in per-user OS app storage (`appdata`).
* **Security Impact:**
  - Solves the multi-machine clone/restore race condition: before appending, the engine checks whether the last line of `<device>.jsonl` matches `last_id`. If not (e.g. a restored VM backup or cloned drive), Palimpsest mints a new random UUIDv4 device ID, preserving the integrity of existing logs without risk of overwriting.

### 2.4 Removal of `corrupt/` Quarantine Files (ADR 0007 & ARCHITECTURE v3.0 §6)
* **Design Delta:** Instead of copying malformed bytes to `.palimpsest/corrupt/`, bad lines are skipped on read and reported via a non-modal count in the UI and CLI `check`.
* **Security Impact:**
  - Prevents disk exhaustion and filesystem spam attacks where a hostile collaborator commits millions of corrupt lines to flood the user's disk with diagnostic quarantine files.

---

## 3. Enforcement Tagging Framework

To provide unambiguous guidance for engineering implementation and QA validation, all 36 Security Requirements are classified into three distinct enforcement tiers in `docs/SECURITY-REQUIREMENTS.md` (v3.0):

1. **`[Enforced in Code]` (21 items):** Mandatory programmatic controls implemented directly in the Rust core, format validators, or webview presentation components.
2. **`[Process]` (8 items):** Governance, build pipeline, supply-chain verification, CI automated checks, and maintainer infrastructure policies.
3. **`[Deferred]` (7 items):** Requirements belonging to Stage 2 (own reader), reflowable formats (EPUB/HTML), or future scope.

---

## 4. Conclusion & Approval

The lean Architecture v3.0 and ADR 0013 represent an ideal security posture for an open-source scholarly tool: minimal attack surface, boring and proven primitives, zero custom network protocols, and an uncompromised privacy guarantee.

**Status: APPROVED.**
