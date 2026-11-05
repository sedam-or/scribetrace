# ADR 0002: Multi-Format Document Rendering Engine Architecture

**Status:** Archived, superseded by [ADR 0011](../0011-reader-pdf-markdown-and-anchoring.md) for the MVP (PDF.js and Markdown kept; EPUB and HTML sections are deferred, kept here as history)  
**Date:** 2026-11-05  
**Deciders:** Principal Software Architect, Product Management  
**Consulted:** QA Lead  

---

## 1. Context and Problem Statement

ScribeTrace must render four distinct document formats in a unified, distraction-free reading interface (Requirement F2.1):
1. **PDF:** Fixed-layout, paginated, complex typography, equations, multi-column layouts (up to 500 pages / 50 MB, rendering Page 1 in < 2.0 s per F2.3).
2. **EPUB:** Reflowable e-books and monographs with fluid typography, Chapter TOC, and Canonical Fragment Identifiers (CFI).
3. **HTML / arXiv HTML:** Single-page preprints containing arbitrary MathJax/KaTeX formulas, tables, and potentially untrusted third-party scripts.
4. **Markdown / Plain Text:** Code blocks, headings, lists, frontmatter, and plain prose.

The challenge is to provide a single, unified reader shell (zoom, dark mode, navigation, find-in-document, and annotation overlays) while delegating viewport rendering to format-specialized engines without licensing conflicts or security holes.

---

## 2. Decision per Format

### 2.1 PDF Rendering: Mozilla PDF.js (v4.x)
* **Chosen Library:** **Mozilla PDF.js** ([`mozilla/pdf.js`](https://github.com/mozilla/pdf.js))
* **License:** **Apache License 2.0** (100% compatible with ScribeTrace)
* **Alternatives Evaluated:**
  * *Native Poppler / MuPDF (C/C++ via Rust FFI):* High rendering speed, but introduces GPL licensing constraints (MuPDF is AGPLv3; Poppler is GPLv2/v3), cross-compilation headaches across 3 OSes, and memory corruption vulnerabilities from malicious PDF exploits.
  * *PDFium (Chromium's PDF engine):* High fidelity, but heavy C++ binary dependencies and complex IPC canvas streaming.
* **Why PDF.js Wins:**
  1. Mature, standard-compliant PDF parsing written in pure JavaScript, executing entirely within the webview sandbox.
  2. Built-in support for streaming chunked range requests (F2.3), enabling immediate rendering of Page 1 without loading the full 50 MB payload into RAM.
  3. Clean text layer (`TextLayerBuilder`) emitting standard DOM spans that map directly to W3C text selections and SVG annotation overlays.

### 2.2 EPUB Rendering: foliate-js
* **Chosen Library:** **`foliate-js`** by John Factotum ([`johnfactotum/foliate-js`](https://github.com/johnfactotum/foliate-js))
* **License:** **MIT License** (Standalone library is MIT, using vendored `zip.js` BSD-3-Clause and `fflate` MIT)
* **Alternatives Evaluated:**
  * *`futurepress/epub.js` (BSD-2-Clause):* Historically the standard web EPUB library, but suffers from abandoned maintenance, severe memory leaks with pagination, poor CFI recalculation during reflow, and fragile iframe resizing.
* **Why foliate-js Wins:**
  1. Modern, lightweight (~150 KB), zero-dependency core using web standards (Custom Elements, CSS Column layout, Fetch API).
  2. High-performance paginator and virtualized continuous scroll.
  3. First-class EPUB Canonical Fragment Identifier (CFI) generator and resolver, crucial for stable W3C text anchoring (F3.4).
  4. Native support for dark mode, custom typography injection, and seamless DOM text selection.

### 2.3 HTML / arXiv HTML Rendering: Sandboxed Iframe with Strict CSP
* **Rendering Architecture:** Custom sandboxed `<iframe>` embedded in the reader container.
* **Security & Isolation Enforcement:**
  * The iframe is declared with `<iframe sandbox="allow-same-origin">` (omitting `allow-scripts`, `allow-popups`, `allow-modals`, `allow-forms`).
  * Scripts are strictly disabled at the engine level.
  * Outbound network isolation: Injected Content Security Policy:
    ```http
    Content-Security-Policy: default-src 'none'; style-src 'unsafe-inline'; img-src data: blob:; font-src data:; script-src 'none'; connect-src 'none';
    ```
  * MathJax/KaTeX formulas are pre-rendered into static SVG/MathML at import or during background ingestion, preventing arbitrary code execution.
* **Why This Approach Wins:** Eliminates zero-day browser exploits from downloaded preprints and enforces strict zero-network telemetry compliance (Constitution Rule 4 & NFR Security).

### 2.4 Markdown & Plain Text: Unified AST Pipeline (`remark` + `rehype` + `DOMPurify`)
* **Chosen Libraries:** **`remark-parse`**, **`remark-gfm`**, **`rehype-stringify`**, and **`DOMPurify`** (all MIT-licensed).
* **Architecture:** Markdown is parsed into an abstract syntax tree (mdast), transformed to hast, sanitized with DOMPurify, and rendered as semantic HTML elements.
* **Why This Approach Wins:** Deterministic node structure where headings, paragraphs, and list items have deterministic DOM IDs (`data-block-id`), facilitating robust text anchoring and deep-linking from notes (F7.4).

---

## 3. Unified Reader Viewport Abstraction

All four engines implement a standard TypeScript interface (`DocumentRenderer`):

```typescript
export interface DocumentViewport {
  // Navigation & Positioning
  scrollToPosition(pos: DocumentPosition): Promise<void>;
  getCurrentPosition(): DocumentPosition;
  
  // Selection & Anchoring
  getSelectedRange(): AnchorSelection | null;
  resolveAnchor(target: AnchorTarget): ResolvedDOMRange | null;
  
  // Search & Chrome
  findInDocument(query: string, options: SearchOptions): Promise<SearchMatch[]>;
  setTheme(theme: 'light' | 'dark' | 'sepia'): void;
  setZoom(zoomFactor: number): void;
}
```

---

## 4. Consequences

### Positive:
* 100% open-source, permissive licensing (Apache-2.0 and MIT throughout).
* Memory usage remains bounded under 100 MB per open document view.
* Complete isolation and security when viewing untrusted academic papers.
* Uniform annotation capabilities (highlight, comment, diff) layered transparently above all four document formats.

### Negative & Mitigations:
* *Complex EPUB reflow handling:* When the window resizes, text nodes reflow across virtual columns.
  * *Mitigation:* Decouple annotation rendering from physical pixels: store CFI + W3C TextQuote selectors and dynamically re-calculate DOM bounding client rects on viewport resize events.

---

## 5. Security Impact

* **Hostile Document Isolation:** Academic papers downloaded from web repositories (arXiv, open preprints) may contain malicious JavaScript, tracking pixels, or exploit payloads targeting webview engines.
* **Strict CSP Enforcement:** The HTML and EPUB reader viewports enforce an immutable Content Security Policy:
  `default-src 'none'; style-src 'unsafe-inline'; img-src data: blob:; font-src data:; script-src 'none'; connect-src 'none'; frame-src 'none';`
  Scripts are categorically disabled at the webview level. Outbound network telemetry or tracking pixels cannot execute.
* **Zip-Bomb & Decompression Defense (EPUB):** EPUB container extraction in `foliate-js` enforces strict safety limits: maximum uncompressed file size (100 MB), maximum total archive size (500 MB), and a maximum compression ratio of 100:1 to thwart zip-bomb denial-of-service attacks.
* **Parser Sandboxing:** PDF.js runs inside an isolated Web Worker; a crash in the PDF rendering engine is caught gracefully, presenting an error state without crashing the ScribeTrace desktop application.

