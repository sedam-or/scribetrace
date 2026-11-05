# ScribeTrace: Threat Model & Trust Boundary Analysis (STRIDE) (v3.0, Lean)

**Document Version:** 3.0  
**Status:** Approved for Lean Architecture v3.0 Baseline  
**Date:** 2026-11-05  
**Product Name:** ScribeTrace  
**Document Classification:** Public / Open Source  
**Repository Path:** `docs/THREAT-MODEL.md`  

---

## 1. Executive Summary & Security Philosophy

**ScribeTrace** is an open-source, local-first desktop application (Linux, Windows, macOS) designed to read scholarly papers (PDF, Markdown) and annotate them with an append-only, restorable history. Workspaces reside entirely within user-selected local folders, and annotations, synthesized notes, and session logs are stored strictly as plain-text sidecars in `<workspace>/.scribetrace/`. Source documents remain sacred, immutable, and strictly read-only.

In Lean Architecture v3.0:
* **Stage 1 (Zotero Companion & History Recorder):** Ingests annotations, paper metadata, and web links saved via Zotero Connector on click; manages append-only history via per-device log files (`papers/<paper-id>/<device-id>.jsonl`); maintains Markdown notes; exports to Obsidian vaults and W3C JSON-LD.
* **Single Network Destination:** The desktop binary connects **only** to Zotero on `127.0.0.1:23119` on explicit user click; zero outbound internet sockets exist in the application code. Update visibility delegates to the OS default browser opening the fixed release URL.
* **Stage 2 (Integrated Reader & Markup Engine):** Adds native in-app rendering and marking for PDF (via Mozilla PDF.js in an isolated worker) and Markdown behind a validated technical gate. EPUB and arXiv HTML rendering are deferred.

---

## 2. System Architecture & Data-Flow Diagram (DFD v3.0)

The following Mermaid Data-Flow Diagram maps the structural components across defined **Trust Boundaries (TB-1 through TB-8)** for the lean staged MVP:

```mermaid
flowchart TD
    subgraph TB_EXTERNAL ["Trust Boundary 1: Untrusted External Inputs & Sources"]
        ZOT_API["Local Zotero API (127.0.0.1:23119)<br/>(Metadata, Quotes, Titles, Web Links via Connector)"]
        SYNC_IN["Untrusted Sidecars via Sync/Git<br/>(<device-id>.jsonl, note.md, workspace.json)"]
        SRC_DOC["Source Documents (Stage 2: PDF, MD)<br/>[EPUB & HTML Deferred]"]
    end

    subgraph TB_WEBVIEW ["Trust Boundary 2: Presentation Shell (Sandboxed Webview)"]
        UI_TIMELINE["Timeline & What-Changed Viewer<br/>(Escaped Strings / React / DOMPurify)"]
        UI_NOTES["Notes Editor & Markdown Preview<br/>(rehype-sanitize + DOMPurify)"]
        UI_READER["Stage 2 Viewport: PDF.js Worker / MD<br/>(CSP: connect-src 'none')"]
    end

    subgraph TB_IPC ["Trust Boundary 3: Tauri v2 Type-Safe IPC Bridge"]
        IPC_GATEWAY["Tauri IPC Capability Router<br/>(Narrow Typed Commands Allowlist)"]
    end

    subgraph TB_RUST_CORE ["Trust Boundary 4: Memory-Safe Core Engine (Rust Backend)"]
        CORE_ZOTERO["Zotero Broker (crates/scribetrace-zotero)<br/>(Pinned 127.0.0.1:23119, No Redirects, Allowlist)"]
        CORE_LOG["History Keeper (crates/scribetrace-core)<br/>(Per-Device Logs, Union Merge, Sequence Purge)"]
        CORE_NOTES["Notes Store (note.md + revisions)<br/>(Outside-Edit Detection on Open)"]
        CORE_EXP["Export Pipeline (Markdown, W3C JSON-LD)<br/>(Marked Subfolder Overwrite Rules)"]
        CORE_VAL["Validators (crates/scribetrace-format)<br/>(Schema, 1 MB Limit, Identifiers, Bad-Line Skip)"]
        CORE_JAIL["Path Jail Enforcement<br/>(Workspace + Granted Read Folders + Dialog)"]
    end

    subgraph TB_HOST_OS ["Trust Boundary 5: Host Filesystem & System Services"]
        FS_SIDECARS[("Project Folder (.scribetrace/)<br/>papers/<paper-id>/<device-id>.jsonl")]
        FS_APPDATA[("Per-User OS App Storage<br/>(device.json UUID & last_id - NOT synced)")]
        FS_OBSIDIAN[("User-Chosen Obsidian Vault<br/>(scribetrace-export/ subfolder)")]
        SYS_BROWSER["System Default Web Browser<br/>(External Links & Releases Page Dispatcher)"]
    end

    subgraph TB_DISTRIBUTION ["Trust Boundary 6 & 7: Release, Distribution & Infrastructure"]
        GH_RELEASE["GitHub Releases & Actions CI<br/>(SLSA Provenance, Sigstore Signatures, SBOM)"]
        VPS_WEB["Maintainer VPS Docs & Website<br/>(Static Site, Caddy/Nginx TLS, security.txt)"]
    end

    %% External Data Flow
    ZOT_API <==>|Loopback on Click (Broker Only)| CORE_ZOTERO
    SYNC_IN -->|Sync external writes| CORE_LOG
    SRC_DOC -.->|Stage 2 Read-Only Handles| CORE_JAIL

    %% IPC Bridge Flow
    IPC_GATEWAY <==>|Structured JSON-RPC (Tauri Allowlist)| UI_TIMELINE
    IPC_GATEWAY <==>|Structured JSON-RPC (Tauri Allowlist)| UI_NOTES
    IPC_GATEWAY <==>|Structured JSON-RPC (Stage 2)| UI_READER

    %% Core Components Flow
    IPC_GATEWAY --> CORE_ZOTERO
    IPC_GATEWAY --> CORE_LOG
    IPC_GATEWAY --> CORE_NOTES
    IPC_GATEWAY --> CORE_EXP

    CORE_ZOTERO --> CORE_VAL
    CORE_LOG --> CORE_VAL
    CORE_LOG --> CORE_JAIL
    CORE_NOTES --> CORE_JAIL
    CORE_EXP --> CORE_JAIL

    %% Filesystem Flow
    CORE_JAIL <-->|Append / Read per-device logs| FS_SIDECARS
    CORE_LOG <-->|Read / Write local device.json| FS_APPDATA
    CORE_JAIL -->|Bounded Overwrite Export| FS_OBSIDIAN

    %% External Navigation & Update Flow
    UI_TIMELINE -->|Click External Link / zotero://| IPC_GATEWAY
    IPC_GATEWAY -->|Prompt User Confirmation| SYS_BROWSER
    IPC_GATEWAY -->|Check for Updates: Open Release Page| SYS_BROWSER
    GH_RELEASE -->|Scoped CI Deploy| VPS_WEB
```

---

## 3. Trust Boundaries & Threat Actor Profiles (v3.0)

### 3.1 Trust Boundaries Defined

| Boundary ID | Trust Boundary Name | Separation Description | Untrusted Inputs Traversing Boundary |
| :--- | :--- | :--- | :--- |
| **TB-1** | **Untrusted Local Sources & Zotero API** | Local Zotero instance / Host filesystem $\rightarrow$ Core engine | Manipulated Zotero metadata (XSS payloads in titles/notes), web links saved via Zotero Connector, hostile JSON responses, Stage 2 PDF binaries. *(No browser extension; EPUB/HTML deferred)*. |
| **TB-2** | **Hostile Sidecar & Sync Channels** | External sync tools / Git branches $\rightarrow$ Core history keeper | Tampered `<device-id>.jsonl` files, injected script tags, forged sequence numbers, malformed lines, conflicting sync files. |
| **TB-3** | **Webview-to-Core IPC Bridge** | React presentation layer $\rightarrow$ Rust backend core | Compromised webview renderer sending forged IPC commands, path traversal strings, or unauthorized request triggers. |
| **TB-4** | **Filesystem & Path Canonicalization** | Core engine $\rightarrow$ Host operating system filesystem | Relative paths (`../../`), symlink loops, directory traversal outside workspace, escape into ungranted host storage. |
| **TB-5** | **Inter-Process / Protocol Handlers** | OS environment $\rightarrow$ System default browser / Zotero app | Malicious external URLs (`file://`, `javascript:`), manipulated `zotero://` links, manual update release URL dispatch. |
| **TB-6** | **Software Supply Chain** | Upstream registries (crates.io, npm) $\rightarrow$ Developer & CI build | Malicious dependency packages, compromised build dependencies, typo-squatting, tampered CI GitHub Action steps. |
| **TB-7** | **Release Distribution** | Official GitHub repository $\rightarrow$ Package managers & browser downloads | Spoofed release page binaries, DNS tampering against GitHub. *(In-app update client removed)*. |
| **TB-8** | **Maintainer VPS & Website** | Internet $\rightarrow$ Maintainer VPS hosting website & docs | SSH brute-force, web server exploit, site defacement, replacement of published release checksums or signatures. |

---

## 4. STRIDE Threat Analysis by Trust Boundary (v3.0)

Threats are evaluated using qualitative Likelihood ($1\text{--}5$) and Impact ($1\text{--}5$).

---

### 4.1 TB-1: Untrusted Document & Integration Ingestion

| Threat ID | STRIDE Category | Threat Scenario & Attack Mechanism | L | I | Risk Score / Severity | Architectural Control & Mitigation | Mapped Req ID |
| :--- | :--- | :--- | :---: | :---: | :---: | :--- | :--- |
| **THREAT-1.1** | **Elevation of Privilege** | **XSS in Rendered arXiv / Web HTML Documents [Deferred to Stage 2]:** Adversary crafts an arXiv HTML document containing script payloads. | 2 | 5 | **10 (Medium)** | *Deferred to Stage 2.* MVP reads PDF and Markdown only; web pages and arXiv HTML are not rendered directly. | **SEC-3 (Deferred)** |
| **THREAT-1.2** | **Denial of Service** | **EPUB Decompression Bomb (Zip Bomb) [Deferred to Stage 2]:** High-ratio EPUB zip bomb to crash host OS. | 2 | 4 | **8 (Medium)** | *Deferred to Stage 2.* EPUB parsing completely excluded from MVP. | **SEC-4 (Deferred)** |
| **THREAT-1.3** | **Elevation of Privilege** | **PDF Parser Memory Corruption [Stage 2 Scope]:** Crafted PDF triggers memory safety bugs in parsing libraries. | 2 | 5 | **10 (Medium)** | In Stage 1, zero PDF parsing occurs (Zotero marks imported as pure data). In Stage 2, PDF parsing is isolated within Mozilla PDF.js inside an unprivileged worker. | **SEC-5** |
| **THREAT-1.4** | **Tampering / Info Disclosure** | **EPUB XML External Entity (XXE) Injection [Deferred to Stage 2]:** XML external entities referencing local files. | 2 | 4 | **8 (Medium)** | *Deferred to Stage 2.* No XML parsing in Stage 1 MVP. | **SEC-6 (Deferred)** |
| **THREAT-1.5** | **Elevation of Privilege** | **Markdown Polyglot Script Injection:** A malicious Markdown document or note embeds raw HTML or SVG script payloads. | 4 | 4 | **16 (High)** | Parse Markdown into AST using `mdast`/`hast`, pass through `rehype-sanitize`, and sanitize final HTML with `DOMPurify` before DOM injection. | **SEC-7** |

---

### 4.2 TB-2: Hostile Sidecar Storage & Sync Channels (`.scribetrace/`)

| Threat ID | STRIDE Category | Threat Scenario & Attack Mechanism | L | I | Risk Score / Severity | Architectural Control & Mitigation | Mapped Req ID |
| :--- | :--- | :--- | :---: | :---: | :---: | :--- | :--- |
| **THREAT-2.1** | **Elevation of Privilege / Tampering** | **Stored XSS via Annotation Comments & Note Sidecars:** Stored script payloads in `.scribetrace/papers/<id>/<device>.jsonl` or Zotero title/note imports. | 4 | 5 | **20 (Critical)** | Treat all loaded sidecar text and Zotero metadata as untrusted string literals. Disallow executable HTML, escape all strings during React rendering, and sanitize markdown notes via `DOMPurify`. | **SEC-8, SEC-9** |
| **THREAT-2.2** | **Tampering** | **History Reordering via Clock Spoofing:** Adversary injects future timestamps to alter event order or hide mutations. | 3 | 4 | **12 (High)** | Timestamps (`ts`) are strictly display-only metadata; ordering is governed by per-device monotonic sequence counters (`seq`) and set union. Clock drift cannot alter event presence or integrity. | **SEC-10, SEC-11** |
| **THREAT-2.3** | **Denial of Service** | **Sidecar JSON Bomb / Disk Exhaustion via Diagnostic Files:** Massive 500 MB JSON lines or filesystem spam via quarantine writes. | 3 | 3 | **9 (Medium)** | Enforce strict 1 MB line limit; stream line-by-line; skip invalid/corrupt lines on read and display count in UI/CLI; **zero** diagnostic file writes to disk. | **SEC-12, SEC-13** |
| **THREAT-2.4** | **Tampering** | **Path Traversal via Paper & Device Identifiers:** Injected traversal tokens (`../../`) into `<paper-id>` or `<device-id>` paths. | 3 | 5 | **15 (High)** | Strict regex validation (`^[A-Za-z0-9_-]+$`) on paper and device IDs. Canonical path resolution confines all writes within `<workspace>/.scribetrace/`. | **SEC-14, SEC-21** |
| **THREAT-2.5** | **Repudiation / Info Disclosure** | **False History Erasure / Ghost History Liability:** Confidential peer review manuscript history survives or is restored across machines. | 4 | 4 | **16 (High)** | Local files for that paper are deleted immediately. A `purge` event specifying `covers = {device: highest seq seen}` is appended to the local device log; union merge ignores all events with `seq <= covers[device]`. Peer devices hide covered events and show an opt-in delete banner. Honest UI disclosure warns of sync copies. | **SEC-11** |
| **THREAT-2.6** | **Information Disclosure / Tampering** | **Device ID Collisions & Identity Leaks in Synced Folders:** Shared workspace folder sharing an ID across machines; hostnames leaking user identity in Git. | 3 | 3 | **9 (Medium)** | Generate anonymous random UUIDv4 for `device_id` and store it in `device.json` strictly in **per-user OS app storage** (`~/.config/scribetrace/`), never in the synced workspace folder. | **SEC-34** |

---

### 4.3 TB-3: Presentation Shell & Tauri v2 IPC Bridge

| Threat ID | STRIDE Category | Threat Scenario & Attack Mechanism | L | I | Risk Score / Severity | Architectural Control & Mitigation | Mapped Req ID |
| :--- | :--- | :--- | :---: | :---: | :---: | :--- | :--- |
| **THREAT-3.1** | **Elevation of Privilege** | **Arbitrary OS Command Execution via Compromised Webview:** Exploited UI attempts to spawn native commands (`sh`, `powershell`). | 2 | 5 | **10 (Medium)** | Enforce Tauri v2 capability isolation: shell plugin disabled, zero process spawning APIs in `tauri.conf.json`. Webview restricted to strict typed IPC allowlist. | **SEC-15, SEC-16** |
| **THREAT-3.2** | **Information Disclosure** | **Unbounded Filesystem Read via IPC Arguments:** Manipulated IPC commands attempting to read `/home/user/.ssh/id_rsa`. | 3 | 5 | **15 (High)** | Rust backend validates every requested path using `dunce::canonicalize` against the active workspace jail and user-granted read folders (Zotero storage). | **SEC-17** |
| **THREAT-3.3** | **Tampering** | **Arbitrary Host File Overwrite via Export / Save IPC:** Manipulating export calls to overwrite critical host system files or arbitrary files in external Obsidian vaults. | 2 | 5 | **10 (Medium)** | Confine exports to user-selected paths via native OS dialog or dedicated marked subfolder (`scribetrace-export/`). Only overwrite unchanged ScribeTrace-generated files. | **SEC-18, SEC-36** |

---

### 4.4 TB-4: Local Filesystem & Path Traversal

| Threat ID | STRIDE Category | Threat Scenario & Attack Mechanism | L | I | Risk Score / Severity | Architectural Control & Mitigation | Mapped Req ID |
| :--- | :--- | :--- | :---: | :---: | :---: | :--- | :--- |
| **THREAT-4.1** | **Tampering** | **Accidental or Malicious Mutation of Sacred Source Documents [Stage 2 Scope]:** Overwriting original paper binaries. | 2 | 5 | **10 (Medium)** | In Stage 1, PDFs are never opened. In Stage 2, file handles are opened with strictly read-only operating system flags (`read = true, write = false`). | **SEC-19** |
| **THREAT-4.2** | **Information Disclosure / Elevation of Privilege** | **Symlink Traversal Escape:** Symbolic links or NTFS junctions pointing outside the workspace boundary to expose sensitive host files. | 3 | 4 | **12 (High)** | Canonicalize all paths during directory traversal; ignore symlinks pointing outside the workspace or granted directories; break circular symlink loops. | **SEC-20** |
| **THREAT-4.3** | **Denial of Service / Tampering** | **Windows Alternate Data Stream (ADS) & Reserved Device Names:** Filenames with `:` or reserved DOS device names (`CON`, `PRN`, `NUL`). | 3 | 3 | **9 (Medium)** | Validate all filenames against reserved DOS device names. Strip Windows Alternate Data Stream colons (`:`) from relative paths. Reject null bytes (`\0`). | **SEC-21** |
| **THREAT-4.4** | **Information Disclosure** | **Author Username Leakage:** Application harvests OS login username or environment variables and embeds them into public Git sidecars. | 4 | 2 | **8 (Medium)** | Default author label is hardcoded to literal string `'Me'`. No environment username harvesting without explicit user configuration. | **SEC-35** |

---

### 4.5 TB-5: Inter-Process Communication & External Links

| Threat ID | STRIDE Category | Threat Scenario & Attack Mechanism | L | I | Risk Score / Severity | Architectural Control & Mitigation | Mapped Req ID |
| :--- | :--- | :--- | :---: | :---: | :---: | :--- | :--- |
| **THREAT-5.1** | **Elevation of Privilege** | **Malicious Deep Link Parameter Injection (`scribetrace://`) [Deferred to Stage 2]:** Argument injection via custom URL protocol. | 2 | 4 | **8 (Medium)** | *Deferred to Stage 2.* Stage 1 registers no custom URL scheme; links point directly to `zotero://`. | **SEC-22 (Deferred)** |
| **THREAT-5.2** | **Information Disclosure / Phishing** | **Silent External Navigation & Link Hijacking:** Embedded web or `zotero://` links triggering silent external actions. | 4 | 4 | **16 (High)** | Intercept all external hyperlinks and `zotero://` URIs. Present native user confirmation dialog displaying full unshortened destination URI before dispatching to default system handler. | **SEC-23, SEC-24** |

---

### 4.6 TB-6: Supply Chain & Build Pipeline

| Threat ID | STRIDE Category | Threat Scenario & Attack Mechanism | L | I | Risk Score / Severity | Architectural Control & Mitigation | Mapped Req ID |
| :--- | :--- | :--- | :---: | :---: | :---: | :--- | :--- |
| **THREAT-6.1** | **Tampering / Elevation of Privilege** | **Compromised Upstream Dependencies (crates.io / npm):** Compromised dependency injecting backdoors or exfiltrating keys. | 3 | 5 | **15 (High)** | Commit strict lockfiles (`Cargo.lock`, `pnpm-lock.yaml`). Run `cargo-audit` and `pnpm audit` in CI. Enforce `--ignore-scripts`. Generate CycloneDX SBOM. | **SEC-25, SEC-26** |
| **THREAT-6.2** | **Tampering** | **GitHub Actions Workflow Hijacking:** Unpinned actions or exposed secrets modifying release binaries. | 2 | 5 | **10 (Medium)** | Pin all GitHub Actions to 40-character immutable commit SHAs. Enforce `permissions: contents: read`. Require 2FA and signed commits for maintainers. | **SEC-27** |

---

### 4.7 TB-7: Release Distribution & Browser Update Channel

| Threat ID | STRIDE Category | Threat Scenario & Attack Mechanism | L | I | Risk Score / Severity | Architectural Control & Mitigation | Mapped Req ID |
| :--- | :--- | :--- | :---: | :---: | :---: | :--- | :--- |
| **THREAT-7.1** | **Tampering / Elevation of Privilege** | **Forged Update Binary Injection & Stale Version Vulnerabilities:** In-place updater hijacked to execute rogue binary. | 1 | 5 | **5 (Low)** | Entire in-place updater and in-app network fetching code removed from the application. "Check for Updates" menu item opens the fixed, hardcoded GitHub release page in the system browser via `tauri-plugin-opener`. Zero updater RCE surface. | **SEC-28** |
| **THREAT-7.2** | **Repudiation / Tampering** | **Release Binary Spoofing on GitHub Releases:** Attackers replace binaries on release page with Trojanized versions. | 2 | 5 | **10 (Medium)** | Generate SLSA Level 3 provenance attestations using GitHub Artifact Attestations and Sigstore. Publish `SHA256SUMS` signed with maintainer's detached GPG/Minisign key. | **SEC-29** |

---

### 4.8 TB-8: Maintainer VPS & Integration Loopback Hardening

| Threat ID | STRIDE Category | Threat Scenario & Attack Mechanism | L | I | Risk Score / Severity | Architectural Control & Mitigation | Mapped Req ID |
| :--- | :--- | :--- | :---: | :---: | :---: | :--- | :--- |
| **THREAT-8.1** | **Elevation of Privilege / Tampering** | **Maintainer VPS Host Compromise via SSH / Network:** Attackers compromise documentation VPS and replace release links. | 2 | 5 | **10 (Medium)** | Harden VPS: non-root deploy user, SSH key authentication only, UFW firewall (ports 22, 80, 443), automated security patching (`unattended-upgrades`), fail2ban. | **SEC-30** |
| **THREAT-8.2** | **Information Disclosure / Tampering** | **Insecure Website Transport & Missing Security Headers:** Eavesdropping or iframe clickjacking on official site. | 3 | 3 | **9 (Medium)** | Enforce TLS 1.3 via Let's Encrypt, HSTS preload (`max-age=63072000; includeSubDomains; preload`), and strict security headers (CSP, nosniff, Referrer-Policy). | **SEC-31** |
| **THREAT-8.3** | **Repudiation** | **Uncoordinated Vulnerability Discovery:** Public disclosure of zero-days due to missing security contact info. | 3 | 3 | **9 (Medium)** | Publish standard `/.well-known/security.txt` conforming to RFC 9116 with GitHub Private Vulnerability Reporting URL and PGP key. | **SEC-32** |
| **THREAT-8.4** | **Information Disclosure / Tampering** | **Local Loopback SSRF & Port Scanning via Zotero Interface:** Malicious script abuses loopback exception to probe internal services (Redis, Ollama, dev servers) on `127.0.0.1`. | 3 | 4 | **12 (High)** | Webview CSP enforces `connect-src 'none'`. All loopback communication handled exclusively by Rust core broker (`crates/scribetrace-zotero`): pinned to `127.0.0.1:23119`, no DNS lookup, no HTTP redirects, GET-only path allowlist (`/api/users/0/items*`), required `X-Zotero-Version` response header, 15 MB payload cap, 5.0 s timeout. | **SEC-33** |

---

## 5. Comprehensive Risk Assessment Matrix (v3.0)

All 26 evaluated threats are mapped across the Risk Matrix below:

```
+----------------------------------------------------------------------------------------------------+
|                                    PALIMPSEST RISK RATING MATRIX (v3.0)                            |
+----------------------+--------------------+--------------------+--------------------+--------------+
| Likelihood \ Impact  | Low (1-2)          | Medium (3)         | High (4)           | Critical (5) |
+----------------------+--------------------+--------------------+--------------------+--------------+
| 5 (Almost Certain)   |                    |                    |                    |              |
| 4 (Likely)           | THREAT-4.4         |                    | THREAT-1.5, 2.5    | THREAT-2.1   |
|                      |                    |                    | THREAT-5.2         |              |
| 3 (Moderate)         |                    | THREAT-2.3, 2.6    | THREAT-2.2, 4.2    | THREAT-2.4   |
|                      |                    | THREAT-4.3, 8.2    | THREAT-8.4         | THREAT-3.2   |
|                      |                    | THREAT-8.3         |                    | THREAT-6.1   |
| 2 (Unlikely)         |                    | THREAT-1.2 (Def)   | THREAT-1.4 (Def)   | THREAT-1.1 (Def)
|                      |                    |                    | THREAT-5.1 (Def)   | THREAT-1.3   |
|                      |                    |                    |                    | THREAT-3.1   |
|                      |                    |                    |                    | THREAT-3.3   |
|                      |                    |                    |                    | THREAT-4.1   |
|                      |                    |                    |                    | THREAT-6.2   |
|                      |                    |                    |                    | THREAT-7.2   |
|                      |                    |                    |                    | THREAT-8.1   |
| 1 (Rare)             |                    |                    |                    | THREAT-7.1   |
+----------------------+--------------------+--------------------+--------------------+--------------+
```

---

## 6. Changelog & Architecture v3.0 Alignment

### Version 3.0 (2026-11-05) - Lean Architecture Alignment
* **Single Network Destination:** Network perimeter reduced strictly to local loopback `127.0.0.1:23119` (Zotero) on explicit click.
* **Update Model (THREAT-7.1):** Removed all in-app HTTP update code and in-app update parsing. Updates are browsed via the OS default browser opening the fixed GitHub release page, lowering THREAT-7.1 risk to Low ($1 \times 5 = 5$).
* **Link Capture Analysis (ADR 0013):** Formally validated that web link captures are delegated to Zotero Connector; ScribeTrace requires no browser extension, no web scraping engine, and no custom URL protocol handlers in Stage 1.
* **Sequence-Based Purge (THREAT-2.5):** Purge evaluation updated to exact sequence vector `covers = {device: highest seq seen}`, ensuring pure-function evaluation immune to clock skew.
* **Zero Disk-Flooding Amplification (THREAT-2.3):** Eliminated diagnostic file writes to disk; malformed lines are skipped in-memory and reported via UI/CLI counts.
