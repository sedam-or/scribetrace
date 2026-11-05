# Security Policy

## Supported Versions

ScribeTrace is committed to providing rapid security updates and patches for active release lines.

| Version | Supported          | Security Maintenance Tier |
|:---|:---|:---|
| **1.0.x** | :white_check_mark: | Full Active Security Support (Patches & Fixes) |
| **< 1.0** | :x:                | Unsupported (Pre-release) |

---

## Reporting a Vulnerability

We strongly encourage coordinated vulnerability disclosure. If you discover a security vulnerability in ScribeTrace, **please do not open a public issue or discuss it in public chat channels.**

### Preferred Reporting Method
Please report security vulnerabilities using **GitHub Private Vulnerability Reporting**:
- Navigate to the [Security Advisories](https://github.com/scribetrace-app/scribetrace/security/advisories) tab of the repository.
- Click **"Report a vulnerability"** to submit your findings confidentially.

### Alternative Reporting Channel
If you cannot use GitHub Security Advisories, send an encrypted email to:
- **Security Contact:** `security@scribetrace.app`
- **PGP Fingerprint:** `8F3A B920 4D11 C57E E390  128A 47BC 99E2 3120 F89A`

Please include:
1. A detailed description of the vulnerability and attack vector.
2. Step-by-step reproduction instructions or a minimal Proof-of-Concept (PoC).
3. Affected versions and operating systems (Linux, Windows, macOS).
4. Potential impact (e.g. arbitrary file read, XSS, denial of service).
5. Any known mitigations or proposed remediations.

---

## Response Times & SLA Commitments

Our security response team adheres to strict service level agreements:

* **Initial Acknowledgement:** Within **48 hours** of report receipt.
* **Triage & Severity Assessment:** Within **5 business days**, including CVSS v3.1 scoring.
* **Remediation Target:**
  * **Critical Severity (CVSS 9.0–10.0):** Patch released within **7 calendar days**.
  * **High Severity (CVSS 7.0–8.9):** Patch released within **14 calendar days**.
  * **Medium Severity (CVSS 4.0–6.9):** Patch released within **30 calendar days**.
  * **Low / Informational (CVSS < 4.0):** Addressed in next scheduled minor release.

---

## Disclosure Policy

ScribeTrace follows a standard **90-day coordinated disclosure policy**:
1. We will work with the reporter to validate the vulnerability and develop a patch.
2. Pre-release security advisories will be shared with the reporter for review before release.
3. Upon publishing the fix, a public GitHub Security Advisory and CVE will be issued crediting the researcher (unless anonymity is requested).

---

## Scope & Security Boundary

### In-Scope Surfaces
- ScribeTrace desktop client binaries (Linux, Windows, macOS).
- Untrusted document parsers (PDF, EPUB, arXiv HTML, Markdown).
- Local storage and sidecar ingestion (`.scribetrace/` JSON Lines).
- Tauri v2 IPC bridge capabilities and webview sandboxing.
- Official application auto-updater and release distribution pipelines.

### Out-of-Scope Surfaces
- Compromise requiring pre-existing root/administrator access on the host system.
- Attacks requiring physical access to an unlocked host machine.
- Social engineering of developers or maintainers.
- Non-security bugs and cosmetic issues.
- Attacks targeting third-party cloud synchronization servers (e.g., Dropbox, Google Drive infrastructure).
