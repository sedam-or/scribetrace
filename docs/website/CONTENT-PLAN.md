# ScribeTrace Official Website: Content Plan & Technical SEO Specification

**Document Version:** 1.0  
**Date:** 2026-11-05  
**Author:** Growth & Marketing Lead  
**Target Domain:** `https://scribetrace.app` (Fallback: `https://scribetrace-reader.org`)  
**Status:** Approved for Implementation  
**Audience:** Marketing, DevOps, Web Developers, Content Writers, Translators  

---

## 1. Information Architecture & Sitemap

The ScribeTrace official website is architected as a **docs-first, ultra-fast static website** hosted on the maintainer's VPS. It prioritizes instant binary downloads, deep technical credibility, comprehensive documentation, and bilingual accessibility (English primary, Indonesian secondary).

### 1.1 Complete URL Hierarchy

```
https://scribetrace.app/
│
├── en/ (or root / canonical to en)
│   ├── index.html                   # Home: Hero, 20s demo, OS download detection, value pillars
│   ├── features/
│   │   └── index.html               # Features deep-dive: Formats, Annotations, History, Notes, Logs
│   ├── compare/
│   │   └── index.html               # Objective comparison vs Zotero, KOReader, Hypothesis, Obsidian
│   ├── download/
│   │   └── index.html               # Direct downloads for Linux, macOS, Windows + Package managers
│   ├── docs/
│   │   ├── index.html               # Documentation Portal & Quick Start Guide
│   │   ├── format-spec/             # JSONL Sidecar Schema & W3C Anchoring Model
│   │   └── keyboard-shortcuts/      # Power-user shortcuts reference
│   ├── blog/
│   │   ├── index.html               # Blog listing & RSS feed
│   │   ├── introducing-scribetrace/  # Launch post
│   │   └── why-sidecars/            # Deep-dive on non-destructive PDF annotation
│   ├── community/
│   │   └── index.html               # Academic reading groups, lab adoption, governance, Code of Conduct
│   └── sponsor/
│       └── index.html               # Open Collective, GitHub Sponsors, grant roadmap
│
├── id/                              # Indonesian Localization (Full parity)
│   ├── index.html                   # Beranda: Pembaca paper lokal dengan riwayat anotasi abadi
│   ├── features/index.html          # Fitur: Format, Anotasi, Riwayat, Catatan Samping, Log Baca
│   ├── compare/index.html           # Perbandingan: ScribeTrace vs Zotero, KOReader, Hypothesis
│   ├── download/index.html          # Unduh: Linux, macOS, Windows & Panduan Paket
│   ├── docs/index.html              # Dokumentasi & Panduan Memulai Cepat
│   ├── blog/index.html              # Blog & Artikel Ilmiah
│   ├── community/index.html         # Komunitas & Tata Kelola
│   └── sponsor/index.html           # Dukungan & Pendanaan Publik
│
├── sitemap.xml                      # Multilingual sitemap with xhtml:link hreflang annotations
├── robots.txt                       # Search crawler indexing rules
└── site.webmanifest                 # PWA metadata & application icons
```

---

## 2. Page-by-Page Content & Copy Outlines

### 2.1 Home Page (`/` / `/en/index.html`)

#### Header / Navigation Bar
* **Logo:** ScribeTrace layered manuscript icon + logotype.
* **Navigation Links:** Features, Compare, Docs, Blog, Community, GitHub (with live star badge).
* **CTA Button:** `Download Free` (Detects OS automatically).
* **Language Switcher:** `EN` / `ID` dropdown selector.

#### Section 1: Above-the-Fold Hero
* **Tagline Badge:** `Open Source • Local-First • Zero Telemetry • Apache 2.0`
* **H1 Headline:**  
  *Primary:* **The Paper Reader That Remembers How Your Thinking Evolved**  
  *A/B Test Variant:* **Read Research Papers with an Immutable History of Every Note**
* **Sub-headline (Body Lead):**  
  *ScribeTrace is an open-source desktop reader for PDF, EPUB, arXiv HTML, and Markdown. It treats your local folder as a project, preserves every highlight and note in plain sidecar files, and never touches your original documents.*
* **Primary CTAs:**
  * **Dynamic Download Button:** `Download for Linux (.AppImage / Flatpak)` / `Download for macOS (Universal DMG)` / `Download for Windows (.msix)` — *OS detected via `navigator.userAgentData` / `navigator.userAgent`*.
  * **Secondary Link:** `View on GitHub (v1.0.0)` or `Read the Quick Start →`.
* **Social Proof Bar:** *"Trusted by researchers, PhD students, and open-science labs worldwide."* Badges: `No Account Required`, `100% Offline`, `Bit-for-Bit Read-Only Originals`.

#### Section 2: Interactive 20-Second Video / Visual Product Demo
* High-resolution embedded HTML5 video / animated WebP mockup:
  * Shows opening a folder of preprints.
  * Demonstrates highlighting an equation, writing a margin comment, and capturing a quote into the markdown side notes pane.
  * Shows dragging the version slider back in time to reveal how a note looked during the first literature review pass.
  * Shows the generated `.scribetrace/` folder containing transparent, versionable JSON Lines files.

#### Section 3: The Three Core Value Pillars
```
+---------------------------+---------------------------+---------------------------+
|   1. Immutable History    |   2. Bit-for-Bit Sacred   |   3. Local-First & Zero   |
|   Append-Only Event Log   |    Sidecars in Project    |     Forced Telemetry      |
|                           |                           |                           |
| Every highlight, revision,| Your original source PDFs,| No cloud logins, no API   |
| and note deletion is      | EPUBs, and preprints are  | keys, and no telemetry.   |
| preserved. Travel back to | strictly read-only. Markup| Data lives in plain UTF-8 |
| see how your understanding| lives in human-readable   | JSON Lines inside your    |
| deepened across sessions. | `.scribetrace/` sidecars.  | folder. You own it forever|
+---------------------------+---------------------------+---------------------------+
```

#### Section 4: Feature Highlights Grid
* **Multi-Format Reading Engine:** High-performance PDF (virtualized canvas), reflowable EPUB, sandboxed arXiv HTML, and Markdown.
* **Side Notes with Deep Links:** Integrated markdown synthesis pane with 1-click text citation links back to exact page and paragraph anchors.
* **Automated Reading Log:** Accurate active reading duration tracking with a 5-minute inactivity timeout. Know exactly how many hours went into each paper.
* **Conflict-Free Synchronization:** Seamlessly share and sync project folders via Git, Syncthing, or Dropbox. Deterministic Hybrid Logical Clock (HLC) union merge handles offline edits automatically.

#### Section 5: Competitor Comparison Teaser
* A concise 4-row teaser comparing ScribeTrace vs. Zotero 7 and Readwise Reader.
* Link to the complete breakdown: `See how ScribeTrace compares to other research tools →`.

#### Section 6: Bottom Download CTA & Lab Adoption
* **Headline:** Ready to own your research reading?
* **Subhead:** Free, open source, and runs on your computer in under 60 seconds.
* **Buttons:** OS download selector, Homebrew command (`brew install --cask scribetrace`), Winget command (`winget install scribetrace`), and Flatpak command.

---

### 2.2 Features Deep-Dive Page (`/features/`)

* **Hero:** Everything you need to read, critique, and synthesize scholarship without compromise.
* **Pillar 1: Multi-Format Reading Without Document Pollution:**
  * PDF rendering benchmark: Cold-starts 500-page monographs in under 2.0 seconds.
  * EPUB & Sandboxed arXiv HTML: Reflowable text with customizable typography, dark mode, sepia, and responsive margins.
  * Zero original modification guarantee: Cryptographic SHA-256 verification proves original documents remain completely untouched.
* **Pillar 2: The Annotation Engine & W3C Anchoring:**
  * 6 curated annotation colors with keyboard shortcuts.
  * W3C Web Annotation composite selectors: `TextQuoteSelector` (prefix/suffix context) + `TextPositionSelector`.
  * The 4-tier fuzzy resolution algorithm: Never loses a highlight when responsive layouts reflow. Orphan quarantine drawer for unmatched anchors.
* **Pillar 3: The Immutable History Engine:**
  * The append-only event stream architecture (`.jsonl`).
  * Soft-deletion and 1-click state rollback.
  * Named snapshots: Create milestones ("First Skim", "Exam Revision", "Peer Review Rebuttal").
* **Pillar 4: Dual-Layer Side Notes & Session Reading Log:**
  * Dual-layer storage: Versioned history in `.history.jsonl`, projected live state in `.md` for instant opening in Obsidian or VS Code.
  * Active reading timer: Automatic session detection that ignores idle time. Workspace-level reading analytics.
* **Pillar 5: Open Formats & Interoperability:**
  * 1-click Markdown export.
  * Standard W3C JSON-LD export.
  * Flattened annotated PDF export (burning annotations into standard PDF appearance streams without touching the source file).

---

### 2.3 Comprehensive Comparison Page (`/compare/`)

* **Positioning Principle:** Rigorous, fair, respectful, and verifiable against published technical documentation. No deceptive marketing FUD.
* **Comprehensive Comparison Matrix Table:**
  * Directly mirrors Section 4 of `docs/BUSINESS.md` and Section 8 of `docs/REQUIREMENTS.md`.
  * Dimensions: License, Pricing, Storage Architecture, Original File Mutation, Markup History, Workspace Paradigm, Formats Supported, Side Notes & Log, Telemetry.
* **Dedicated Section: ScribeTrace vs. Zotero 7:**
  * *Where Zotero excels:* Excellent citation generation, metadata extraction via CrossRef/DOI, and bibliographic database management.
  * *Where ScribeTrace excels:* Local folder project model, append-only annotation history (Zotero overwrites past states), sidecar files adjacent to papers (Zotero locks notes into `zotero.sqlite`), and integrated reading duration logs.
  * *Verdict:* Use Zotero for citation bibliographies; use ScribeTrace for deep reading and longitudinal paper markup.
* **Dedicated Section: ScribeTrace vs. KOReader:**
  * *Where KOReader excels:* Outstanding e-ink and mobile performance.
  * *Where ScribeTrace excels:* Desktop-native multi-column research interface, dual-layer markdown side notes, W3C Web Annotation standard selectors, and append-only event streams instead of single-state Lua dumps.
* **Dedicated Section: ScribeTrace vs. Hypothesis:**
  * *Where Hypothesis excels:* Collaborative public web page annotations with live social feeds.
  * *Where ScribeTrace excels:* 100% offline, local-first operation on confidential manuscripts, no cloud account dependencies, and folder-based sidecar files.
* **Dedicated Section: ScribeTrace vs. Readwise Reader & MarginNote:**
  * *Where they excel:* Readwise offers broad web feeds and RSS; MarginNote offers 3D mind-mapping on iPad.
  * *Where ScribeTrace excels:* Free forever, open-source Apache 2.0, zero recurring subscription fees, cross-platform (Linux, Windows, macOS), and zero vendor lock-in.

---

### 2.4 Download & Changelog Page (`/download/`)

* **Hero Section:** Download ScribeTrace for your operating system. Current version: `v1.0.0` (Released November 5, 2026).
* **Operating System Cards:**
  * **Linux:**
    * `Download .AppImage` (x86_64 standalone)
    * `Download .deb` (Debian, Ubuntu, Linux Mint)
    * `Flatpak:` `flatpak install flathub app.scribetrace.ScribeTrace`
    * `Arch AUR:` `yay -S scribetrace-bin`
  * **macOS:**
    * `Download Universal DMG` (Apple Silicon M1/M2/M3/M4 & Intel Core)
    * `Homebrew Cask:` `brew install --cask scribetrace`
    * *Notice:* Notarized with Apple Developer ID. Requires macOS 12 Monterey or newer.
  * **Windows:**
    * `Download Windows Installer (.msix)` (Windows 10/11 64-bit)
    * `Download Portable (.zip)`
    * `Winget:` `winget install scribetrace`
    * *Notice:* Digitally signed executable.
* **Verification & Integrity:**
  * Link to `SHA256SUMS.txt` and PGP signatures.
  * Inline instructions for verifying checksums in terminal (`sha256sum -c SHA256SUMS.txt`).
* **Interactive Changelog / Release Feed:**
  * Reverse chronological display parsed from `CHANGELOG.md`.
  * Highlighted features, improvements, and bug fixes for each tag.

---

### 2.5 Documentation Portal (`/docs/`)

* **Section 1: Getting Started in 60 Seconds:**
  * Step 1: Open ScribeTrace and select any folder containing research papers.
  * Step 2: Notice the `.scribetrace/` workspace folder created automatically.
  * Step 3: Highlight your first claim, write a margin note, and open the Notes tab (`Ctrl+\` / `Cmd+\`).
* **Section 2: Workspace Management:**
  * How ScribeTrace tracks document hashes (SHA-256) so files can be renamed or reorganized without losing annotations.
  * Git integration best practices (committing `.scribetrace/` into your research repo).
* **Section 3: Storage Format & W3C Specifications (`/docs/format-spec/`):**
  * Full schema breakdown for `.scribetrace/workspace.json`, `.scribetrace/annotations/<hash>.jsonl`, `.scribetrace/notes/<hash>.history.jsonl`, and reading logs.
  * W3C Web Annotation composite selector definitions (`TextQuoteSelector`, `TextPositionSelector`).
* **Section 4: Keyboard Shortcuts Cheatsheet (`/docs/keyboard-shortcuts/`):**
  * Navigation, zoom, highlight colors (`1`–`6`), note toggling (`Ctrl+\`), and search (`Ctrl+K` / `Cmd+K`).
* **Section 5: Frequently Asked Questions (FAQ):**
  * *Will ScribeTrace alter my PDF files?* Never. Source documents are bit-for-bit read-only.
  * *Can I sync my folder with Dropbox / Syncthing / Git?* Yes. Our append-only union merge algorithm is designed specifically for conflict-free syncing.
  * *Is there an AI summary feature?* No. We keep ScribeTrace focused on pure human comprehension, total privacy, and offline speed.

---

### 2.6 Blog & Editorial Section (`/blog/`)

* **Editorial Vision:** High-signal technical essays on reading philosophy, local-first software architecture, open science, and document standards.
* **Launch Articles (Pre-Drafted):**
  1. `/blog/introducing-scribetrace/`: *"Introducing ScribeTrace: The Paper Reader That Remembers How Your Thinking Evolved."*
  2. `/blog/why-sidecars/`: *"Why We Don't Touch Your PDFs: The Architectural Case for Sidecar Storage."*
  3. `/blog/escaping-the-sqlite-silo/`: *"Escaping the SQLite Silo: Why Research Annotations Belong in Your Project Folder."*
* **Syndication & RSS:** Clean valid RSS 2.0 feed (`/blog/feed.xml`) and Atom feed (`/blog/atom.xml`).

---

### 2.7 Community & Governance Page (`/community/`)

* **Academic Lab Adoption:** Guidelines for incorporating ScribeTrace into lab reading groups and student literature review onboarding.
* **Contributing Guide:** How to clone, build in < 15 minutes (`pnpm install && pnpm dev`), and contribute to the Tauri v2 codebase.
* **Governance Model:** Community-led maintainership, Contributor Covenant v2.1 Code of Conduct, and RFC process for sidecar schema updates.
* **Discussions & Matrix / Discord Outposts:** Direct links to GitHub Discussions, Matrix room `#scribetrace:matrix.org`, and academic Mastodon/Bluesky feeds.

---

### 2.8 Sponsor & Public Goods Funding (`/sponsor/`)

* **Mission:** ScribeTrace is non-extractive open-source software. The core app is 100% free and will never have paywalled features or commercial telemetry.
* **Funding Channels:**
  * **GitHub Sponsors:** Direct support for maintainers.
  * **Open Collective:** Transparent community ledger for code-signing certs, domain infrastructure, and build runners.
* **Institutional Grant Roadmap:** Transparently tracking grant applications to NLnet NGI Zero, Sovereign Tech Fund, Alfred P. Sloan Foundation, and Chan Zuckerberg Initiative EOSS.

---

## 3. Technical SEO Checklist & Architecture

A world-class technical SEO posture guarantees organic search engine indexing, fast Core Web Vitals, rich social preview cards, and zero crawl friction.

### 3.1 Metadata & Title Tags Specification

Search engines prioritize title tags under 60 characters and meta descriptions between 140–160 characters:

| URL Path | `<title>` Tag (< 60 chars) | `<meta name="description">` (140–160 chars) |
| :--- | :--- | :--- |
| `/` | `ScribeTrace | Local-First Research Paper Reader` | *Open-source desktop paper reader for PDF, EPUB, & arXiv HTML. Store immutable annotation history in local sidecars. Free, offline, zero telemetry.* |
| `/features/` | `Features | ScribeTrace Research Paper Annotator` | *Explore ScribeTrace features: multi-format reading, append-only history log, dual-layer markdown side notes, W3C selectors, and reading session logs.* |
| `/compare/` | `Compare ScribeTrace vs Zotero, KOReader & Hypothesis` | *Compare ScribeTrace with Zotero 7, KOReader, Hypothesis, and Readwise. Learn why local-first sidecars and immutable markup history empower researchers.* |
| `/download/` | `Download ScribeTrace Free for Linux, macOS & Windows` | *Download ScribeTrace for Linux (Flatpak, AppImage), macOS (Universal DMG, Homebrew), and Windows (MSIX, Winget). Fast, open source, and offline.* |
| `/docs/` | `Documentation & Quick Start | ScribeTrace Reader` | *Official ScribeTrace documentation. Quick start guide, project workspace configuration, JSONL sidecar format specifications, and keyboard shortcuts.* |
| `/blog/` | `ScribeTrace Blog | Open Science & Local-First Tools` | *Essays on scholarly reading workflows, non-destructive document annotation, W3C selector anchoring, and local-first open-source architecture.* |

---

### 3.2 Schema.org JSON-LD Structured Data

Inject standard `SoftwareApplication` JSON-LD schema into the `<head>` of the home and download pages. This unlocks rich Google Search snippets with rating stars, license info, and direct download links:

```html
<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "SoftwareApplication",
  "name": "ScribeTrace",
  "operatingSystem": "Linux, macOS, Windows 10, Windows 11",
  "applicationCategory": "EducationalApplication, ResearchApplication",
  "offers": {
    "@type": "Offer",
    "price": "0",
    "priceCurrency": "USD"
  },
  "description": "Open-source, local-first research paper reader with immutable annotation history. Preserves highlights, notes, and reading logs in local sidecar files.",
  "softwareVersion": "1.0.0",
  "license": "https://www.apache.org/licenses/LICENSE-2.0",
  "downloadUrl": "https://scribetrace.app/download/",
  "publisher": {
    "@type": "Organization",
    "name": "ScribeTrace Open Source Project",
    "url": "https://scribetrace.app"
  },
  "url": "https://scribetrace.app",
  "sameAs": [
    "https://github.com/scribetrace-app/scribetrace"
  ]
}
</script>
```

---

### 3.3 Open Graph & Twitter Cards Specification

Every page must define standard social metadata for high-converting embeds on Hacker News, Reddit, Twitter/X, Mastodon, Bluesky, Discord, and Slack:

```html
<!-- Open Graph / Facebook / LinkedIn / Slack -->
<meta property="og:type" content="website">
<meta property="og:site_name" content="ScribeTrace">
<meta property="og:title" content="ScribeTrace – Local-First Research Paper Reader">
<meta property="og:description" content="Open-source desktop reader with immutable annotation history. Local sidecars for PDF, EPUB, and arXiv HTML. No accounts, zero telemetry.">
<meta property="og:url" content="https://scribetrace.app/">
<meta property="og:image" content="https://scribetrace.app/assets/og/scribetrace-social-card.png">
<meta property="og:image:width" content="1200">
<meta property="og:image:height" content="630">
<meta property="og:image:alt" content="ScribeTrace paper reader desktop interface displaying PDF paper, highlights, and side notes">

<!-- Twitter Cards -->
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:title" content="ScribeTrace – Local-First Research Paper Reader">
<meta name="twitter:description" content="The paper reader that remembers how your thinking evolved. Open source, offline, zero accounts.">
<meta name="twitter:image" content="https://scribetrace.app/assets/og/scribetrace-social-card.png">
```

---

### 3.4 Multilingual SEO & `hreflang` Architecture

The site supports full bilingual parity: **English (`en`)** and **Indonesian (`id`)**.

To prevent duplicate content penalties and ensure search engines serve the appropriate language in search results, every HTML page must declare self-referencing and reciprocal `hreflang` tags, alongside an `x-default` fallback pointing to English:

```html
<!-- In https://scribetrace.app/en/compare/ -->
<link rel="canonical" href="https://scribetrace.app/en/compare/">
<link rel="alternate" hreflang="en" href="https://scribetrace.app/en/compare/">
<link rel="alternate" hreflang="id" href="https://scribetrace.app/id/compare/">
<link rel="alternate" hreflang="x-default" href="https://scribetrace.app/en/compare/">

<!-- In https://scribetrace.app/id/compare/ -->
<link rel="canonical" href="https://scribetrace.app/id/compare/">
<link rel="alternate" hreflang="en" href="https://scribetrace.app/en/compare/">
<link rel="alternate" hreflang="id" href="https://scribetrace.app/id/compare/">
<link rel="alternate" hreflang="x-default" href="https://scribetrace.app/en/compare/">
```

---

### 3.5 XML Sitemap (`sitemap.xml`) & `robots.txt`

#### Sourced `sitemap.xml` Syntax (Multilingual Engine)
```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xhtml="http://www.w3.org/1999/xhtml">
  <url>
    <loc>https://scribetrace.app/en/</loc>
    <xhtml:link rel="alternate" hreflang="en" href="https://scribetrace.app/en/"/>
    <xhtml:link rel="alternate" hreflang="id" href="https://scribetrace.app/id/"/>
    <xhtml:link rel="alternate" hreflang="x-default" href="https://scribetrace.app/en/"/>
    <lastmod>2026-11-05</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://scribetrace.app/id/</loc>
    <xhtml:link rel="alternate" hreflang="en" href="https://scribetrace.app/en/"/>
    <xhtml:link rel="alternate" hreflang="id" href="https://scribetrace.app/id/"/>
    <xhtml:link rel="alternate" hreflang="x-default" href="https://scribetrace.app/en/"/>
    <lastmod>2026-11-05</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.9</priority>
  </url>
  <url>
    <loc>https://scribetrace.app/en/features/</loc>
    <xhtml:link rel="alternate" hreflang="en" href="https://scribetrace.app/en/features/"/>
    <xhtml:link rel="alternate" hreflang="id" href="https://scribetrace.app/id/features/"/>
    <xhtml:link rel="alternate" hreflang="x-default" href="https://scribetrace.app/en/features/"/>
    <lastmod>2026-11-05</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>https://scribetrace.app/en/compare/</loc>
    <xhtml:link rel="alternate" hreflang="en" href="https://scribetrace.app/en/compare/"/>
    <xhtml:link rel="alternate" hreflang="id" href="https://scribetrace.app/id/compare/"/>
    <xhtml:link rel="alternate" hreflang="x-default" href="https://scribetrace.app/en/compare/"/>
    <lastmod>2026-11-05</lastmod>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>https://scribetrace.app/en/download/</loc>
    <xhtml:link rel="alternate" hreflang="en" href="https://scribetrace.app/en/download/"/>
    <xhtml:link rel="alternate" hreflang="id" href="https://scribetrace.app/id/download/"/>
    <xhtml:link rel="alternate" hreflang="x-default" href="https://scribetrace.app/en/download/"/>
    <lastmod>2026-11-05</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.9</priority>
  </url>
  <url>
    <loc>https://scribetrace.app/en/docs/</loc>
    <xhtml:link rel="alternate" hreflang="en" href="https://scribetrace.app/en/docs/"/>
    <xhtml:link rel="alternate" hreflang="id" href="https://scribetrace.app/id/docs/"/>
    <xhtml:link rel="alternate" hreflang="x-default" href="https://scribetrace.app/en/docs/"/>
    <lastmod>2026-11-05</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>
</urlset>
```

#### Production `robots.txt`
```text
User-agent: *
Allow: /
Disallow: /api/
Disallow: /internal/

Sitemap: https://scribetrace.app/sitemap.xml
```

---

### 3.6 Core Web Vitals & Performance Engineering

As a desktop tool advocating performance, ScribeTrace's website must demonstrate elite web engineering:

```
+---------------------------------------------------------------------------------------------------+
|                                   CORE WEB VITALS TARGET THRESHOLDS                               |
+--------------------------+--------------------------+-----------------------+---------------------+
|  LCP (Largest Paint)     |  INP (Interaction Next)  |  CLS (Layout Shift)   |  Total Page Weight  |
|  < 1.0 second (P95)      |  < 50 milliseconds       |  0.00 (Zero Shift)    |  < 200 KB (HTML/CSS)|
+--------------------------+--------------------------+-----------------------+---------------------+
```

1. **Zero Client-Side JavaScript Hydration for Content:** Generate pure static HTML (via VitePress, Astro, or Zola). Articles, comparison tables, and copy render without waiting for client-side JavaScript execution.
2. **System Typography & Modern Font Subsetting:** Use modern system font stacks (`system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif`) to eliminate render-blocking Google Font requests.
3. **Responsive Image Optimization:** All product screenshots and diagram assets authored in next-gen `.webp` and `.avif` with explicit `width` and `height` attributes to eliminate layout shifts (CLS 0.00).
4. **Fast Static Server Caching & Compression:** Gzip and Brotli compression enabled at the reverse proxy (Nginx / Caddy), serving static assets with immutable cache-control headers (`Cache-Control: public, max-age=31536000, immutable`).

---

## 4. Indonesian Localization (Bahasa Indonesia) Spec

In alignment with project requirements, Indonesian is a tier-1 first-class localization:

### 4.1 Key Terminology Translation Dictionary

| English Concept | Bahasa Indonesia Translation | Contextual Usage Note |
| :--- | :--- | :--- |
| **Paper Reader** | **Pembaca Makalah Ilmiah / Paper** | "Paper" is widely adopted in Indonesian academic circles; "makalah ilmiah" provides formal context. |
| **Annotation History** | **Riwayat Anotasi Abadi** | "Abadi" (immutable/eternal) conveys the append-only, non-destructive concept. |
| **Sidecar File** | **Berkas Sidecar (Berdampingan)** | Technical term retained; clarified as files stored directly alongside papers. |
| **Local-First** | **Utamakan Lokal (Local-First)** | Retain the recognized English movement name in parentheses. |
| **Append-Only Event Log** | **Log Peristiwa Hanya-Tambah** | Clear, precise computer science terminology. |
| **Side Notes** | **Catatan Samping** | Dedicated margin panel. |
| **Reading Session Log** | **Log Sesi Membaca** | Duration and timeline tracking. |
| **Zero Telemetry** | **Bebas Telemetri / Tanpa Pelacakan** | Clear user privacy guarantee. |

### 4.2 Hero Translation Sample (Bahasa Indonesia)

* **H1:** **Pembaca Paper Ilmiah yang Mengabadikan Setiap Jejak Pemikiran Anda**
* **Sub-headline:**  
  *ScribeTrace adalah aplikasi desktop sumber terbuka (open-source) untuk membaca PDF, EPUB, arXiv HTML, dan Markdown. ScribeTrace memperlakukan folder lokal Anda sebagai proyek mandiri, menyimpan riwayat sorotan dan catatan dalam berkas sidecar yang transparan, tanpa pernah mengubah berkas dokumen asli.*
* **CTA Button:** `Unduh Gratis untuk Linux / macOS / Windows`

---

## 5. Summary of Technical Requirements for DevOps

The official website and docs platform on the maintainer's VPS must support the following technical requirements:
1. **Static Site Generator (SSG):** VitePress, Astro, or Zola producing clean static HTML files.
2. **Clean URL Routing:** Automatic extensionless routing (e.g. `/compare/` resolving to `/compare/index.html`).
3. **Locale Routing:** Supporting `/` (canonical to `/en/`) and `/id/` with root auto-redirect based on `Accept-Language` header (while respecting manual switcher cookie/localStorage).
4. **Server Headers:**
   * Strict CSP: `default-src 'self'; img-src 'self' data: https:; script-src 'self'; style-src 'self' 'unsafe-inline'`.
   * Security Headers: `X-Frame-Options: DENY`, `X-Content-Type-Options: nosniff`, `Referrer-Policy: strict-origin-when-cross-origin`.
   * Fast Caching: Immutable cache headers for `/assets/*`.
5. **SEO Assets:** Direct serving of `/sitemap.xml` and `/robots.txt` at the root domain.
6. **Privacy Analytics:** Integration with Plausible or Umami (no tracking cookies, zero personal data storage).
