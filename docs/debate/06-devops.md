# Round 6: DevOps & Platform Critique

**Role:** Senior DevOps Engineer  
**Lens:** Build & release pipelines, multi-OS distribution (GitHub Releases, Flathub, Homebrew, winget), code signing, CI minutes & budget, contributor onboarding friction.  
**Reference:** [05-tech-options.md](05-tech-options.md) by Architect.

---

### 1. Decision 2 (Desktop Shell): The "Electron Pivot Checkpoint" Trap

* **Architect's Proposal:** Tauri v2 for Stage 1, with a checkpoint at Stage 2 start to evaluate PDF rendering and potentially pivot to Electron.
* **Strongest Objection:** A framework pivot at Stage 2 is not a "checkpoint"—it is a catastrophic rewrite of the build, release, CI, and packaging pipelines. Tauri and Electron have fundamentally incompatible build toolchains, system dependencies, and runtime footprints:
  * **CI Pipeline & Contributor Setup:** Tauri requires Rust + Cargo + OS webview toolchains (`webkit2gtk-4.1` on Linux, MSVC on Windows, Xcode CLI on macOS). Electron requires Node.js, Chromium headers, and native build toolchains (`node-gyp`). Switching invalidates every GitHub Actions workflow, cache strategy (`sccache`), and contributor onboarding runbook.
  * **Binary Size & Distribution:** Tauri outputs ~12–18 MB native installers (`.deb`, `.AppImage`, `.msi`, `.dmg`). Electron ships a bundled ~120 MB Chromium runtime. A 10× binary bloat directly hurts Flathub storage, GitHub Releases bandwidth, and user download friction.
  * **Core Architecture Divergence:** The core library is in Rust (`palimpsest-core`). In Tauri, frontend RPC is native IPC. In Electron, running Rust requires Node-API/Neon native addons, which are notorious CI cross-compilation nightmares across macOS Universal (x86_64 + aarch64) and Windows arm64.
* **Risk:** The team wastes weeks setting up Tauri CI and packaging, only to throw it away and rebuild from scratch for Electron in Stage 2.
* **Alternative to Defend:** **Commit to Tauri v2 now and drop the Electron pivot.** If Linux WebKitGTK exhibits PDF.js rendering quirks during Stage 2, resolve them in the web layer (e.g. PDF.js SVG backend, offscreen rendering, or foliate-js) rather than swapping the underlying desktop shell.

---

### 2. Decision 9 (Packaging & Updates): Dropping Auto-Updates is a False Economy

* **Architect's Proposal:** Defer auto-updates (SEC-28) to post-MVP; rely solely on package managers and manual website downloads to preserve the "no network" pledge.
* **Strongest Objection:** In a Stage 1 MVP, rapid iterations and bug fixes (especially for Zotero schema edge cases) are frequent. Without an update mechanism, macOS and Windows direct-download users will be stranded on broken v0.1 builds:
  * **Package Manager Reality:** While Linux users enjoy automated updates via Flathub or package managers, **macOS and Windows users who download `.dmg` or `.msi` from GitHub have zero native update mechanism**. They will never manually check GitHub Releases weekly.
  * **Zero Server Infrastructure:** Tauri v2's `@tauri-apps/plugin-updater` requires **no custom server**. It polls a static `latest.json` manifest hosted on GitHub Releases and validates cryptographic Minisign signatures locally on the user's machine before applying.
  * **Network Pledge Compliance:** Auto-updates do not violate "local-first, offline". We simply make update checks **manual on click** ("Check for Updates" button in Settings) or prompt the user with an explicit opt-in toggle during onboarding (`[ ] Check for updates on startup`).
* **Risk:** High support ticket volume and user churn from users running obsolete, buggy versions with no knowledge that fixes exist.
* **Alternative to Defend:** Implement **Tauri's static-hosted updater for Windows & macOS in Stage 1**, configured to **manual on-demand check only** (zero background network pings). Linux delegates updating to Flathub and system package managers.

---

### 3. Decision 4 (History Log): Per-Device File Sprawl & Fragile Erase Marker

* **Architect's Proposal:** Separate append-only files per device (`<paper>/<device>.jsonl`) merged at read time; "forget history" writes an "erase requested" marker file.
* **Strongest Objection:**
  * **Filename Volatility & Sprawl:** If `<device>` is derived from system hostnames (e.g. `laptop`, `desktop`), hostname changes, OS reinstalls, or container environments will spawn fragmented, orphaned event logs in the sync directory.
  * **Sync Resurrection of Markers:** In distributed sync (Git, Syncthing, Dropbox), marker files frequently suffer from race conditions, conflicting file renames, or resurrection by offline peers.
* **Risk:** Incomplete data erasure (violating the user's privacy expectation) and sync conflicts on marker metadata files.
* **Alternative to Defend:**
  1. Base `<device>` on a persistent random UUID generated once in `.palimpsest/device.json` per installation, not on OS hostnames.
  2. Implement "forget history" as a **physical local deletion of `<paper>/` sidecars** plus writing an explicit tombstone event (`{"type": "purge", "target": "<paper-key>", "ts": HLC}`) into the local log, so union merges deterministically suppress resurrected events without magic marker files.

---

### 4. Decision 6 (Zotero Integration): Reject Direct SQLite Fallback

* **Architect's Proposal:** Use Zotero Local HTTP API (`127.0.0.1:23119`), with direct SQLite reading (`zotero.sqlite`) as a fallback if the API is disabled.
* **Strongest Objection:** Supporting direct SQLite access as a fallback is a severe maintenance and reliability trap:
  * `zotero.sqlite` is locked with exclusive SQLite WAL locks while Zotero is running.
  * Zotero's internal schema is undocumented, changes between major versions (Zotero 6 vs 7), and differs across custom plugins.
  * Bundling SQLite drivers and testing against an undocumented, evolving third-party schema in CI adds massive CI maintenance burden.
* **Risk:** Palimpsest crashes or corrupts locked database files; CI tests fail when Zotero updates internal table structures.
* **Alternative to Defend:** **Hard fail when the Local HTTP API is unavailable.** If disabled, show a guided 2-click prompt: *"Please enable Zotero Local API in Zotero Preferences → Advanced"*. In CI, mock the local HTTP API with a lightweight 50-line mock HTTP server in tests—clean, deterministic, and 100% reliable.

---

### 5. Code Signing & CI Budget Realities (Cost Picture)

* **GitHub Actions CI Minutes:** Building Tauri on 3 OSes (`ubuntu-22.04`, `macos-14`, `windows-latest`) takes ~12–15 min per release without cache, and ~4 min with `sccache` + GitHub Actions cache. For public open-source repos, standard GitHub-hosted runners are free and unlimited.
* **Code Signing Requirements:**
  * **macOS:** Apple Developer Program ($99/year) is essential to avoid macOS Gatekeeper quarantine. Without notarization, users must run `xattr -cr` in Terminal.
  * **Windows:** Use **SignPath.io** (free code signing for open source projects) or Microsoft Azure Trusted Signing (~$10/mo).
  * **Linux:** Flathub signs Flatpaks automatically; AppImage/deb can be signed via GPG in GitHub Actions at $0 cost.
