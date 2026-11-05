# ADR 0001: Desktop Application Framework Selection (Tauri v2 vs. Electron)

**Status:** Archived: merged into [ADR 0006](../0006-staged-mvp-and-delivery-shape.md)  
**Date:** 2026-11-05  
**Deciders:** Principal Software Architect, Product Management  
**Consulted:** QA Lead, Systems Engineering  

---

## 1. Context and Problem Statement

ScribeTrace is a cross-platform (Linux, Windows, macOS) desktop reader designed for heavy academic workloads. Researchers frequently keep dozens of complex documents (50–500 page PDFs, EPUB monographs, arXiv HTML preprints) open simultaneously across long reading sessions on standard laptops (reference baseline: 4 CPU cores, 8 GB RAM, SSD).

The application requires:
1. Low memory consumption to avoid swapping when opening multiple large documents.
2. High-throughput, multi-threaded native filesystem access for background cryptographic hashing (SHA-256), recursive workspace directory scanning (1,000+ files in < 3 s), and atomic JSONL file appends.
3. Fast cold startup (< 2 s).
4. Strong security isolation for rendering untrusted third-party HTML/arXiv papers.
5. Simple one-command developer workflow and automated CI builds across Linux (AppImage, deb, Flatpak), Windows (MSI/NSIS), and macOS (Universal binary: Apple Silicon + Intel).

---

## 2. Options Considered

1. **Option A: Tauri v2 (Rust backend + Webview frontend)**
2. **Option B: Electron (Chromium + Node.js)**
3. **Option C: Native GUI (Rust + iced / egui / Slint or Qt/C++)**

---

## 3. Comparative Analysis Matrix

| Evaluation Criteria | Option A: Tauri v2 (Rust + Webview) | Option B: Electron (Chromium + Node.js) | Option C: Native GUI (Rust/Qt) |
| :--- | :--- | :--- | :--- |
| **Idle Memory (1 Window)** | ~30–45 MB | ~180–250 MB | ~15–30 MB |
| **Memory with 10 Open PDFs** | ~180–300 MB | ~1.2–2.0 GB (severe pressure on 8 GB RAM) | ~120–200 MB |
| **Binary Installer Size** | ~10–18 MB | ~85–120 MB | ~15–35 MB |
| **Background Threading (I/O)** | Native Rust threads / `tokio` (Zero GC pauses) | Node.js Worker Threads / C++ add-ons | Native OS threads |
| **Web Tech Compatibility** | 100% (Standard HTML5/CSS3/React/TypeScript) | 100% (Standard HTML5/CSS3/React/TypeScript) | Very low (cannot run PDF.js or foliate-js directly) |
| **Document Reader Libraries** | Reuses PDF.js & foliate-js in Webview | Reuses PDF.js & foliate-js in Chromium | Must bind poppler/mupdf/MuPDF (C/C++ licensing risks) |
| **Security Architecture** | Explicit IPC command whitelist; capability-based | Node integration risks; requires context isolation | Process boundary controls |
| **License Compatibility** | Dual MIT / Apache-2.0 | MIT (with multi-licensed Chromium/Node blobs) | Varies (Qt is LGPL/GPL/Commercial) |

---

## 4. Decision Outcome

**Chosen Option:** **Option A: Tauri v2 (Stable 2.0, Rust + Webview with TypeScript/React).**

### Decision Rationale:
1. **Memory Efficiency:** Tauri v2 relies on the operating system's native webview engine (`WebKitGTK` on Linux, `WebView2` on Windows, and `WKWebView` on macOS) via the `wry` and `tao` crates. This eliminates bundling a full Chromium binary per application instance, reducing idle memory by over 75% compared to Electron. On our target 8 GB RAM reference machine, this guarantees smooth multi-document multitasking without memory thrashing.
2. **Rust Backend for Data Integrity & Performance:**
   * Cryptographic content-hashing (SHA-256) is executed in Rust using vectorized instructions (`sha2` crate), running up to 8x faster than Node.js streams.
   * Workspace filesystem watching uses native OS event queues (`notify` crate) with kernel-level debouncing.
   * Sidecar append-only I/O utilizes native `fsync` and advisory multi-process file locking (`fs2` crate) for rock-solid durability.
3. **Web Frontend for Proven Reader Rendering:** The user interface and document viewports utilize mature web-based rendering engines (PDF.js, foliate-js, and Markdown DOM renderers) rendered inside the webview, avoiding the massive engineering burden and C/C++ dependency risks of building native PDF/EPUB rendering from scratch.
4. **License Alignment:** Tauri v2 is dual-licensed under MIT and Apache-2.0, completely compatible with ScribeTrace's Apache-2.0 license.

---

## 5. Consequences

### Positive:
* Sub-15 MB installer bundles, drastically reducing download friction for academic users.
* Sub-100 ms UI latency and instantaneous window rendering.
* Robust thread safety and memory safety guarantees in the Rust backend for all file operations.
* Clear architectural boundary: Rust handles storage, hashing, search indexing, and OS integration; React/TypeScript handles UI presentation, reading controls, and interactive annotation overlays.

### Negative / Trade-Offs & Mitigations:
* **Webview Engine Discrepancies:** Linux uses WebKitGTK, macOS uses WKWebView, and Windows uses Edge WebView2. Sub-pixel text rendering and minor CSS differences can occur.
  * *Mitigation:* Normalize canvas rendering in PDF.js and avoid browser-specific CSS features. Maintain automated headless rendering tests in CI across all 3 platforms.
* **Rust Toolchain Learning Curve:** Contributors modifying the backend require Rust knowledge.
  * *Mitigation:* Keep the Rust backend modular and focused strictly on the core engine (I/O, hashing, SQLite, IPC). The majority of frontend UI and reader features remain standard TypeScript/React.

---

## 6. Security Impact

* **Elimination of Node.js RCE Surface:** Unlike Electron, where a misconfigured renderer can achieve full operating system shell access via Node.js APIs, Tauri v2's frontend webview has zero direct access to the filesystem, shell, or network sockets.
* **Granular IPC Capability Allowlist:** Communication between the presentation layer and the Rust backend is strictly confined to a type-safe, compile-time capability allowlist (`tauri.conf.json`). Only explicitly registered commands (`workspace:list`, `storage:append_event`, `search:query`) can be invoked.
* **Memory Safety:** The core backend is authored in 100% safe Rust, eliminating buffer overflows, use-after-free, and pointer aliasing vulnerabilities during document parsing and sidecar manipulation.
* **OS Sandboxing:** On macOS and Windows, Tauri applications inherit OS-level app sandbox containers and AppContainer restrictions.

