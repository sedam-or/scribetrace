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

---

## 2. Development Setup

*(Detailed build commands will be expanded as spikes graduate into the main workspace)*

### Prerequisites
- **Rust Toolchain:** Stable Rust (1.80+) via `rustup`.
- **Node.js:** Node.js 20+ with `pnpm` (or `npm`).
- **OS Dependencies (Linux):** `libwebkit2gtk-4.1-dev`, `build-essential`, `curl`, `wget`, `file`, `libxdo-dev`, `libssl-dev`, `libayatana-appindicator3-dev`.

---

## 3. Pull Request Guidelines

1. **Branch Naming:**
   - Feature branches: `feat/<feature-name>`
   - Bug fixes: `fix/<issue-number>-<description>`
   - Documentation: `docs/<topic>`
2. **Commit Messages:** Follow `COMMIT-PROTOCOL.md` (Conventional Commits: `feat:`, `fix:`, `docs:`, etc.).
3. **Pre-PR Verification:**
   - Ensure `cargo fmt --check` and `cargo clippy -- -D warnings` pass.
   - Run tests: `cargo test`.
   - Ensure no secrets or temporary files are included.
4. **Link Issues:** Reference related issues in the PR description (e.g., `Closes #42`).

---

## 4. Architectural Boundaries

- Review `docs/ARCHITECTURE.md` before proposing cross-layer changes.
- Core domain logic belongs in `scribetrace-core` (pure Rust, platform-agnostic).
- Desktop bindings belong in `src-tauri/`.
- UI rendering and interactions belong in `src/`.

---

## 5. Security Vulnerabilities

Please **do not report security vulnerabilities in public issues**. Refer to `SECURITY.md` for our coordinated disclosure process via GitHub Private Vulnerability Reporting or encrypted email.

---

## 6. Code of Conduct

All contributors and participants are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md).
