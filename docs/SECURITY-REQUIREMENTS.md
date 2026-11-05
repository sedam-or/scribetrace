# ScribeTrace: Security Requirements Specification (v3.0, Lean)

**Document Version:** 3.0  
**Status:** Approved for Lean Architecture v3.0 & QA Implementation  
**Date:** 2026-11-05  
**Product Name:** ScribeTrace  
**Standard Syntax:** EARS (Easy Approach to Requirements Syntax)  
**Repository Path:** `docs/SECURITY-REQUIREMENTS.md`  

---

## 1. Introduction & Methodology

This specification defines the formal, testable security requirements for **ScribeTrace**, aligned with `docs/ARCHITECTURE.md` (v3.0, Lean) and ADRs 0006–0013.

### 1.1 Requirements Syntax (EARS)
All requirements follow **Easy Approach to Requirements Syntax (EARS)** patterns:
* **Ubiquitous:** `The system SHALL <response>`
* **Event-driven:** `WHEN <trigger> THEN the system SHALL <response>`
* **State-driven:** `WHILE <state> the system SHALL <response>`
* **Unwanted Behavior:** `IF <condition/fault> THEN the system SHALL <response>`
* **Optional / Feature-driven:** `WHERE <feature> is enabled the system SHALL <response>`

### 1.2 Enforcement Tier Taxonomy
Every requirement is explicitly tagged with its enforcement mechanism:
* **`[Enforced in Code]`:** Compiled into and verified by the Rust backend core, format validators, or webview UI.
* **`[Process]`:** Governed by CI/CD, dependency lockfiles, build provenance attestations, repository workflows, or VPS server configurations.
* **`[Deferred]`:** Belongs to Stage 2 (in-app reader), reflowable formats (EPUB/HTML), or post-v1 capabilities.

---

## 2. Security Requirements Specification (SEC-1 through SEC-36)

### 2.1 Principle 1: Secure & Deny by Default (Network, CSP & Navigation)

#### SEC-1: Single-Destination Loopback Network Rule `[Enforced in Code]`
* **EARS Pattern (State-driven):** WHILE the application is running the system SHALL NOT initiate outbound internet connections, bind external listening sockets, or issue DNS lookup requests, EXCEPT when the user explicitly triggers an action, limited strictly to a local loopback HTTP request to Zotero on `127.0.0.1:23119` via the core broker.
* **Threat Mitigation:** `THREAT-3.1`, `THREAT-5.2`, `THREAT-8.4` (Data exfiltration, telemetry, SSRF)
* **Verification Method:** Network namespace test (`unshare -n`); automated tests assert zero dropped packets, zero DNS queries, and exactly one loopback connection to `127.0.0.1:23119` on manual Zotero import click.

#### SEC-2: Strict Webview Content Security Policy (CSP) `[Enforced in Code]`
* **EARS Pattern (Ubiquitous):** The system SHALL enforce a Content Security Policy on all internal webview contexts restricting script execution to local pre-compiled bundles: `default-src 'none'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob:; font-src 'self' data:; connect-src 'none'; frame-src 'self';`.
* **Threat Mitigation:** `THREAT-1.1`, `THREAT-2.1` (Stored and Reflected XSS)
* **Verification Method:** Playwright DOM test asserting dynamic evaluation (`eval()`), inline `<script>`, and remote script tags throw CSP errors.

#### SEC-3: Sandboxed Isolated Viewport for arXiv & HTML Documents `[Deferred - Stage 2]`
* **EARS Pattern (Event-driven):** WHEN the user opens an HTML or arXiv paper THEN the system SHALL render the document inside a sandboxed viewport (`<iframe sandbox="allow-same-origin">` with `allow-scripts` omitted) with an injected document CSP: `default-src 'none'; script-src 'none'; connect-src 'none'; form-action 'none'; style-src 'unsafe-inline'; img-src data: blob:;`.
* **Threat Mitigation:** `THREAT-1.1` (Untrusted HTML script injection)
* **Verification Method:** Penetration Test (Deferred to Stage 2).

#### SEC-4: Decompression Bomb Safeguards for EPUB Archives `[Deferred - Stage 2]`
* **EARS Pattern (Unwanted Behavior):** IF an EPUB archive exceeds an uncompressed size of $200\text{ MB}$, a single file entry size of $50\text{ MB}$, a compression ratio of $10:1$, or $10,000$ entries THEN the system SHALL terminate decompression, quarantine the document, and log a non-modal warning.
* **Threat Mitigation:** `THREAT-1.2` (Zip bomb / Denial of Service)
* **Verification Method:** Unit & Chaos Test (Deferred to Stage 2).

#### SEC-5: PDF Parser Isolation `[Deferred - Stage 2]`
* **EARS Pattern (Ubiquitous):** The system SHALL isolate PDF parsing within a sandboxed worker context (Mozilla PDF.js in webview or isolated Rust process) such that a panic or crash during PDF parsing does not terminate the host core engine or corrupt state.
* **Threat Mitigation:** `THREAT-1.3` (PDF memory corruption / Denial of Service)
* **Verification Method:** Chaos / Fuzz Test feeding malformed PDF corpus into worker; assert worker crashes recover gracefully.

#### SEC-6: XML External Entity (XXE) Prevention `[Deferred - Stage 2]`
* **EARS Pattern (Ubiquitous):** The system SHALL parse all EPUB, SVG, and XML document metadata with DTD external entity resolution and external parameter entities strictly disabled.
* **Threat Mitigation:** `THREAT-1.4` (Local file disclosure via XXE)
* **Verification Method:** Unit Test (Deferred to Stage 2).

#### SEC-7: Markdown Sanitization Pipeline `[Enforced in Code]`
* **EARS Pattern (Event-driven):** WHEN rendering Markdown content THEN the system SHALL parse the document into an AST (`mdast`/`hast`), pass the AST through a strict schema sanitizer (`rehype-sanitize`), and process final HTML through `DOMPurify` before insertion into the DOM tree.
* **Threat Mitigation:** `THREAT-1.5` (Markdown script injection and HTML polyglots)
* **Verification Method:** Automated test parsing 50 markdown polyglot payloads; assert zero raw `<script>`, `<iframe>`, `<object>`, or `<embed>` tags survive to the rendered DOM.

---

### 2.2 Principle 2: Least Privilege & Data Boundary Validation

#### SEC-8: Sanitization of Untrusted Sidecar Strings & Zotero Metadata `[Enforced in Code]`
* **EARS Pattern (Event-driven):** WHEN loading annotation comments, user tags, author labels, or Zotero-imported metadata from `.scribetrace/` sidecar files or the local API THEN the system SHALL treat all loaded text as untrusted string literals and SHALL NOT interpret loaded fields as executable markup or HTML.
* **Threat Mitigation:** `THREAT-2.1`, `THREAT-8.4` (Stored XSS via sync repositories or manipulated Zotero items)
* **Verification Method:** Integration test loading crafted `.jsonl` files and Zotero fixtures with script tags; assert strings render purely as escaped text.

#### SEC-9: Notes Markdown Isolation `[Enforced in Code]`
* **EARS Pattern (State-driven):** WHILE the Side Notes editor displays synthesis notes THEN the system SHALL isolate rendered note previews using `DOMPurify` with an allowlist limited to standard formatting tags and attribute allowlists prohibiting event handlers.
* **Threat Mitigation:** `THREAT-2.1` (Stored XSS in notes)
* **Verification Method:** Unit test injecting arbitrary attributes (`onerror`, `onload`); verify disallowed attributes are stripped.

#### SEC-10: Sequence Integrity & Clock-Independent Ordering `[Enforced in Code]`
* **EARS Pattern (Ubiquitous):** The system SHALL order events within each device file using a strictly monotonically increasing per-device sequence counter (`seq`), SHALL treat timestamps (`ts`) as display-only metadata, and SHALL NOT allow modified system clocks to alter event state transitions or omit historical events during union merge.
* **Threat Mitigation:** `THREAT-2.2` (History reordering via clock manipulation)
* **Verification Method:** Property test permuting wall-clock timestamps across simulated device streams; assert union merge produces identical state.

#### SEC-11: Non-Destructive Deletions and Sequence-Covered Forget Semantics `[Enforced in Code]`
* **EARS Pattern (Event-driven):**  
  1. WHEN an item is deleted in the UI THEN the system SHALL append an `action: "delete"` tombstone to the local device log (`<device-id>.jsonl`) and preserve all historical events in the log.  
  2. WHEN the user confirms "Forget Paper History" THEN the system SHALL unlink all local history files for that paper on the host machine, append a `purge` event specifying `covers = {device: highest seq seen}` to the local log, and exclude all events where `seq <= covers[device]` during union read across all devices.  
  3. WHEN opening a paper where a purge event was appended by another device THEN the system SHALL hide covered events, display a non-modal banner offering "Delete hidden files", and warn that external sync copies (Git/cloud) may retain files.
* **Threat Mitigation:** `THREAT-2.2`, `THREAT-2.5` (Silent history erasure, accidental retention of confidential manuscripts)
* **Verification Method:** Unit & Property test verifying union merge ignores events covered by `seq <= covers[device]`; assert local files unlinked and UI banner displayed.

#### SEC-12: Maximum Line-Length Bounds on JSONL Sidecars `[Enforced in Code]`
* **EARS Pattern (Unwanted Behavior):** IF an individual line in a `.scribetrace/` sidecar file exceeds $1,048,576\text{ bytes}$ ($1\text{ MB}$) THEN the system SHALL abort processing that line, advance the reader to the next newline, and increment the skipped invalid line counter.
* **Threat Mitigation:** `THREAT-2.3` (JSON bomb / Memory exhaustion DoS)
* **Verification Method:** Stream 20 MB unbroken line into sidecar reader; assert reader skips line without memory spike and processes subsequent valid lines.

#### SEC-13: Fault-Tolerant Bad Line Skipping Without Disk Amplification `[Enforced in Code]`
* **EARS Pattern (Unwanted Behavior):** IF a sidecar line contains malformed JSON syntax or non-UTF-8 bytes THEN the system SHALL skip the invalid line during read operations, increment a session invalid line counter displayed in the UI status bar and CLI `check`, and SHALL NOT write diagnostic files to disk.
* **Threat Mitigation:** `THREAT-2.3` (Crash via malformed input & disk spam amplification)
* **Verification Method:** Fault injection test with random bytes in a JSONL file; assert valid lines load, file bytes remain unchanged, and status bar shows 1 skipped line.

#### SEC-14: Sidecar Path Resolution Jail `[Enforced in Code]`
* **EARS Pattern (Ubiquitous):** The system SHALL validate that all sidecar paths resolve within the `<workspace>/.scribetrace/` directory using canonical path resolution and SHALL reject any path containing directory traversal tokens (`..`), null bytes, or symbolic link redirections outside the workspace.
* **Threat Mitigation:** `THREAT-2.4` (Path traversal via metadata files)
* **Verification Method:** Pass `../../etc/passwd` to internal sidecar path resolution routines; assert `Err(PathEscape)` returned.

---

### 2.3 Principle 3: IPC Capability Hardening & Command Confinement

#### SEC-15: Zero Native Shell Execution via IPC `[Enforced in Code]`
* **EARS Pattern (Ubiquitous):** The Tauri application configuration (`tauri.conf.json`) SHALL NOT enable the `shell` plugin, raw command execution APIs, or process spawning capabilities.
* **Threat Mitigation:** `THREAT-3.1` (RCE via compromised webview)
* **Verification Method:** CI static audit parsing `tauri.conf.json`; assert shell capabilities are completely omitted from permissions.

#### SEC-16: Strict Typed IPC Allowlist `[Enforced in Code]`
* **EARS Pattern (Ubiquitous):** The system SHALL route all webview-to-core requests through an explicit, typed Tauri v2 IPC allowlist; any IPC message matching an unregistered command name or invalid payload schema SHALL be dropped and logged.
* **Threat Mitigation:** `THREAT-3.1` (Unauthorized IPC dispatch)
* **Verification Method:** Integration test invoking unregistered command (`window.__TAURI__.invoke('system_exec')`); assert promise rejection.

#### SEC-17: Extended Path Jail Enforcement (Workspace, Granted Read Folders, and Export Destination) `[Enforced in Code]`
* **EARS Pattern (Event-driven):** WHEN an IPC command requests file access by path or identifier THEN the Rust core SHALL verify using `dunce::canonicalize` that the target path resides strictly within: (1) the active workspace directory, (2) an explicit user-granted read-only folder, or (3) a user-selected export destination chosen via native OS dialog, and SHALL reject all paths outside these boundaries.
* **Threat Mitigation:** `THREAT-3.2`, `THREAT-3.3` (Arbitrary file read / write via IPC)
* **Verification Method:** Integration test requesting unauthorized host paths; assert core returns `PermissionDenied`.

#### SEC-18: Restricted Write Paths for Export Operations `[Enforced in Code]`
* **EARS Pattern (Event-driven):** WHEN exporting document data (Markdown, W3C JSON-LD) THEN the system SHALL write files exclusively to paths explicitly selected by the user via native OS Save File dialog or within `<workspace>/.scribetrace/`.
* **Threat Mitigation:** `THREAT-3.3` (Arbitrary file write / overwrite)
* **Verification Method:** Integration test attempting export to system directories without dialog interaction; assert backend denies write.

---

### 2.4 Principle 4: Filesystem Safety & Document Integrity

#### SEC-19: Strict Read-Only File Handles for Source Documents `[Deferred - Stage 2]`
* **EARS Pattern (Ubiquitous):** The system SHALL open all source documents (PDF, Markdown) using read-only operating system file descriptors (`read = true, write = false`) and SHALL NOT acquire write locks or leases on source files.
* **Threat Mitigation:** `THREAT-4.1` (Mutation of original source documents)
* **Verification Method:** System call trace (`strace` asserting `O_RDONLY` flags on source documents).

#### SEC-20: Symlink Escape Prevention in Workspace Scanner & Export `[Enforced in Code]`
* **EARS Pattern (Event-driven):** WHEN scanning workspace folders or exporting files THEN the system SHALL resolve and canonicalize symbolic links, and SHALL ignore any symlink whose target resolves outside the canonical workspace boundary or introduces a circular loop.
* **Threat Mitigation:** `THREAT-4.2` (Symlink traversal outside project)
* **Verification Method:** Integration test with symlink pointing to `/tmp`; assert zero external files appear in catalog.

#### SEC-21: Filename & Identifier Sanitization `[Enforced in Code]`
* **EARS Pattern (Ubiquitous):** The system SHALL sanitize all document relative paths, sidecar filenames, `<paper-id>`, and `<device-id>` strings, enforcing characters matching `^[A-Za-z0-9_-]+$`, and rejecting Windows Alternate Data Streams (`:`), null bytes (`\0`), and reserved DOS device names.
* **Threat Mitigation:** `THREAT-4.3` (Filesystem driver exploitation & ADS bypass)
* **Verification Method:** Unit test passing 30 reserved/illegal path strings to filename validator; assert 100% rejected or normalized.

---

### 2.5 Principle 5: Inter-Process Communication & External Navigation

#### SEC-22: Strict Parsing of Custom Protocol Deep Links `[Deferred - Stage 2]`
* **EARS Pattern (Event-driven):** WHERE the application registers a `scribetrace://` protocol URL handler the system SHALL validate the URL against a strict schema: host SHALL be `doc`, path SHALL be empty, and query parameters SHALL contain only valid alphanumeric content hashes and UUIDv4 IDs.
* **Threat Mitigation:** `THREAT-5.1` (Argument injection via deep links)
* **Verification Method:** Unit Test (Deferred to Stage 2).

#### SEC-23: Interception of External Links & User Confirmation `[Enforced in Code]`
* **EARS Pattern (Event-driven):** WHEN the user clicks a hyperlink referencing an external web URL (`http://` or `https://`) or a `zotero://` URI THEN the system SHALL display a native confirmation dialog presenting the complete, unshortened destination URI and requesting explicit user authorization.
* **Threat Mitigation:** `THREAT-5.2` (Silent link hijacking and phishing)
* **Verification Method:** E2E test clicking external link in viewport; assert navigation blocked until confirmation button is clicked.

#### SEC-24: External Dispatch Exclusively to System Default Handler `[Enforced in Code]`
* **EARS Pattern (Event-driven):** WHEN an external URI is approved by the user THEN the system SHALL dispatch the URI strictly to the operating system's default handler via `tauri-plugin-opener` (restricting allowed schemes to `http`, `https`, and `zotero`) and SHALL NEVER render external web pages inside the ScribeTrace webview.
* **Threat Mitigation:** `THREAT-5.2` (Webview escape into external browsing)
* **Verification Method:** Integration test verifying URL dispatch launches external process; assert webview origin remains unchanged.

---

### 2.6 Principle 6: Supply Chain Hardening & Build Integrity

#### SEC-25: Deterministic Dependency Lockfiles & Auditing `[Process]`
* **EARS Pattern (Ubiquitous):** All application dependencies SHALL be pinned with cryptographic hashes in version-controlled lockfiles (`Cargo.lock`, `pnpm-lock.yaml`), and CI SHALL execute `cargo-audit` and `pnpm audit` on every pull request, failing on any advisory with CVSS score $\ge 7.0$.
* **Threat Mitigation:** `THREAT-6.1` (Compromised dependencies)
* **Verification Method:** CI gate triggering on PR with known vulnerable dependency; assert audit job blocks merge.

#### SEC-26: Software Bill of Materials (SBOM) Generation `[Process]`
* **EARS Pattern (Event-driven):** WHEN creating a release build THEN the system build pipeline SHALL generate a complete Software Bill of Materials (SBOM) in CycloneDX JSON format covering all Rust and JavaScript dependencies.
* **Threat Mitigation:** `THREAT-6.1` (Supply chain visibility)
* **Verification Method:** Verify release artifacts contain `bom.cyclonedx.json` conforming to CycloneDX 1.5 schema.

#### SEC-27: Pinned GitHub Actions & Least-Privilege CI Tokens `[Process]`
* **EARS Pattern (Ubiquitous):** All GitHub Actions workflows SHALL reference third-party actions by full 40-character commit SHAs, SHALL configure top-level `permissions: contents: read`, and SHALL require 2FA and signed commits for all maintainers.
* **Threat Mitigation:** `THREAT-6.2` (CI workflow tampering)
* **Verification Method:** Linter gate (`zizmor` or `actionlint`) verifying zero mutable action tags and minimum token scope.

---

### 2.7 Principle 7: Release Distribution & Browser-Based Update Visibility

#### SEC-28: External Browser Dispatch for Update Visibility `[Enforced in Code]`
* **EARS Pattern (Event-driven):** WHEN the user clicks "Check for Updates" in the application menu THEN the system SHALL display the currently installed version number and dispatch the user's default system browser via `tauri-plugin-opener` to the fixed, compiled-in release page URL (`https://github.com/sedam-or/scribetrace/releases`), and SHALL NOT execute in-app HTTP requests, run update routines, or parse remote version data.
* **Threat Mitigation:** `THREAT-7.1` (Updater hijacking & RCE)
* **Verification Method:** Integration test verifying menu action opens system browser with hardcoded release URL; assert zero in-app network calls.

#### SEC-29: SLSA Level 3 Provenance & Checksum Signatures `[Process]`
* **EARS Pattern (Event-driven):** WHEN compiling official release binaries THEN the release pipeline SHALL generate SLSA Level 3 provenance attestations using Sigstore/GitHub Artifact Attestations, and SHALL publish a `SHA256SUMS` file signed with maintainer detached signature.
* **Threat Mitigation:** `THREAT-7.2` (Trojanized release assets)
* **Verification Method:** Automated script verifying signatures and SLSA attestations using `gh attestation verify`.

---

### 2.8 Principle 8: Website & Maintainer Infrastructure Hardening

#### SEC-30: Hardened VPS Bastion & Deployment Pipeline `[Process]`
* **EARS Pattern (Ubiquitous):** The maintainer's documentation VPS SHALL operate with non-root deploy accounts, SSH key-only authentication, host firewall (UFW) permitting only ports 22, 80, 443, automated security patching (`unattended-upgrades`), and `fail2ban`.
* **Threat Mitigation:** `THREAT-8.1` (VPS server compromise)
* **Verification Method:** Configuration audit inspecting `/etc/ssh/sshd_config`, `ufw status`, and `fail2ban-client status`.

#### SEC-31: Mandatory TLS & HTTP Security Headers on Website `[Process]`
* **EARS Pattern (Ubiquitous):** The documentation web server SHALL enforce HTTPS with TLS 1.3, HSTS (`max-age=63072000; includeSubDomains; preload`), and strict security headers (`Content-Security-Policy`, `X-Content-Type-Options: nosniff`, `Referrer-Policy`, `Permissions-Policy`).
* **Threat Mitigation:** `THREAT-8.2` (Transport eavesdropping & clickjacking)
* **Verification Method:** DAST header audit asserting A+ grade on Mozilla Observatory scan.

#### SEC-32: Coordinated Security Disclosure Point (`security.txt`) `[Process]`
* **EARS Pattern (Ubiquitous):** The website and repository SHALL publish a standardized `/.well-known/security.txt` conforming to RFC 9116, specifying GitHub Private Vulnerability Reporting URL, contact email, PGP key, and disclosure policy.
* **Threat Mitigation:** `THREAT-8.3` (Uncoordinated vulnerability disclosure)
* **Verification Method:** Automated HTTP test validating RFC 9116 syntax.

---

### 2.9 Principle 9: Companion Integration & Device Privacy Hardening

#### SEC-33: Zotero Broker Request Hardening & SSRF Prevention `[Enforced in Code]`
* **EARS Pattern (State-driven):** WHILE communicating with local Zotero the system SHALL route all requests exclusively through the Rust core Zotero broker (`crates/scribetrace-zotero/`) with hardcoded loopback address `127.0.0.1:23119`, SHALL NOT perform DNS lookups, SHALL disable HTTP redirects, SHALL restrict requests to GET operations on `/api/users/0/items*`, SHALL require valid `X-Zotero-Version` headers, and SHALL enforce a response body limit of $15\text{ MB}$ and $5.0\text{ s}$ timeout.
* **Threat Mitigation:** `THREAT-8.4` (SSRF, host port scanning, rogue local service probing)
* **Verification Method:** Unit & Integration test with mock HTTP server verifying non-allowlisted ports, paths, redirects, and oversized payloads are rejected.

#### SEC-34: Anonymous Device Identification & Outer-Storage Isolation `[Enforced in Code]`
* **EARS Pattern (Ubiquitous):** The system SHALL generate an anonymous random UUIDv4 for each installation's `device_id`, SHALL store this identifier in `device.json` exclusively within the host operating system's per-user application storage directory, and SHALL NOT write the device identifier into the shared workspace directory or derive it from hardware, hostname, or system usernames.
* **Threat Mitigation:** `THREAT-2.6`, `THREAT-4.4` (Collaborator identity leakage and multi-device ID collision in synced folders)
* **Verification Method:** Filesystem audit asserting `device_id` is a valid UUIDv4 in OS app storage outside `<workspace>/.scribetrace/`.

#### SEC-35: Default Author Identity Privacy `[Enforced in Code]`
* **EARS Pattern (Ubiquitous):** The system SHALL configure the default author display name in all newly created annotation events to the literal string `'Me'`, and SHALL NOT default to, read, or populate the author field with host OS usernames or environment variables without explicit user configuration.
* **Threat Mitigation:** `THREAT-4.4` (Metadata leakage in collaborative Git repositories)
* **Verification Method:** Unit test asserting new events default to `author: "Me"`.

#### SEC-36: Safe Bounded Obsidian Vault Export `[Enforced in Code]`
* **EARS Pattern (Event-driven):** WHEN exporting notes and annotations to an Obsidian vault THEN the system SHALL write exclusively into a user-selected folder within a dedicated, marked subfolder (`scribetrace-export/`), and SHALL ONLY overwrite files previously generated by ScribeTrace bearing the marker line (`# Generated by ScribeTrace`) whose contents have not been modified by the user.
* **Threat Mitigation:** `THREAT-3.3` (Arbitrary file overwrite & sync loop hazards in external vaults)
* **Verification Method:** Integration test verifying export creates marked subfolder and refuses to overwrite user-modified or unmarked files.

---

## 3. Requirement Traceability Matrix (MBSE Compliance)

| Req ID | Security Principle | Enforcement Tier | Threat Mitigated | Architecture Subsystem (`satisfiedBy`) | Verification Strategy (`verifiedBy`) |
| :--- | :--- | :---: | :--- | :--- | :--- |
| **SEC-1** | 1. Secure by Default | `[Enforced in Code]` | `THREAT-3.1, 5.2, 8.4` | Core Network Broker / Webview CSP | Network socket monitor (`unshare -n` / loopback-only audit) |
| **SEC-2** | 1. Secure by Default | `[Enforced in Code]` | `THREAT-1.1, 2.1` | Tauri Webview Shell (`index.html`) | DOM CSP inspection / Playwright test blocking `eval()` |
| **SEC-3** | 1. Secure by Default | `[Deferred]` | `THREAT-1.1` | Deferred (Stage 2) | Penetration test (Deferred to Stage 2) |
| **SEC-4** | 3. Parser Isolation | `[Deferred]` | `THREAT-1.2` | Deferred (Stage 2) | Zip bomb extraction test (Deferred to Stage 2) |
| **SEC-5** | 3. Parser Isolation | `[Deferred]` | `THREAT-1.3` | `app/web/src/reader/` (Stage 2) | Fuzz testing with corrupted PDF corpus |
| **SEC-6** | 4. Boundary Validation | `[Deferred]` | `THREAT-1.4` | Deferred (Stage 2) | Unit test with DTD entity injection (Deferred to Stage 2) |
| **SEC-7** | 4. Boundary Validation | `[Enforced in Code]` | `THREAT-1.5` | `scribetrace-core::notes` | Unit test verifying `rehype-sanitize` + `DOMPurify` pipeline |
| **SEC-8** | 4. Boundary Validation | `[Enforced in Code]` | `THREAT-2.1, 8.4` | `scribetrace-format` | Integration test loading XSS strings in comment/title fields |
| **SEC-9** | 4. Boundary Validation | `[Enforced in Code]` | `THREAT-2.1` | `app/web/src/notes/` | Unit test asserting DOMPurify attribute allowlist enforcement |
| **SEC-10**| 6. Data Integrity | `[Enforced in Code]` | `THREAT-2.2` | `scribetrace-format` | Property test verifying sequence & clock independence |
| **SEC-11**| 6. Data Integrity | `[Enforced in Code]` | `THREAT-2.2, 2.5` | `scribetrace-core::domain` | Property test verifying `covers` sequence purge rule |
| **SEC-12**| 3. Parser Isolation | `[Enforced in Code]` | `THREAT-2.3` | `scribetrace-format` | Unit test streaming 20 MB line; asserting 1 MB truncation guard |
| **SEC-13**| 4. Boundary Validation | `[Enforced in Code]` | `THREAT-2.3` | `scribetrace-core::adapters` | Fault injection test verifying bad line skip without disk spam |
| **SEC-14**| 2. Least Privilege | `[Enforced in Code]` | `THREAT-2.4` | `scribetrace-format` | Path assertion test rejecting `../` traversal in identifiers |
| **SEC-15**| 2. Least Privilege | `[Enforced in Code]` | `THREAT-3.1` | `app/src-tauri/tauri.conf.json` | Static config audit asserting zero shell capabilities in Tauri v2 |
| **SEC-16**| 2. Least Privilege | `[Enforced in Code]` | `THREAT-3.1` | `app/src-tauri/src/ipc/` | Integration test invoking unregistered commands; asserting rejection |
| **SEC-17**| 2. Least Privilege | `[Enforced in Code]` | `THREAT-3.2, 3.3` | `scribetrace-core::adapters` | Integration test verifying jail across workspace and granted folders |
| **SEC-18**| 2. Least Privilege | `[Enforced in Code]` | `THREAT-3.3` | `scribetrace-core::adapters` | Integration test verifying writes restricted to dialog destinations |
| **SEC-19**| 6. Data Integrity | `[Deferred]` | `THREAT-4.1` | `scribetrace-core` (Stage 2) | System call audit (`strace` asserting `O_RDONLY` file open flags) |
| **SEC-20**| 4. Boundary Validation | `[Enforced in Code]` | `THREAT-4.2` | `scribetrace-core::adapters` | Integration test verifying symlinks escaping workspace are skipped |
| **SEC-21**| 4. Boundary Validation | `[Enforced in Code]` | `THREAT-4.3` | `scribetrace-format` | Unit test rejecting Windows ADS (`:`) and reserved device names |
| **SEC-22**| 2. Least Privilege | `[Deferred]` | `THREAT-5.1` | Deferred (Stage 2) | Unit test parsing malformed deep links (Deferred to Stage 2) |
| **SEC-23**| 1. Secure by Default | `[Enforced in Code]` | `THREAT-5.2` | `app/web/src/components/` | E2E test asserting external link dialog presents full URL/URI |
| **SEC-24**| 1. Secure by Default | `[Enforced in Code]` | `THREAT-5.2` | `app/src-tauri/src/ipc/` | Integration test verifying browser dispatch uses system default |
| **SEC-25**| 8. Supply Chain | `[Process]` | `THREAT-6.1` | `.github/workflows/ci.yml` | CI gate running `cargo-audit` and `pnpm audit` on every PR |
| **SEC-26**| 8. Supply Chain | `[Process]` | `THREAT-6.1` | `.github/workflows/release.yml` | CI gate generating CycloneDX JSON SBOM during release build |
| **SEC-27**| 8. Supply Chain | `[Process]` | `THREAT-6.2` | `.github/workflows/*.yml` | Linter gate (`zizmor`/`actionlint`) verifying commit SHA pinning |
| **SEC-28**| 8. Supply Chain | `[Enforced in Code]` | `THREAT-7.1` | `app/src-tauri/src/ipc/` | Integration test verifying manual update action opens system browser |
| **SEC-29**| 8. Supply Chain | `[Process]` | `THREAT-7.2` | `.github/workflows/release.yml` | Verification script asserting SLSA Level 3 provenance generation |
| **SEC-30**| 10. Website / VPS | `[Process]` | `THREAT-8.1` | Maintainer VPS Hosting Config | Configuration audit (SSH key only, UFW, fail2ban active) |
| **SEC-31**| 10. Website / VPS | `[Process]` | `THREAT-8.2` | Caddy / Nginx Reverse Proxy Config | DAST header audit (Mozilla Observatory A+ grade, HSTS preload) |
| **SEC-32**| 9. Secure SDLC | `[Process]` | `THREAT-8.3` | `website/public/.well-known/security.txt` | Automated HTTP check validating RFC 9116 security.txt |
| **SEC-33**| 1. Secure by Default | `[Enforced in Code]` | `THREAT-8.4` | `scribetrace-zotero` | Unit test verifying Zotero loopback path allowlist and limits |
| **SEC-34**| 7. User Privacy | `[Enforced in Code]` | `THREAT-2.6, 4.4` | `scribetrace-core::adapters` | Filesystem audit verifying `device.json` in OS app storage |
| **SEC-35**| 7. User Privacy | `[Enforced in Code]` | `THREAT-4.4` | `scribetrace-core::domain` | Unit test verifying default author label is literal string `'Me'` |
| **SEC-36**| 2. Least Privilege | `[Enforced in Code]` | `THREAT-3.3` | `scribetrace-core::adapters` | Integration test verifying marked subfolder overwrite rules |

---

## 4. Changelog & Architecture Alignment

### Version 3.0 (2026-11-05) - Lean Architecture Alignment
* **SEC-1 (Single Loopback Network Egress):** Eliminated update check exception. Binary connects strictly to Zotero on `127.0.0.1:23119` on click; zero outbound internet sockets in binary ([ADR 0012](adr/0012-distribution-updates-and-network-rule.md)).
* **SEC-28 (Browser-Based Update Visibility):** Removed all in-app update network fetching and updater routines. "Check for Updates" displays version and opens GitHub release URL in system browser via `tauri-plugin-opener` ([ADR 0012](adr/0012-distribution-updates-and-network-rule.md)).
* **SEC-11 (Sequence-Based Forget Semantics):** Aligned with lean `covers = {device: highest seq seen}` vector. Erase operations are clock-independent; peer devices display non-modal banner to delete hidden files ([ADR 0007](adr/0007-per-device-history-log-merge-and-forget.md)).
* **SEC-13 (Bad Line Resilience):** Removed disk writes for malformed lines; invalid lines skipped in-memory and reported via UI/CLI counts without diagnostic files, eliminating disk-filling vectors ([ADR 0007](adr/0007-per-device-history-log-merge-and-forget.md)).
* **SEC-34 (Simplified Device State):** Aligned with `device.json` storing random UUIDv4 in per-user OS app storage outside synced folders ([ADR 0007](adr/0007-per-device-history-log-merge-and-forget.md)).
* **Enforcement Tiers:** Every requirement explicitly tagged `[Enforced in Code]`, `[Process]`, or `[Deferred]`.
* **Link Capture Analysis (ADR 0013):** Confirmed no new component or browser extension; web links captured via Zotero Connector import through existing SEC-33 Zotero broker.
