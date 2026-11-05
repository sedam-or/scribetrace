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
pnpm install --frozen-lockfile
pnpm lint && pnpm test && pnpm build        # UI: boundary lint, 3 headless tests, bundle (dist/ is needed by the shell crate)
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo llvm-cov -p scribetrace-core --fail-under-lines 80 --summary-only
cargo deny check && cargo audit --ignore RUSTSEC-2024-0370 --ignore RUSTSEC-2024-0429
./tools/check-gates.sh                      # layering and budget gates
```

Run the app: start `env WEBKIT_DISABLE_DMABUF_RENDERER=1 cargo run -p scribetrace-shell` after `pnpm build` (per [ADR 0014](adr/0014-linux-idle-memory-budget.md) for Linux WebKitGTK memory tuning), or `pnpm dev` for the UI alone.
The rules a change must meet are in [DEFINITION-OF-DONE](DEFINITION-OF-DONE.md).

### Repository map

| Path | What lives here | May depend on |
|---|---|---|
| `src-core/` | crate `scribetrace-core`: `domain/` (pure), `application/` (use cases + ports), `adapters/` (JSONL store, Zotero HTTP, clock, ids) | nothing UI, no tauri, no TLS |
| `src-tauri/` | crate `scribetrace-shell`: thin IPC gateway; `build.rs` + `capabilities/default.json` are the command allowlist | core |
| `src/` | React UI. Talks to the core only through `src/rpc/rpc.ts`; `tauri-adapter.ts` is the single Tauri-aware file | nothing else |
| `tests-ui/`, `src-core/tests/`, `src-tauri/tests/` | headless tests per layer (no display needed) | |
| `test-support/zotero-standin/` | dev-only loopback stand-in for Zotero, serves `tests/fixtures/zotero/` | |
| `tools/` | `check-gates.sh`, `lint-ui-boundaries.mjs` (+ its self-test), `transitive-baseline.txt` | |
| `spikes/` | throw-away experiments, not workspace members | |

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

---

## 4. Packaging & Linux Release

ScribeTrace ships production-ready Linux bundles (`.deb` and standalone `.AppImage`):

### Building Release Installers
```bash
pnpm install --frozen-lockfile
pnpm build
pnpm tauri build -b deb,appimage
```
Bundles are generated under `target/release/bundle/`:
- Debian package: `target/release/bundle/deb/ScribeTrace_0.1.0_amd64.deb` (~1.4 MB)
- Standalone AppImage: `target/release/bundle/appimage/ScribeTrace_0.1.0_amd64.AppImage`

Both packages are dramatically under the Stage 1 release budget ($\le 20$ MB).

### Installing & Running
- **Debian / Ubuntu / Pop!_OS:**
  ```bash
  sudo dpkg -i target/release/bundle/deb/ScribeTrace_0.1.0_amd64.deb
  # Launches with WEBKIT_DISABLE_DMABUF_RENDERER=1 per ADR 0014
  scribetrace
  ```
- **Any Linux distribution (AppImage):**
  ```bash
  chmod +x target/release/bundle/appimage/ScribeTrace_0.1.0_amd64.AppImage
  ./target/release/bundle/appimage/ScribeTrace_0.1.0_amd64.AppImage
  ```
