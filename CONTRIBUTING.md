# Contributing to ScribeTrace

Thank you for your interest in contributing to ScribeTrace. This document provides guidelines to ensure contributions align with the project's requirements, architectural invariants, and governance.

Please read this guide before opening an issue or submitting a pull request.

---

## 1. Governance & Principles

ScribeTrace is an open-source, local-first companion tool for academic paper reading. All contributions must respect our core principles:
1. **Sacred Originals:** Source paper files (PDF, Markdown) are strictly read-only. We never alter source bytes.
2. **Local-First & Offline:** 100% functional offline. Zero outbound telemetry or account requirements.
3. **Open Formats:** Annotation history is stored as append-only plain-text sidecars in `.scribetrace/`.
4. **Apache-2.0 Licensing:** All contributions are submitted under the terms of the Apache License, Version 2.0.
5. **Developer Certificate of Origin (DCO):** We use the standard DCO process instead of a Contributor License Agreement (CLA) to ensure low friction and clear legal provenance.

---

## 2. Developer Certificate of Origin (DCO) & Sign-Off

To ensure legal clarity and protect the project and its users without the administrative friction of a corporate Contributor License Agreement (CLA), ScribeTrace adopts the standard **Developer Certificate of Origin (DCO 1.1)**, widely used across the Linux kernel and open-source ecosystems.

### How to Sign Off Your Commits
Every commit submitted to ScribeTrace must contain a `Signed-off-by:` line with your real name and email address, asserting that you have the legal right to submit the code under the Apache-2.0 license.

You can add this sign-off automatically using the `-s` flag in git:
```bash
git commit -s -m "feat(core): implement monotonic sequence check"
```

This appends a line matching your Git identity:
```text
Signed-off-by: Jane Contributor <jane@example.org>
```

### DCO 1.1 Text
By signing off a commit, you certify the following statements:
> **Developer Certificate of Origin 1.1**  
> Source: https://developercertificate.org/ (Checked: 2026-11-06)  
> 
> By making a contribution to this project, I certify that:  
> (a) The contribution was created in whole or in part by me and I have the right to submit it under the open source license indicated in the file; or  
> (b) The contribution is based upon previous work that, to the best of my knowledge, is covered under an appropriate open source license and I have the right under that license to submit that work with modifications, whether created in whole or in part by me, under the same open source license (unless I am permitted to submit under a different license), as indicated in the file; or  
> (c) The contribution was provided directly to me by some other person who certified (a), (b) or (c) and I have not modified it.  
> (d) I understand and agree that this project and the contribution are public and that a record of the contribution (including all personal information I submit with it, including my sign-off) is maintained indefinitely and may be redistributed consistent with this project or the open source license(s) involved.

We explicitly **do not require a Contributor License Agreement (CLA)**. Copyright remains with each individual contributor.

---

## 3. Development Setup

*(Detailed build commands will be expanded as spikes graduate into the main workspace)*

### Prerequisites
- **Rust Toolchain:** Stable Rust (1.80+) via `rustup`.
- **Node.js:** Node.js 20+ with `pnpm` (or `npm`).
- **OS Dependencies (Linux):** `libwebkit2gtk-4.1-dev`, `build-essential`, `curl`, `wget`, `file`, `libxdo-dev`, `libssl-dev`, `libayatana-appindicator3-dev`.

---

## 4. Pull Request Guidelines

1. **Branch Naming:**
   - Feature branches: `feat/<feature-name>`
   - Bug fixes: `fix/<issue-number>-<description>`
   - Documentation: `docs/<topic>`
2. **Commit Messages & DCO Sign-off:** Follow `COMMIT-PROTOCOL.md` (Conventional Commits) and ensure every commit includes `-s` / `Signed-off-by:`.
3. **Pre-PR Verification:**
   - Ensure `cargo fmt --check` and `cargo clippy -- -D warnings` pass.
   - Run tests: `cargo test`.
   - Ensure `cargo-deny --config deny.toml check licenses` passes.
   - Ensure no secrets or temporary files are included.
4. **Link Issues:** Reference related issues in the PR description (e.g., `Closes #42`).

---

## 5. Architectural Boundaries

- Review `docs/ARCHITECTURE.md` before proposing cross-layer changes.
- Core domain logic belongs in `scribetrace-core` (pure Rust, platform-agnostic).
- Desktop bindings belong in `src-tauri/`.
- UI rendering and interactions belong in `src/`.

---

## 6. Security Vulnerabilities

Please **do not report security vulnerabilities in public issues**. Refer to `SECURITY.md` for our coordinated disclosure process via GitHub Private Vulnerability Reporting or encrypted email.

---

## 7. Code of Conduct

All contributors and participants are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md).
