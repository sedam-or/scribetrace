# ScribeTrace Privacy Policy

**Effective Date:** 2026-11-06  
**Document Status:** Approved Project Privacy Policy  
**Repository Path:** `PRIVACY.md`  

> **Summary:** ScribeTrace is a local-first desktop application with zero telemetry, zero analytics, and zero cloud tracking. Your documents, notes, and reading history remain entirely on your computer.

---

## 1. Desktop Application Privacy Architecture

ScribeTrace is designed around the architectural principle of **Privacy by Design**. The software respects your privacy by refusing to collect personal data or transmit local documents across the network.

### 1.1 What the Application Stores
* **Local Workspace Data:** All reading histories, annotation snapshots, and note revisions are saved strictly as plain-text files inside the `.scribetrace/` directory within your chosen local workspace:
  * Event logs: `<workspace>/.scribetrace/papers/<paper-id>/<device-id>.jsonl`
  * Synthesis notes: `<workspace>/.scribetrace/notes/`
* **Local Application Configuration:** A single configuration file (`device.json`) containing an anonymous random UUIDv4 device identifier is stored in your operating system's per-user application configuration directory (`~/.config/scribetrace/` on Linux, `%APPDATA%/scribetrace/` on Windows, `~/Library/Application Support/scribetrace/` on macOS). This file is never written into the shared workspace directory or synced across machines.
* **Sacred Originals:** Source papers (PDFs, Markdown) are opened using strictly read-only operating system file descriptors (`O_RDONLY`). ScribeTrace never modifies, copies, or uploads source documents.
* **Zero Accounts:** ScribeTrace does not require account registration, passwords, API tokens, or login credentials.

### 1.2 Strict Single-Destination Network Invariant
Aligned with [ARCHITECTURE.md](docs/ARCHITECTURE.md) and security specification **SEC-1**:
* **Zero Outbound Internet Sockets:** The application binary compiles zero outbound network telemetry and initiates no unprompted background network requests.
* **The Single Loopback Exception:** When you explicitly click "Import / Refresh from Zotero", the Rust core broker sends a local HTTP GET request strictly to `127.0.0.1:23119` (your local Zotero desktop instance). This traffic never leaves your local machine.
* **Browser-Delegated Update Checks:** In accordance with **SEC-28**, clicking "Check for Updates" executes zero in-app network requests and parses no remote manifests. The app displays your installed version and opens the official GitHub release page ([https://github.com/sedam-or/scribetrace/releases](https://github.com/sedam-or/scribetrace/releases), checked: 2026-11-06) in your default operating system browser via `tauri-plugin-opener`.
* **External Link Confirmation:** Any external link clicked in notes or metadata triggers a confirmation dialog showing the full unshortened destination URL before dispatching to your system browser (**SEC-23**, **SEC-24**).

### 1.3 Telemetry, Cookies, and Analytics
* **Zero Telemetry:** ScribeTrace contains no analytics libraries, tracking pixels, crash-reporting agents, or usage telemetry.
* **No Cookies or Local Tracking:** The sandboxed webview operates with local bundle isolation and sets no persistent cookies or web storage tracking identifiers.

---

## 2. Research Validation & Diary Study (GDPR Compliance)

When conducting user research, diary studies, or usability evaluations (such as the validation protocol detailed in [docs/VALIDATION-PROTOCOL.md](docs/VALIDATION-PROTOCOL.md)), the project strictly complies with the European Union General Data Protection Regulation (**GDPR**, Regulation (EU) 2016/679) regarding lawful processing, data minimization, and participant rights.

### 2.1 Participant Informed Consent
All research participants review and sign an explicit informed consent agreement before participating in any evaluation study ([docs/VALIDATION-PROTOCOL.md §4](docs/VALIDATION-PROTOCOL.md)):

> **Informed Consent Agreement:**  
> *"I agree to take part in a short test of a note-history tool. I understand that: (1) the tool runs only on my computer and sends nothing anywhere; (2) I will share only the times at which I ran the report and my answers in a chat, never paper titles, highlighted text or notes; (3) I can read the program's source first, stop at any time, and ask for my answers to be deleted; (4) notes are kept under a code (P1 to P8), not my name, and deleted 90 days after the final report (target). I am responsible for not testing on confidential manuscripts."*

### 2.2 Data Minimization & Protection Measures
* **Data We Collect:**
  * Voluntary execution timestamps (UTC run logs generated locally by the research prototype).
  * Hand-written notes from voluntary post-study interviews, stored under pseudonymous participant codes (P1 to P8).
  * Task completion times recorded during usability testing.
* **Data We NEVER Collect:**
  * Paper titles, author names, or publication abstracts.
  * Highlighted document text, marginalia, or user-authored notes.
  * Audio or video screen recordings.
  * Contact information beyond the minimal email address required to coordinate the research session (deleted immediately after study conclusion).

### 2.3 Retention & Right to Erasure (GDPR Art. 17)
* **Retention Limit:** All pseudonymous interview notes and timestamp logs are stored on encrypted local media and permanently deleted within **90 days** of publishing the final validation report.
* **Right to Withdraw & Erasure:** Participants may withdraw from the study at any time without penalty and may request immediate deletion of their pseudonymous responses by notifying the study coordinator.
* **Ethics Compliance:** Participants are responsible for confirming whether their home institutions require prior institutional review board (IRB) or ethics committee approval before enrolling.

---

## 3. Contact & Rights

To exercise your rights under GDPR or if you have questions regarding ScribeTrace's privacy practices, please contact the maintainers via:
* **GitHub Repository:** [https://github.com/sedam-or/scribetrace](https://github.com/sedam-or/scribetrace) (Checked: 2026-11-06)
* **Security & Vulnerability Reporting:** Coordinated vulnerability disclosure details in [SECURITY.md](SECURITY.md).
