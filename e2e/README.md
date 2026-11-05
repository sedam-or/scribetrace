# ScribeTrace Linux End-to-End Test Harness

**Purpose:** Real-window end-to-end integration test harness for ScribeTrace running under Xvfb with `tauri-driver` (WebKitWebDriver).

---

## 1. Architecture & Design Principles

1. **Real Window Testing:**
   Unlike headless unit and IPC contract tests, this harness drives the compiled release desktop application using the standard W3C WebDriver protocol over `tauri-driver`.
2. **Zero-Dependency Budget Compliance:**
   The frontend application bundle maintains a strict budget ($\le 6$ direct JS packages). To respect this invariant, the `e2e/` test harness is isolated in its own folder with its own `package.json`. The custom WebDriver client (`e2e/lib/webdriver.mjs`) is implemented using native Node.js `fetch` with **0 external npm dependencies**.
3. **Stand-in Zotero Server:**
   The harness automatically spawns `test-support/zotero-standin` on `127.0.0.1:23119` serving recorded fixtures from `tests/fixtures/zotero/`. No live Zotero installation is required in CI.
4. **Lightness & Timing Assertions:**
   Asserts cold start latency ($\le 2.0\text{ s}$), timeline opening latency ($\le 1.0\text{ s}$), and measures whole-process-tree idle memory consumption across the main process and WebKitGTK helpers (`WebKitWebProcess`, `WebKitNetworkProcess`) according to the standardized 10 s settle + 5 s median protocol in `docs/TECH-OPTIMIZATION.md` §3.

---

## 2. Directory Layout

```
e2e/
├── package.json               # Isolated e2e configuration (type: module, 0 deps)
├── run-linux.sh               # Portable runner script with xvfb-run wrapping
├── test_walking_skeleton.mjs  # Slice 1 Walking Skeleton real-window test
├── lib/
│   ├── webdriver.mjs          # Zero-dependency W3C WebDriver client
│   ├── harness.mjs            # Stand-in server & tauri-driver process supervisor
│   └── process_tree.mjs       # Whole-process-tree PSS/RSS memory measurement
└── README.md                  # This documentation
```

---

## 3. How to Run Locally

### Prerequisites (Linux)
```bash
sudo apt-get install -y xvfb webkit2gtk-driver libwebkit2gtk-4.1-dev
cargo install tauri-driver --locked
```

### Running the Test
```bash
# 1. Build the release binary
cargo build --release

# 2. Execute the E2E test runner
./e2e/run-linux.sh --release
```

---

## 4. GitHub Actions CI Integration

Wired into GitHub Actions (`.github/workflows/e2e.yml`) running on `ubuntu-latest`:
```yaml
name: Linux E2E (Real Window)
on:
  push:
    branches: ["main"]
  pull_request:
    branches: ["main"]

jobs:
  e2e-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - name: Install WebKitWebDriver & Xvfb
        run: sudo apt-get update && sudo apt-get install -y xvfb webkit2gtk-driver libwebkit2gtk-4.1-dev
      - name: Install tauri-driver
        run: cargo install tauri-driver --locked
      - name: Build ScribeTrace
        run: cargo build --release
      - name: Run E2E Test Suite
        run: ./e2e/run-linux.sh --release
```
