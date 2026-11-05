# Development Guide — ScribeTrace

This document outlines local environment setup, installed toolchain versions, one-command verification, and how to execute prototype spikes.

---

## 1. Prerequisites & Toolchain Versions

All development tools are installed at user level without root/sudo privilege:

| Component | Tool / Package | Exact Version | Purpose & Rationale |
|:---|:---|:---|:---|
| **OS** | Linux x86_64 | Linux 6.13.x | Host development environment |
| **Rust Toolchain** | `rustc` / `cargo` | `1.99.1` | Core systems language |
| **Rust Components** | `clippy`, `rustfmt`, `llvm-tools` | Installed via `rustup` | Formatting, static analysis, code coverage (SEC-26) |
| **Supply Chain** | `cargo-deny` | `0.20.2` | Licence allowlist and banned crate validation (SEC-25) |
| **Vulnerability Audit** | `cargo-audit` | `0.22.2` | Advisory database scans for dependencies (SEC-26) |
| **Next-Gen Test Runner**| `cargo-nextest` | `0.9.149` | Fast parallel test execution and partition runner |
| **Coverage Instrumentation**| `cargo-llvm-cov` | `0.9.1` | Source-level code coverage measurement |
| **Node.js Runtime** | `node` | `v22.23.2` | Frontend build tooling and PDF.js baseline harness |
| **Package Manager** | `pnpm` | `9.15.9` | Pinned fast package manager via Corepack |
| **System Webview** | `WebKitGTK 4.1` (`libwebkit2gtk-4.1-dev`) | Pre-installed | Native Linux WebKit engine for Tauri v2 |
| **Headless Display** | Native X.Org `DISPLAY=:0` / Docker container | Native `:0` active | Running webview test harnesses without dedicated monitors |

### Declined Virtualization Environments (With Rationale)
- **macOS Virtual Machine (Declined):** Apple SLA strictly restricts macOS virtualization to physical Apple-branded hardware. Running a macOS VM on standard Linux hardware violates licence constraints. Real macOS coverage is provided directly by free GitHub Actions macOS runners (`macos-latest`) on the public repository.
- **Windows Virtual Machine (Declined):** A local Windows VM requires ~40 GB disk space (on an ~84 GB SSD) and ~6 GB RAM, starving local dev headroom. Real WebView2 validation is provided via free GitHub Actions Windows runners (`windows-latest`) with zero overhead on local development.

---

## 2. One-Command Build & Test

To verify code formatting, static analysis, and all unit / property / integration tests across all crates:

```bash
# Check formatting
cargo fmt --all -- --check

# Run strict clippy linting across all targets
cargo clippy --all-targets -- -D warnings

# Execute all tests with nextest
cargo nextest run

# Run supply-chain security checks
cargo deny check
cargo audit
```

---

## 3. Running Prototype Spikes

Each spike under `spikes/` can be run independently:

### SPIKE-1: Log-Merge commutativity & replay
```bash
cd spikes/log-merge
cargo test --release
```
Runs 10,000 property-based commutativity/idempotency cases and replay benchmarks.

### SPIKE-2: Purge suppression & visibility
```bash
cd spikes/purge
cargo test --release
```
Runs 10,000 random purge order permutations and boundary coverage table tests.

### SPIKE-4: Tauri WebView & PDF.js Lightness
```bash
cd spikes/tauri-webview
pnpm install
cargo build --release
./measure.sh
```
Executes cold start and memory benchmarks against a 500-page / 50 MB PDF fixture.
