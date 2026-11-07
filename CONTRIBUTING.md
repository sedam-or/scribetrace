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

### Prerequisites

All platforms need Git, Rust stable via [rustup](https://www.rust-lang.org/tools/install), Node.js 22 or later, and pnpm 9.15.9. Enable Corepack once:

```sh
corepack enable
corepack prepare pnpm@9.15.9 --activate
```

Install the native Tauri dependencies for your operating system before building the desktop app:

- **Ubuntu/Debian:** `sudo apt-get update && sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential curl wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev`
- **Fedora:** `sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file libappindicator-gtk3-devel librsvg2-devel libxdo-devel gcc gcc-c++ make`
- **macOS:** install Xcode Command Line Tools with `xcode-select --install`.
- **Windows:** install Visual Studio Build Tools with the **Desktop development with C++** workload. WebView2 is included on Windows 10 (1803+) and later; install the Evergreen WebView2 Runtime on older supported systems.

See the [official Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for other Linux distributions and current platform details.

### Clone, build, and test

```sh
git clone https://github.com/sedam-or/scribetrace.git
cd scribetrace
corepack enable
corepack prepare pnpm@9.15.9 --activate
pnpm install --frozen-lockfile
pnpm lint
pnpm test
pnpm build
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
```

Run the desktop app on Linux after `pnpm build`:

```sh
WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo run -p scribetrace-shell --features custom-protocol
```

On macOS or Windows, run `cargo run -p scribetrace-shell --features custom-protocol` after `pnpm build`.

The real-window E2E harness is Linux-only. Install its additional dependencies and `tauri-driver`, then build and run it:

```sh
sudo apt-get install -y xvfb webkit2gtk-driver xdotool
cargo install tauri-driver --locked
pnpm build
cargo build --release -p scribetrace-shell --features custom-protocol
./e2e/run-linux.sh --release
```

The runner uses Xvfb automatically when no display is available. It also sets the WebKitGTK environment required for the test. The harness enforces a 2-second cold-start limit and a 200 MiB whole-process-tree idle-memory limit. Containerized or heavily shared hosts can add memory overhead; use a native Linux host for meaningful local memory measurements and compare with the GitHub Actions E2E result. See [e2e/README.md](e2e/README.md) and [docs/TECH-OPTIMIZATION.md](docs/TECH-OPTIMIZATION.md) for details.

### Your first PR

1. Find an open issue labelled [`good first issue`](https://github.com/sedam-or/scribetrace/labels/good%20first%20issue) and comment that you plan to work on it.
2. Create a branch, for example `fix/42-short-description` or `docs/contributor-setup`.
3. Make focused commits with `git commit -s`; each commit needs a `Signed-off-by:` line.
4. Run `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test --workspace --locked`, `pnpm lint`, `pnpm test`, and `pnpm build`.
5. Push your branch and open a pull request against `main`. Describe the change, link the issue (for example, `Closes #42`), and include the checks you ran.

Contributors should use a fork and pull request. Maintainers may merge changes after required CI and DCO checks pass. Main-branch protection allows maintainer administrators to bypass the pull-request requirement for operational pushes.

---

## 4. Pull Request Guidelines

1. **Branch Naming:**
   - Feature branches: `feat/<feature-name>`
   - Bug fixes: `fix/<issue-number>-<description>`
   - Documentation: `docs/<topic>`
2. **Commit Messages & DCO Sign-off:** Follow `COMMIT-PROTOCOL.md` (Conventional Commits) and ensure every commit includes `-s` / `Signed-off-by:`.
3. **Pre-PR Verification:** Follow the full clean-clone build and test commands in Section 3. At minimum, run Rust formatting, clippy, workspace tests, UI lint and tests, and the UI build. Ensure no secrets or temporary files are included.
4. **Link Issues:** Reference related issues in the PR description (e.g., `Closes #42`).

---

## 5. Architectural Boundaries

- Review `docs/ARCHITECTURE.md` before proposing cross-layer changes.
- Core domain logic belongs in `scribetrace-core` (pure Rust, platform-agnostic).
- Desktop bindings belong in `src-tauri/`.
- UI rendering and interactions belong in `src/`.

---

## 6. Security Vulnerabilities

Please **do not report security vulnerabilities in public issues**. Use GitHub's private vulnerability reporting for this repository.

---

## 7. Code of Conduct

All contributors and participants are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md).
