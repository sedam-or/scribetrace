# Round 6: Security & Trust Review (Technical Options)

**Author:** Security Lead  
**Context:** Review of `docs/debate/05-tech-options.md` through the attack surface and trust lens. Max 1 page; links to earlier specs instead of repeating.

---

## 1. Where I Agree

* **Tauri v2 over Electron (Decision 2):** Tauri's Rust core and narrow typed IPC capabilities give us compile-time memory safety and a radically smaller attack surface than shipping a bloated, CVE-prone Chromium runtime. Keep the Stage 2 checkpoint for PDF rendering, but defend Tauri for security isolation.
* **Per-device log files (Decision 4):** Storing `<paper>/<device>.jsonl` eliminates concurrent multi-process file locking collisions and isolates write authority per machine.
* **Dropping background watchers & live Obsidian mirrors (Decisions 4 & 7):** Moving to user-triggered on-open comparisons and on-click exports eliminates filesystem race conditions and unintentional data leakage.

---

## 2. Strongest Objections & Risks

### Objection 1: The "Erase Marker" Tombstone Fallacy (Decision 4)
Architect proposes an "erase requested" marker to signal history deletion across synced devices.
* **The Security Flaw:** Writing a marker into a synced folder does **not** delete history; it leaves the old `<device>.jsonl` files sitting on disk in plain text. Any collaborator, sync client (Git/Dropbox), or local tool can still read confidential annotations. Worse, an unverified marker can be ignored, lost in a sync conflict, or maliciously injected.
* **The Risk:** Researchers under non-disclosure agreements or double-blind review ethics require actual data destruction. An app that claims a paper is "forgotten" while leaving its history intact on disk creates legal and institutional liability.

### Objection 2: The Unbounded Loopback Hole (Decision 6)
Architect proposes carving out a "loopback to Zotero only" exception to the zero-network rule (`http://127.0.0.1:23119`).
* **The Security Flaw:** If the webview or an unconstrained HTTP client can query `127.0.0.1`, an attacker can leverage Server-Side Request Forgery (SSRF) or local port scanning against other services running on the researcher’s workstation (e.g., local database ports, Ollama, Jupyter, or development servers).
* **The Risk:** Rogue scripts or malformed document metadata could probe the local host.

### Objection 3: The "No Updates" Trap (Decision 9)
Architect proposes dropping auto-updates entirely in MVP to avoid an outbound network connection.
* **The Security Flaw:** Document readers process complex, untrusted binary files (PDFs, embedded fonts). When a critical parsing vulnerability or XSS flaw is patched in Palimpsest or PDF.js, a client with zero update notifications leaves researchers permanently running a vulnerable binary. Academics rarely visit GitHub release pages manually.

---

## 3. Alternatives I Defend

1. **For "Forget History" (Decision 4): Deterministic Local Shred + Purge Epoch**
   * On "Forget paper": immediately hard-delete (unlink and overwrite) all local log files for that paper on this machine.
   * Write an authenticated `purge.json` containing `(paper_id, purge_timestamp, device_nonce)`. When any other device opens the workspace, it checks this timestamp and **hard-deletes** all pre-existing events predating the purge epoch, refusing to replay them.
   * Transparent UI disclosure: *"Local history shredded. If syncing via Git or Dropbox, verify that remote files are deleted."* Never pretend software can remotely scrub offline machines without network access.

2. **For Zotero Loopback (Decision 6): Rust-Brokered, Pinned IPC Gateway**
   * The webview CSP must retain `connect-src 'none'`. The webview **never** touches network sockets directly.
   * All Zotero calls are handled exclusively by a dedicated Rust IPC command:
     * Hardcoded target: `127.0.0.1:23119` strictly (no DNS lookup, `allow_redirects = false`).
     * Strict path allowlist: `/api/users/0/items*` only.
     * Triggered **only on explicit user click** ("Import from Zotero"); never in the background.
     * Validate response headers (`X-Zotero-Version`) and enforce strict schema parsing before returning data to the UI.

3. **For Updates (Decision 9): Zero-Egress by Default + Explicit "Check for Updates" Button**
   * Keep zero background network traffic by default (no background update checkers).
   * Provide an explicit, user-triggered menu action: *"Check for Updates"*.
   * When clicked, it performs a single HTTPS GET to fetch a static manifest (`https://palimpsest.app/releases/latest.json`) with pinned TLS.
   * The app **does not execute in-place binary self-updates**; it simply alerts the user: *"Version X.Y is available with security updates. [Download / Package Manager Command]"*. This eliminates auto-updater RCE attack vectors while preventing abandoned, vulnerable installations.

4. **For Device IDs (Decision 4): Anonymous Random UUIDv4**
   * Never derive `<device>.jsonl` filenames from system hostnames, usernames, or MAC addresses, which leak researcher identities in shared Git repos.
   * Generate an anonymous `UUIDv4` stored in `.palimpsest/device-id` upon initial workspace setup.
