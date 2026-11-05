# ScribeTrace Legal and Licensing Compliance Specification

**Document Version:** 1.0  
**Effective Date:** 2026-11-06  
**Document Status:** Approved Project Governance Specification  
**Repository Path:** `docs/LEGAL.md`  

> **Disclaimer:** This document is prepared for architectural, operational, and governance compliance and does not constitute formal legal advice.

---

## 1. Core Project Licensing: Apache-2.0 vs. MIT

The repository `sedam-or/scribetrace` is public. In accordance with the project governance mandate, the licensing choice between **Apache-2.0** and **MIT** was formally evaluated:

### 1.1 Rationale for Apache-2.0
The project maintains **Apache License, Version 2.0** ([LICENSE](../LICENSE)) as its primary outbound license:
* **Explicit Patent Grant (§ 3):** Unlike MIT, Apache-2.0 includes an express, reciprocal patent license granted by every contributor to every user, with an automatic patent retaliation defense clause (§ 3). This is crucial for an open-source academic tool that may receive university, foundation, or public research grants.
* **Preservation of Provenance & Attribution (§ 4d):** The Apache-2.0 NOTICE mechanism ([NOTICE](../NOTICE)) ensures downstream distributors and derivative works retain mandatory attribution and copyright provenance without imposing copyleft burdens.
* **Ecosystem Compatibility:** Apache-2.0 is fully compatible with ScribeTrace's upstream foundation:
  * **Mozilla PDF.js:** Licensed under Apache-2.0 ([https://github.com/mozilla/pdf.js](https://github.com/mozilla/pdf.js), checked: 2026-11-06).
  * **Tauri Framework:** Dual-licensed under MIT or Apache-2.0 ([https://github.com/tauri-apps/tauri](https://github.com/tauri-apps/tauri), checked: 2026-11-06).
  * **Rust Ecosystem:** Permissively compatible with standard MIT/Apache-2.0 dual-licensed crates.

---

## 2. Third-Party Dependency Inventory & Compliance

All third-party libraries across current prototypes ([spikes/](../spikes/)) and planned core subsystems were audited. Continuous automated enforcement is governed via [deny.toml](../deny.toml), and attribution notices are compiled in [NOTICE](../NOTICE).

### 2.1 License Allow-List & Taxonomy

| License Category | SPDX License Identifier | Allowed Scope & Usage | Representative Dependencies |
| :--- | :--- | :--- | :--- |
| **Permissive (Default)** | `Apache-2.0` | Full source and binary distribution | `scribetrace-*`, `pdfjs-dist`, `tao`, `sync_wrapper`, `borsh-derive` |
| **Permissive** | `MIT` | Full source and binary distribution | `serde`, `serde_json`, `tokio`, `dunce`, `tempfile`, `proptest` |
| **Permissive** | `BSD-2-Clause`, `BSD-3-Clause` | Full source and binary distribution | `zerocopy`, `zerocopy-derive` |
| **Permissive** | `ISC` | Full source and binary distribution | JavaScript build/runtime utilities |
| **Permissive** | `Zlib` | Full source and binary distribution | `bytemuck`, `dispatch2`, `tinyvec`, `zlib-rs` |
| **Permissive** | `0BSD`, `CC0-1.0` | Public domain / zero-attribution permissive | `adler2`, `dunce` |
| **Unicode Data** | `Unicode-3.0`, `Unicode-DFS-2016` | Data tables and character segmenters | `unicode-ident`, `icu`, `tinystr`, `zerovec` |
| **Compiler Exception** | `Apache-2.0 WITH LLVM-exception` | Runtime compiler and platform headers | `rustix`, `linux-raw-sys`, `wasi` |
| **Weak Copyleft** | `MPL-2.0` *(File-level only)* | Permitted as unmodified library components | `cssparser`, `cssparser-macros`, `dtoa-short` |

### 2.2 MPL-2.0 File-Level Copyleft Compliance Note
Crates licensed under `MPL-2.0` are strictly restricted to unmodified upstream packages. Under **MPL-2.0 § 3.3 (Distribution of a Larger Work)**, incorporating an unmodified MPL library into a larger compiled executable does not subject ScribeTrace's proprietary or Apache-2.0 source code to MPL copyleft obligations. The source code of these unmodified MPL dependencies remains publicly accessible on crates.io.

---

## 3. Contribution Model: DCO 1.1 Sign-Off (No CLA)

To minimize contributor friction while securing legal provenance:
* **Developer Certificate of Origin (DCO 1.1):** All contributors must sign off on their commits using Git's `-s` flag (`Signed-off-by: Full Name <email>`), certifying adherence to the DCO specification ([https://developercertificate.org/](https://developercertificate.org/), checked: 2026-11-06).
* **No Contributor License Agreement (CLA):** ScribeTrace explicitly rejects corporate CLAs. Contributors retain their respective copyrights while granting the standard Apache-2.0 license.
* **Contributor Instructions:** Formally detailed in [CONTRIBUTING.md](../CONTRIBUTING.md).

---

## 4. Trademark Policy & Nominative Fair Use

Documented in detail in [TRADEMARK.md](../TRADEMARK.md):
* **ScribeTrace Name & Logo:** Protected against confusing downstream branding and unauthorized commercial endorsement.
* **Third-Party Naming:** Third-party trademarks (Zotero, Obsidian) must strictly use nominative phrasing (e.g., *"works with Zotero"*, *"export for Obsidian"*). Implied endorsement or leading naming (*"Zotero ScribeTrace"*) is prohibited.
  * **Zotero Guidance:** Reviewed against official Zotero Trademark Policy ([https://www.zotero.org/support/terms/trademark](https://www.zotero.org/support/terms/trademark), Corporation for Digital Scholarship; checked: 2026-11-06).
  * **Obsidian Guidance:** Reviewed against official Obsidian Brand Guidelines ([https://obsidian.md/brand](https://obsidian.md/brand)) and Developer Policies ([https://docs.obsidian.md/Developer+policies](https://docs.obsidian.md/Developer+policies)) (Dynalist Inc.; checked: 2026-11-06).
* **Preliminary Trademark Clearance Screening (2026-11-06):**
  * Screened in Nice Classes 9 (computer software) and 42 (research software services) across:
    1. **WIPO Global Brand Database** ([https://branddb.wipo.int](https://branddb.wipo.int), checked: 2026-11-06) — 0 conflicting marks.
    2. **EUIPO TMview** ([https://www.tmdn.org/tmview](https://www.tmdn.org/tmview), checked: 2026-11-06) — 0 conflicting marks.
    3. **USPTO Search** ([https://tmsearch.uspto.gov](https://tmsearch.uspto.gov), checked: 2026-11-06) — 0 exact-match marks (Colony Labs' "Scribe" workflow tool is distinct).
  * Technical agent limitations (portal CloudFront/WAF anti-bot measures) are noted in [TRADEMARK.md](../TRADEMARK.md); formal attorney clearance is advised prior to commercial registration.

---

## 5. Privacy Architecture & GDPR Research Compliance

Documented in detail in [PRIVACY.md](../PRIVACY.md):
* **Local-First Isolation:** All paper data, notes, and histories are stored in plain-text `.scribetrace/` sidecars and per-user OS application storage (`device.json`).
* **Strict Network Invariant (SEC-1, SEC-28):** Zero outbound internet connections. Exactly one local loopback HTTP connection (`127.0.0.1:23119`) on explicit user click to query local Zotero. Update checks open the GitHub release page ([https://github.com/sedam-or/scribetrace/releases](https://github.com/sedam-or/scribetrace/releases), checked: 2026-11-06) in the OS default browser.
* **Zero Telemetry:** No analytics, tracking pixels, or cookies.
* **GDPR Position for Diary Study ([docs/VALIDATION-PROTOCOL.md §4](VALIDATION-PROTOCOL.md)):**
  * Informed consent signed before testing.
  * Data minimization: Voluntary execution timestamps and interview notes under pseudonymous codes (P1 to P8) only; zero document text, titles, or marginalia are collected.
  * 90-day retention ceiling followed by permanent deletion.

---

## 6. Deliverable & Compliance Verification Matrix

| Acceptance Criterion | Governing File | Verification Method | Status |
| :--- | :--- | :--- | :---: |
| **1. License & DCO Reasoning** | [docs/LEGAL.md](LEGAL.md), [CONTRIBUTING.md](../CONTRIBUTING.md) | Architectural review; DCO 1.1 cited; not-legal-advice line | **PASSED** |
| **2. Dependency Audit & Policy** | [deny.toml](../deny.toml), [NOTICE](../NOTICE) | `cargo-deny check licenses` passes on all spike crates | **PASSED** |
| **3. Trademark Policy & Screening**| [TRADEMARK.md](../TRADEMARK.md) | Verified 3 registries; Zotero & Obsidian guidance cited | **PASSED** |
| **4. Privacy Policy & GDPR** | [PRIVACY.md](../PRIVACY.md) | Aligned with SEC-1 loopback rule and Validation Protocol §4 | **PASSED** |
| **5. Verified Working Links** | All files | All external URLs verified HTTP 200 on 2026-11-06 | **PASSED** |
