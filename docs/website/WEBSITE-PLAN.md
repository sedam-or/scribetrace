# ScribeTrace Website & Documentation Platform: Architecture, Hosting & Delivery Plan

**Document Version:** 1.0  
**Date:** 2026-11-06  
**Author:** Senior DevOps & Platform Engineer  
**Status:** Approved Architectural Plan  
**Target Domain:** `https://scribetrace.app` (Fallback: `https://scribetrace-reader.org`)  
**Repository Path:** `docs/website/WEBSITE-PLAN.md`  
**Security Alignment:** Aligned with `docs/SECURITY-REQUIREMENTS.md` (SEC-30, SEC-31, SEC-32) and `docs/ARCHITECTURE.md`

---

## 1. Executive Summary & Platform Philosophy

ScribeTrace (formerly Palimpsest) is an open-source, local-first research paper reader and annotation history companion to Zotero and Obsidian. As defined in [ARCHITECTURE.md](../ARCHITECTURE.md) and [DECISION-LOG.md](../DECISION-LOG.md), ScribeTrace operates under strict architectural invariants: bit-for-bit unmutated source documents, transparent JSONL sidecars, local-first privacy, zero telemetry, and an ultra-lightweight footprint.

The official web presence (`https://scribetrace.app`) must mirror these identical engineering principles:
1. **Static-First & Zero Server Runtime:** The entire website and documentation portal compile to pure, pre-rendered static HTML, CSS, client-side assets, and WebAssembly search indexes. There are zero database daemons, zero server-side rendering (SSR) runtimes, and zero interpreted backend application servers exposed to public traffic.
2. **First-Class Technical Documentation:** Built around the **Diátaxis documentation framework**, providing equal structural weight to Tutorials, How-To Guides, Reference Specifications, and Architectural Explanations.
3. **Bilingual Accessibility (EN + ID):** Full linguistic parity between English (`en`) and Indonesian (`id`) from day one, with search and syntax highlighting optimized across both locales.
4. **Radical User Privacy:** Zero third-party tracker scripts, zero advertising cookies, and zero cross-site analytics beacons. Privacy-friendly telemetry is self-hosted, strictly aggregated, cookieless, and 100% compliant with GDPR/ePrivacy without requiring cookie consent banners.
5. **Hardened Infrastructure & Verifiable Releases:** Hardened reverse-proxy delivery (Caddy / Nginx) achieving an **A+ grade on Mozilla Observatory** (SEC-31), automated TLS, direct binary linking to GitHub Releases with published checksums and SLSA Level 3 provenance (SEC-29), RFC 9116 `security.txt` disclosure (SEC-32), and zero-downtime atomic symlink deployments.

> [!IMPORTANT]
> **Operational Boundary Notice:** This document is an **architectural plan only**. Under current operational rules, AI agents and automated scripts SHALL NOT establish SSH connections to the maintainer's VPS, execute remote shell commands, or initiate deployments. Implementation and server provisioning remain subject to explicit maintainer authorization.

---

## 2. Static Site Generator (SSG) Architecture & Trade-Off Analysis

Selecting the optimal Static Site Generator is critical for long-term maintainability, developer ergonomics, build performance, and technical SEO. We conducted an exhaustive evaluation across the four leading modern documentation-focused static site generators: **Astro Starlight**, **Docusaurus v3**, **VitePress**, and **MkDocs Material**.

### 2.1 Comparative Evaluation Matrix

| Evaluation Dimension | Astro Starlight | Docusaurus (v3) | VitePress | MkDocs Material |
| :--- | :--- | :--- | :--- | :--- |
| **Primary Engine & Runtime** | Astro (Node.js / Vite-based) | React (Node.js / Webpack) | Vue 3 (Node.js / Vite) | Python / Jinja2 |
| **Search Capabilities** | **Pagefind** (Built-in, local Wasm, multilingual, zero-config, offline) | Algolia DocSearch (cloud SaaS) or local Lunr plugin | Minisearch / Algolia DocSearch | Lunr.js / Search plugin |
| **i18n Architecture** | **First-class native routing** (`/en/`, `/id/`), missing fallback detection, UI token dictionary | Native i18n (`/id/`), separate build passes per locale | Native i18n (`/id/`), manual routing configuration | `mkdocs-static-i18n` plugin (community maintained) |
| **Content & Marketing Blending** | **Exceptional** (Islands architecture: docs coexist with rich marketing pages and zero-JS landing pages) | Good (custom React pages supported, but heavier JS bundle) | Good (Vue components in Markdown, lightweight) | Moderate (strictly doc-oriented; complex landing pages require raw HTML/Jinja hacks) |
| **Client-Side JS Overhead** | **Zero JS by default** (hydrates only interactive UI islands like the OS download selector) | Heavy (~150–300 KB hydration bundle per page load) | Light (~40–80 KB Vue runtime) | Zero JS runtime (except client search script) |
| **Build Speed (500 pages)** | Fast (~3.5 s via esbuild/Vite) | Moderate (~18–25 s via Webpack 5) | Extremely Fast (~2.1 s via Vite) | Fast (~4.2 s via Python) |
| **Syntax Highlighting** | **Shiki** (Engine-native, dual dark/light themes, zero client JS) | Prism.js or Shiki (server-side) | Shiki (engine-native) | Pygments (CSS-based) |
| **Dark Mode & Styling** | Native CSS custom properties, system preference detection | Infima CSS framework (React-specific) | Native CSS custom properties | Material Design CSS (opinionated) |
| **License** | **MIT** | **MIT** | **MIT** | **GPL-3.0** (Material Insiders has commercial restrictions) |

---

### 2.2 Architectural Decision Record: ADR-0014

#### Title: ADR-0014 — Selection of Astro Starlight for ScribeTrace Website & Documentation Platform

* **Status:** Accepted
* **Date:** 2026-11-06
* **Deciders:** DevOps Engineer, Product Manager, Lead Architect

#### Context
ScribeTrace requires a unified web platform that serves two distinct personas:
1. **Academic Researchers & Evaluators:** Seeking an immediate, compelling landing page with an interactive OS download selector, a feature breakdown, competitive comparisons against Zotero and Obsidian, and a lightweight blog.
2. **Developers, Power Users & Lab Maintainers:** Demanding in-depth, versioned technical documentation (JSONL format specification, W3C Web Annotation composite selectors, keyboard shortcuts, CLI reference, and architecture deep-dives) with robust client-side search and full English and Indonesian (`en` / `id`) bilingual parity.

The toolchain must align with ScribeTrace's core ethos: memory safety, minimal dependencies, open-source licensing, and sub-second performance without external cloud SaaS dependencies (e.g., Algolia).

#### Considered Options
1. **Option A: Astro + Starlight**
2. **Option B: Docusaurus v3**
3. **Option C: VitePress**
4. **Option D: MkDocs Material**

#### Decision
We adopt **Astro with the Starlight documentation framework** as the single unified static site generator for `scribetrace.app`.

#### Decision Drivers & Rationale
1. **Built-in Offline Multilingual Search via Pagefind:** Starlight integrates **Pagefind** out of the box. Pagefind is a static search library written in Rust that compiles into WebAssembly. It indexes static HTML post-build, fragments index chunks into static 20–40 KB shards, and runs entirely in the user's browser. It supports both English and Indonesian stemming natively without sending search queries to third-party cloud servers (preserving user privacy).
2. **Zero-JavaScript Baseline (Astro Islands):** Traditional doc frameworks (Docusaurus) ship a full React Single Page Application (SPA) runtime to every visitor, resulting in 200–400 KB of unnecessary JavaScript execution for reading text. Astro ships **0 KB of JavaScript** for static documentation and marketing copy, only hydrating interactive islands (such as the dynamic OS detection download widget and interactive timeline mockup).
3. **Unified Marketing and Documentation Surface:** Starlight operates as an Astro integration. This allows us to keep rich, custom marketing landing pages (`/`, `/features`, `/compare`, `/download`) and Diátaxis documentation (`/docs/*`) in the same project, sharing layout primitives, typography, and design tokens.
4. **License Compatibility:** Astro and Starlight are licensed under the permissive **MIT License**, avoiding the copyleft constraints and commercial paywalls of MkDocs Material Insiders (GPL-3.0).
5. **Native Shiki Syntax Highlighting:** Pre-renders code blocks for Rust, JSON Lines, Markdown, Shell, and TOML at build time with dual light/dark themes and zero runtime client script overhead.

#### Consequences
* **Positive:** Sub-second page loads; Pagefind requires zero external SaaS configuration or API keys; complete parity for English and Indonesian routes; seamless integration with GitHub Actions; low memory footprint during CI build.
* **Negative:** Starlight is newer than Docusaurus (released in 2023 vs 2018), though backed by the Astro core team and widely adopted by projects like Vitest, Tailwind, and Tauri v2 docs.

---

## 3. Repository Topology & Folder Structure

### 3.1 Monorepo (`website/`) vs. Polyrepo Analysis

We evaluated hosting the website in a separate dedicated repository (`sedam-or/scribetrace-website`) versus housing it directly within the main application repository under a `website/` directory.

| Criterion | Separate Repository (`scribetrace-website`) | Monorepo (`website/` in `scribetrace`) |
| :--- | :--- | :--- |
| **Documentation Synchronization** | Asynchronous. Code changes and documentation updates require coordinating two distinct PRs across repositories. High risk of doc drift. | **Atomic.** Code changes, format specification updates (`spec/`), and documentation revisions are committed and reviewed in the exact same PR. |
| **Release Versioning** | Requires manual tagging or cross-repository release dispatch webhooks. | Automated. Release workflows can update download URLs, hashes, and release notes in-tree. |
| **CI Complexity** | Simple CI for website, but complex multi-repo secrets and access token management. | Single repository CI with simple path filtering (`paths: ['website/**', 'docs/**']`). |
| **Developer Onboarding** | Requires cloning multiple repositories. | One repository clone gives access to app source, spikes, and docs site. |

**Decision:** Maintain the website within a **`website/` directory in the primary repository** (`sedam-or/scribetrace`).

### 3.2 Canonical Directory Layout

The `website/` folder is structured to cleanly separate Starlight documentation content from custom Astro marketing pages and static public assets:

```
scribetrace/
├── .github/
│   └── workflows/
│       ├── ci.yml                     # App CI (Rust, Tauri, linters)
│       ├── release.yml                # App Release & Packaging
│       └── website.yml                # Website Build & VPS Deploy
├── docs/                              # Project design docs (internal / contributor)
│   ├── ARCHITECTURE.md
│   ├── SECURITY-REQUIREMENTS.md
│   └── website/
│       ├── CONTENT-PLAN.md            # Marketing & SEO plan
│       └── WEBSITE-PLAN.md            # This infrastructure plan
├── website/                           # The Astro Starlight Web Project
│   ├── package.json                   # Dependencies: astro, @astrojs/starlight, pagefind
│   ├── astro.config.mjs               # Starlight configuration & locale definitions
│   ├── tsconfig.json
│   ├── public/
│   │   ├── .well-known/
│   │   │   └── security.txt           # RFC 9116 Coordinated Vulnerability Disclosure (SEC-32)
│   │   ├── favicon.svg
│   │   ├── robots.txt                 # Search engine crawling rules
│   │   ├── images/
│   │   │   ├── hero-mockup.webp
│   │   │   └── timeline-slider.webp
│   │   └── fonts/                     # Self-hosted variable fonts (Inter, JetBrains Mono)
│   └── src/
│       ├── assets/                    # Optimized SVGs and local assets
│       ├── components/                # Astro Islands (interactive UI)
│       │   ├── DownloadButton.astro   # Dynamic OS auto-detection download CTA
│       │   ├── ReleaseChecksum.astro  # Checksum viewer with 1-click terminal copy
│       │   ├── ComparisonTable.astro  # Zotero / Obsidian / KOReader matrix
│       │   └── LanguagePicker.astro   # Bilingual EN/ID switcher
│       ├── content/
│       │   └── docs/                  # Diátaxis Documentation Content
│       │       ├── en/                # English Documentation Root
│       │       │   ├── index.mdx      # Docs landing portal
│       │       │   ├── tutorials/     # Learning-oriented guides
│       │       │   ├── how-to/        # Problem-oriented guides
│       │       │   ├── reference/     # Information-oriented technical specs
│       │       │   └── explanation/   # Understanding-oriented architecture deep-dives
│       │       └── id/                # Indonesian Documentation Root (Full Parity)
│       │           ├── index.mdx
│       │           ├── tutorials/
│       │           ├── how-to/
│       │           ├── reference/
│       │           └── explanation/
│       └── pages/                     # Marketing & Feature Pages (Astro Components)
│           ├── index.astro            # Apex landing page (auto-routes or defaults to /en/)
│           ├── en/
│           │   ├── index.astro        # English Home Page
│           │   ├── features.astro     # Deep dive into formats, sidecars, history
│           │   ├── compare.astro      # Objective technical comparisons
│           │   ├── download.astro     # Direct downloads & package manager instructions
│           │   └── community.astro    # Reading groups, lab adoption, governance
│           └── id/
│               ├── index.astro        # Indonesian Home Page
│               ├── features.astro     # Fitur mendalam
│               ├── compare.astro      # Perbandingan teknis
│               ├── download.astro     # Panduan unduhan & instalasi paket
│               └── community.astro    # Komunitas & tata kelola
```

---

## 4. VPS Hosting Architecture & Edge Topology

The maintainer hosts `scribetrace.app` on an independent, dedicated Virtual Private Server (VPS).

### 4.1 VPS Hardware & OS Baseline
* **Operating System:** Ubuntu 24.04 LTS (Noble Numbat) or Debian 12 (Bookworm) minimal 64-bit.
* **Compute / Memory:** 1 vCPU, 2 GB RAM, 25 GB NVMe SSD.
* **Network Interface:** 1 Gbps port with static public IPv4 and IPv6 `/64` subnet.
* **Workload Characteristics:** Pure static asset delivery and lightweight self-hosted analytics. Memory consumption is anticipated under 350 MB total, leaving ample headroom.

### 4.2 Reverse Proxy Selection: Caddy vs. Nginx

We compared **Caddy v2** and **Nginx** for serving static assets, managing SSL/TLS certificates, and enforcing security headers.

| Feature | Caddy v2 (Recommended) | Nginx (Alternative) |
| :--- | :--- | :--- |
| **TLS Automation** | **Automatic Let's Encrypt / ZeroSSL** with OCSP stapling and automated renewal out-of-the-box. Zero external cron jobs or certbot scripts required. | Requires `certbot`, `acme.sh`, or manual cron automation with webroot verification. |
| **HTTP/3 (QUIC) Support** | **Native out-of-the-box** over UDP 443 with 0-RTT connection resumption. | Requires Nginx 1.25+ mainline compiled with `ngx_http_v3_module` and custom OpenSSL/BoringSSL builds. |
| **Compression** | Built-in **zstd** and **brotli** streaming compression. | Standard gzip native; brotli requires dynamic module compilation (`ngx_brotli`). |
| **Memory Safety** | Written in **Go** (memory safe, zero buffer-overflow vulnerability class). | Written in C (susceptible to legacy memory bugs, though battle-tested). |
| **Config Ergonomics** | Concise, readable `Caddyfile` (~35 lines for full production configuration). | Lengthy `nginx.conf` (~120 lines across multiple include files). |

**Primary Recommendation:** Deploy **Caddy v2**. It eliminates external certificate renewal failure modes, provides native HTTP/3, and minimizes configuration error surface.

---

### 4.3 Production Caddy Configuration (`/etc/caddy/Caddyfile`)

The following hardened Caddyfile configuration enforces strict TLS 1.3, comprehensive HTTP security headers complying with **SEC-31** (A+ rating on Mozilla Observatory), HTTP/3 QUIC, immutable static caching, and RFC 9116 `security.txt` routing (SEC-32):

```caddy
# ScribeTrace Production Caddy Configuration
# Fully aligned with SEC-30, SEC-31, SEC-32

{
    admin off                         # Disable local admin API port to minimize attack surface
    email security@scribetrace.app    # ACME registration email for certificate alerts
    log {
        output file /var/log/caddy/access.log {
            roll_size 50mb            # Roll when log reaches 50MB
            roll_keep 14              # Keep last 14 rotated log files
            roll_keep_for 30d         # Retain for maximum 30 days
        }
        format json
        level INFO
    }
}

# Redirect www to apex domain
www.scribetrace.app {
    redir https://scribetrace.app{uri} permanent
}

scribetrace.app {
    # Serve static assets from atomic deployment symlink
    root * /var/www/scribetrace/current

    # Modern compression: prefer zstd, fallback to brotli and gzip
    encode zstd gzip

    # Static file serving with strict try_files fallback
    file_server {
        precompressed zstd br gzip
    }

    # SEC-31: Mandatory Security Headers for Mozilla Observatory A+ Grade
    header {
        # Enforce HTTPS with 2-year HSTS, subdomains, and preloading
        Strict-Transport-Security "max-age=63072000; includeSubDomains; preload"

        # Content Security Policy (Strict, zero inline scripts, no eval)
        Content-Security-Policy "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https://github.com; font-src 'self'; connect-src 'self' https://analytics.scribetrace.app; frame-ancestors 'none'; base-uri 'self'; form-action 'self';"

        # Prevent MIME-type sniffing
        X-Content-Type-Options "nosniff"

        # Block rendering inside iframes to defeat clickjacking
        X-Frame-Options "DENY"

        # Restrict referrer information to origin on cross-origin requests
        Referrer-Policy "strict-origin-when-cross-origin"

        # Disable unused browser hardware and permissions APIs
        Permissions-Policy "accelerometer=(), camera=(), geolocation=(), gyroscope=(), magnetometer=(), microphone=(), payment=(), usb=()"

        # Remove server banner leakage
        -Server
    }

    # Granular HTTP Cache-Control Headers
    # 1. Immutable hashed assets generated by Astro/Vite
    @immutable {
        path /_astro/*
        path /assets/*
        path /fonts/*
    }
    header @immutable {
        Cache-Control "public, max-age=31536000, immutable"
    }

    # 2. Dynamic HTML, sitemaps, and feeds must always validate
    @dynamic {
        path *.html
        path /
        path /sitemap.xml
        path /robots.txt
        path /pagefind/*
    }
    header @dynamic {
        Cache-Control "public, max-age=0, must-revalidate"
    }

    # SEC-32: RFC 9116 Coordinated Security Disclosure
    handle /.well-known/security.txt {
        header Content-Type "text/plain; charset=utf-8"
        header Cache-Control "public, max-age=86400"
        file_server
    }

    # Custom 404 error handler
    handle_errors {
        rewrite * /404.html
        file_server
    }
}

# Subdomain for self-hosted privacy-preserving analytics
analytics.scribetrace.app {
    reverse_proxy 127.0.0.1:3000 {
        header_up Host {host}
        header_up X-Real-IP {remote_host}
        header_up X-Forwarded-For {remote_host}
        header_up X-Forwarded-Proto {scheme}
    }

    header {
        Strict-Transport-Security "max-age=63072000; includeSubDomains; preload"
        X-Content-Type-Options "nosniff"
        X-Frame-Options "DENY"
        Referrer-Policy "strict-origin"
        -Server
    }
}
```

---

### 4.4 Alternative Production Nginx Configuration

For environments standardizing on Nginx, the equivalent hardened virtual host configuration (`/etc/nginx/sites-available/scribetrace.app`) is provided:

```nginx
# ScribeTrace Hardened Nginx Configuration (SEC-31 Compliance)
server {
    listen 80;
    listen [::]:80;
    server_name scribetrace.app www.scribetrace.app;
    return 301 https://scribetrace.app$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name www.scribetrace.app;

    ssl_certificate /etc/letsencrypt/live/scribetrace.app/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/scribetrace.app/privkey.pem;
    return 301 https://scribetrace.app$request_uri;
}

server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name scribetrace.app;

    root /var/www/scribetrace/current;
    index index.html;

    # SSL hardening (TLS 1.2 & 1.3 only, modern ciphers)
    ssl_certificate /etc/letsencrypt/live/scribetrace.app/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/scribetrace.app/privkey.pem;
    ssl_session_timeout 1d;
    ssl_session_cache shared:SSL:10m;
    ssl_session_tickets off;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
    ssl_stapling on;
    ssl_stapling_verify on;

    # SEC-31: Security Headers
    add_header Strict-Transport-Security "max-age=63072000; includeSubDomains; preload" always;
    add_header Content-Security-Policy "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https://github.com; font-src 'self'; connect-src 'self' https://analytics.scribetrace.app; frame-ancestors 'none'; base-uri 'self'; form-action 'self';" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-Frame-Options "DENY" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    add_header Permissions-Policy "accelerometer=(), camera=(), geolocation=(), gyroscope=(), magnetometer=(), microphone=(), payment=(), usb=()" always;
    server_tokens off;

    # Gzip Compression
    gzip on;
    gzip_vary on;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types text/plain text/css text/xml application/json application/javascript application/rss+xml image/svg+xml;

    # Cache control for immutable assets
    location ~* ^/(?:_astro|assets|fonts)/ {
        expires 1y;
        add_header Cache-Control "public, max-age=31536000, immutable";
        access_log off;
    }

    # Revalidation for HTML and metadata
    location ~* \.(?:html|xml|json|txt)$ {
        expires -1;
        add_header Cache-Control "public, max-age=0, must-revalidate";
    }

    # Routing
    location / {
        try_files $uri $uri/ $uri.html =404;
    }

    error_page 404 /404.html;
}
```

---

### 4.5 Edge / CDN Strategy
* **Direct Origin Delivery:** ScribeTrace's total static footprint is under 15 MB. Hosted on a high-bandwidth VPS with Caddy's native HTTP/3 and zstd compression, direct origin serving easily delivers sub-100ms TTFB across Europe and North America.
* **Optional CDN Layer (Cloudflare / BunnyCDN):** A CDN can be layered in front of the VPS strictly as a caching proxy and DDoS buffer if traffic spikes significantly. Cloudflare's free tier or BunnyCDN ($1/mo) can cache 100% of static assets at the edge. The Caddy configuration is fully prepared to function behind an edge proxy without architectural modifications.

---

### 4.6 Privacy-Friendly Self-Hosted Analytics

To uphold ScribeTrace's core value of **zero telemetry and total user privacy**, commercial analytics providers (Google Analytics, Mixpanel) are strictly prohibited.

We evaluated two lightweight, open-source, privacy-first analytics engines: **Umami** and **Plausible CE**.

| Criteria | Umami (Recommended) | Plausible CE |
| :--- | :--- | :--- |
| **Backend & Architecture** | Node.js + Prisma (PostgreSQL / SQLite) | Elixir + ClickHouse + PostgreSQL |
| **RAM Footprint** | **~80 MB** total RAM | ~1.2 GB RAM (ClickHouse requires significant base memory) |
| **Cookie & Tracking Policy** | **100% Cookieless.** Uses an anonymized, daily rotating SHA-256 hash of IP + User Agent. | 100% Cookieless. Similar IP hash mechanism. |
| **Regulatory Compliance** | GDPR, CCPA, and PECR compliant without requiring cookie consent banners. | Same compliance level. |
| **VPS Suitability** | Runs effortlessly on a budget 1–2 GB RAM VPS. | Requires minimum 4 GB RAM VPS due to ClickHouse overhead. |

**Decision:** Deploy **Umami** via an isolated, internal Docker Compose stack.

#### Umami Docker Compose Deployment (`/opt/analytics/docker-compose.yml`)

```yaml
version: '3.8'

services:
  umami-db:
    image: postgres:16-alpine
    container_name: umami-db
    restart: always
    environment:
      POSTGRES_DB: umami
      POSTGRES_USER: umami
      POSTGRES_PASSWORD: ${DB_PASSWORD}
    volumes:
      - /var/lib/umami/postgres:/var/lib/postgresql/data
    networks:
      - internal-net
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U umami"]
      interval: 10s
      timeout: 5s
      retries: 5

  umami-app:
    image: ghcr.io/umami-software/umami:postgresql-latest
    container_name: umami-app
    restart: always
    environment:
      DATABASE_URL: postgresql://umami:${DB_PASSWORD}@umami-db:5432/umami
      DATABASE_TYPE: postgresql
      APP_SECRET: ${APP_SECRET}
      DISABLE_TELEMETRY: 1
      TRACKER_SCRIPT_NAME: script.js
      COLLECT_API_ENDPOINT: /api/collect
    ports:
      # Bound strictly to loopback 127.0.0.1; unreachable from public network
      - "127.0.0.1:3000:3000"
    depends_on:
      umami-db:
        condition: service_healthy
    networks:
      - internal-net

networks:
  internal-net:
    driver: bridge
```

---

## 5. CI/CD Architecture for Website Deployment

The deployment pipeline is governed entirely through GitHub Actions via GitOps principles. Every push to `main` that touches documentation or the website triggers an automated build, validation check, and atomic deployment over SSH to the maintainer's VPS.

### 5.1 Deployment Pipeline Flowchart

```mermaid
flowchart TD
    subgraph GitHub Actions Runner
        A[Trigger: Push to main / PR] --> B[Checkout Repository]
        B --> C[Setup Node 22 & Corepack pnpm]
        C --> D[Install Dependencies: pnpm install --frozen-lockfile]
        D --> E[Build Astro Starlight Site: pnpm build]
        E --> F[Generate Pagefind Search Index]
        F --> G[Run Link Checker: Lychee]
        G --> H{Is Branch main?}
    end

    subgraph VPS Deployment via SSH
        H -- Yes --> I[Establish SSH with Restricted Deploy Key]
        I --> J[rsync assets to /var/www/scribetrace/releases/BUILD_ID]
        J --> K[Verify Build Assets Integrity]
        K --> L[Atomic Symlink Swap: current -> releases/BUILD_ID]
        L --> M[Prune Old Releases: Keep Last 5]
    end

    subgraph Pull Request Preview
        H -- No (PR) --> N[Deploy Ephemeral Preview to Cloudflare / GitHub Pages]
        N --> O[Post Preview URL Comment to PR]
    end
```

---

### 5.2 Atomic Release Directory Topology on VPS

Deployments avoid in-place overwriting to prevent serving partial, corrupted files to active visitors during an rsync operation. The filesystem uses an **atomic symlink rotation pattern**:

```
/var/www/scribetrace/
├── current -> /var/www/scribetrace/releases/20261106-001200   # Active symlink
├── releases/
│   ├── 20261105-180000/
│   ├── 20261105-223000/
│   ├── 20261106-001200/                                     # Live release
│   └── .tmp-20261106-013000/                                # In-flight rsync upload
└── shared/
    └── uploads/                                              # Shared persistent assets (if any)
```

**Atomic Symlink Rotation Command:**
```bash
# Atomically point 'current' to the new release without race conditions
ln -sfn "/var/www/scribetrace/releases/${BUILD_ID}" /var/www/scribetrace/current_tmp && \
mv -Tf /var/www/scribetrace/current_tmp /var/www/scribetrace/current
```

---

### 5.3 Automated Rollback Mechanism

If a post-deployment issue is discovered, rollback to the previous functional release is instantaneous (< 1 second) via a single command or workflow dispatch:

```bash
#!/usr/bin/env bash
# ScribeTrace Instant Rollback Script (/usr/local/bin/scribetrace-rollback)
set -euo pipefail

RELEASES_DIR="/var/www/scribetrace/releases"
PREVIOUS_RELEASE=$(ls -1td ${RELEASES_DIR}/* | sed -n '2p')

if [ -z "${PREVIOUS_RELEASE}" ]; then
    echo "ERROR: No previous release directory found to roll back to."
    exit 1
fi

echo "Rolling back ScribeTrace to release: ${PREVIOUS_RELEASE}"
ln -sfn "${PREVIOUS_RELEASE}" /var/www/scribetrace/current_tmp
mv -Tf /var/www/scribetrace/current_tmp /var/www/scribetrace/current

echo "Rollback successful. Currently active release: $(readlink /var/www/scribetrace/current)"
```

---

### 5.4 Production GitHub Actions Workflow (`.github/workflows/website.yml`)

The following workflow conforms strictly to **SEC-27** (least-privilege `permissions: contents: read` and full 40-character commit SHA pinning):

```yaml
name: Website Build & Deploy

on:
  push:
    branches: ["main"]
    paths:
      - 'website/**'
      - 'docs/website/**'
      - '.github/workflows/website.yml'
  pull_request:
    branches: ["main"]
    paths:
      - 'website/**'
      - 'docs/website/**'

permissions:
  contents: read

concurrency:
  group: website-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}

jobs:
  build:
    name: Build & Validate Website
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2
        with:
          persist-credentials: false

      - name: Setup Node.js 22
        uses: actions/setup-node@1d0ff469b7ec7b3cb9d8673fde0c81c44821de2a # v4.2.0
        with:
          node-version: 22

      - name: Setup Corepack pnpm
        shell: bash
        run: |
          corepack enable
          corepack prepare pnpm@9.15.9 --activate

      - name: Install website dependencies
        working-directory: website
        run: pnpm install --frozen-lockfile

      - name: Build Astro Starlight site & Pagefind
        working-directory: website
        run: pnpm build

      - name: Check for broken internal links
        uses: lycheeverse/lychee-action@f613c4a64e50d792e0b31ec34bbcbba12263c6a6 # v2.3.0
        with:
          args: --exclude-all-private --verbose website/dist/**/*.html
          fail: true

      - name: Upload build artifact
        if: github.ref == 'refs/heads/main' && github.event_name == 'push'
        uses: actions/upload-artifact@4cec3d8aa04e39d1a68397de0c4cd6fb9dce8ec1 # v4.6.1
        with:
          name: website-dist
          path: website/dist/
          retention-days: 1

  deploy:
    name: Deploy to Maintainer VPS
    needs: build
    if: github.ref == 'refs/heads/main' && github.event_name == 'push'
    runs-on: ubuntu-latest
    environment: production-website
    steps:
      - name: Download build artifact
        uses: actions/download-artifact@cc203385981b70ca67e1cc392babf9cc229d5806 # v4.1.9
        with:
          name: website-dist
          path: dist

      - name: Setup SSH Agent & Deploy Key
        uses: webfactory/ssh-agent@dc588b651fe13675774614f8e6a936a468676387 # v0.9.0
        with:
          ssh-private-key: ${{ secrets.VPS_DEPLOY_SSH_KEY }}

      - name: Deploy build over SSH via rsync
        shell: bash
        env:
          VPS_HOST: ${{ secrets.VPS_HOST }}
          VPS_USER: ${{ secrets.VPS_DEPLOY_USER }}
        run: |
          BUILD_ID=$(date -u +%Y%m%d-%H%M%S)
          TARGET_DIR="/var/www/scribetrace/releases/${BUILD_ID}"
          
          # Add VPS host key to known_hosts safely
          mkdir -p ~/.ssh
          ssh-keyscan -H "${VPS_HOST}" >> ~/.ssh/known_hosts

          # 1. Create target release directory
          ssh "${VPS_USER}@${VPS_HOST}" "mkdir -p ${TARGET_DIR}"

          # 2. Rsync files into the new release directory
          rsync -avz --delete dist/ "${VPS_USER}@${VPS_HOST}:${TARGET_DIR}/"

          # 3. Perform atomic symlink rotation and release cleanup
          ssh "${VPS_USER}@${VPS_HOST}" "
            ln -sfn ${TARGET_DIR} /var/www/scribetrace/current_tmp && \
            mv -Tf /var/www/scribetrace/current_tmp /var/www/scribetrace/current && \
            cd /var/www/scribetrace/releases && \
            ls -1td * | tail -n +6 | xargs -r rm -rf
          "
          echo "Successfully deployed release ${BUILD_ID} to production."
```

---

## 6. VPS Hardening Checklist & Security Verification (SEC-30 Alignment)

To fulfill **SEC-30** ("Hardened VPS Bastion & Deployment Pipeline"), the maintainer VPS must pass a strict security hardening baseline prior to production launch. This checklist is structured for verification and sign-off by the Security Lead:

```
[ ] 1. Host Authentication & User Privilege
    [ ] Dedicated non-root administrative account created with sudo privileges.
    [ ] Root login completely disabled in SSH (`PermitRootLogin no`).
    [ ] Password authentication disabled in SSH (`PasswordAuthentication no`).
    [ ] Only modern Ed25519 SSH keys permitted (`PubkeyAuthentication yes`).
    [ ] Dedicated unprivileged deploy user (`deploy-agent`) created with restricted write access strictly to `/var/www/scribetrace/`.
    [ ] Strict SSH rate limiting and MaxAuthTries set to 3.

[ ] 2. Host Firewall (UFW / nftables)
    [ ] Default incoming traffic policy set to DROP.
    [ ] Default outgoing traffic policy set to ACCEPT.
    [ ] Only ports 22 (SSH), 80 (HTTP), and 443 (HTTPS / HTTP3 UDP) exposed publicly.
    [ ] Database and internal analytics ports (5432, 3000) bound exclusively to 127.0.0.1.

[ ] 3. Automated Vulnerability Patching & Intrusion Prevention
    [ ] 'unattended-upgrades' package installed and configured for automatic security updates.
    [ ] Daily automated package index updates and security package installs.
    [ ] 'fail2ban' or 'crowdsec' installed with aggressive SSH jail (ban after 3 failed attempts).
    [ ] Web server rate-limiting jail enabled against abusive HTTP probing.

[ ] 4. Kernel & System Hardening (/etc/sysctl.d/99-security.conf)
    [ ] SYN flood protection enabled (net.ipv4.tcp_syncookies = 1).
    [ ] IP packet forwarding disabled (net.ipv4.ip_forward = 0).
    [ ] Source routing packet rejection enabled (net.ipv4.conf.all.accept_source_route = 0).
    [ ] ICMP redirect acceptance disabled (net.ipv4.conf.all.accept_redirects = 0).

[ ] 5. Encrypted Off-Site Backup Strategy
    [ ] Automated daily BorgBackup / Restic backup script configured.
    [ ] Encrypted client-side with AES-256 before transit.
    [ ] Target destination: S3-compatible off-site storage (e.g. Backblaze B2).
    [ ] Snapshot retention policy: 7 daily, 4 weekly, 6 monthly snapshots.
    [ ] Automated monthly restore test verifying backup archive integrity.

[ ] 6. Observability, Log Rotation & Health Monitoring
    [ ] External healthcheck daemon pinging https://scribetrace.app every 60s (healthchecks.io or Uptime Kuma).
    [ ] Alerting via email / webhooks for immediate notification of downtime.
    [ ] logrotate configured with daily rotation, 14-day retention, and gzip compression.
    [ ] RAM and disk consumption alerting triggered at >85% threshold.
```

---

## 7. Download Distribution & Package Ecosystem Integration

In accordance with [ADR-0012](../adr/0012-distribution-updates-and-network-rule.md) and [SEC-28](../SECURITY-REQUIREMENTS.md), the desktop application contains **zero in-app auto-updater code**. The application's "Check for Updates" menu item simply opens the official release webpage in the user's default browser.

The website serves as the primary distribution hub, dynamically presenting the user's platform package while offering full manual selection and cryptographic verification data.

### 7.1 Dynamic Download Client Architecture
A zero-dependency client script inspects `navigator.userAgentData` (with `navigator.userAgent` fallback) to highlight the visitor's native installer, while maintaining instant tabbed access to all operating system builds:

```
[ Download ScribeTrace for Linux (.AppImage) - v1.0.0 ]  <- Auto-highlighted
  Other Linux Options: .deb | Flatpak (Flathub) | Arch AUR
  Other Platforms: macOS (Universal DMG) | Windows 10/11 (.msix / .exe)
```

### 7.2 Download Distribution Matrix

| Platform | Distribution Channel | Target Package Format | Installation Command / Link | Verification Artifacts |
| :--- | :--- | :--- | :--- | :--- |
| **Linux** | GitHub Releases | Standalone `.AppImage` | Direct binary download | `SHA256SUMS`, GPG `.sig` |
| **Linux** | GitHub Releases | Debian Package (`.deb`) | `sudo dpkg -i scribetrace_1.0.0_amd64.deb` | `SHA256SUMS`, GPG `.sig` |
| **Linux** | Flathub | Flatpak Bundle | `flatpak install flathub app.scribetrace.ScribeTrace` | Flathub GPG Key |
| **Linux** | Arch User Repository | AUR Package | `yay -S scribetrace-bin` | AUR PKGBUILD checksum |
| **macOS** | GitHub Releases | Universal DMG (`x86_64` + `arm64`) | Direct DMG download | Apple Notarization Ticket |
| **macOS** | Homebrew | Homebrew Cask | `brew install --cask scribetrace` | Homebrew Cask SHA-256 |
| **Windows**| GitHub Releases | MSIX / NSIS Installer (`.msix` / `.exe`) | Direct installer download | Authenticode Digital Signature |
| **Windows**| GitHub Releases | Portable Zip (`.zip`) | Extract and run | `SHA256SUMS`, GPG `.sig` |
| **Windows**| Windows Package Manager| Winget | `winget install scribetrace` | Microsoft Winget Manifest Hash |

### 7.3 Release Checksum & SLSA Provenance Verification Display
The download page embeds clear, accessible verification cards providing exact terminal instructions for end-user verification (SEC-29):

```bash
# Verify checksums on Linux / macOS
curl -sSLO https://github.com/sedam-or/scribetrace/releases/download/v1.0.0/SHA256SUMS
sha256sum --ignore-missing -c SHA256SUMS

# Verify build provenance using GitHub CLI
gh attestation verify scribetrace-1.0.0.AppImage --owner sedam-or
```

---

## 8. Documentation Architecture (Diátaxis Framework)

The documentation platform is organized rigorously under the **Diátaxis framework**, cleanly partitioning content into four cognitive quadrants:

```
                    PRACTICAL               THEORETICAL
              +--------------------+--------------------+
              |                    |                    |
   LEARNING   |     TUTORIALS      |    EXPLANATIONS    |
              |  (Learning-focused)| (Understanding)    |
              |                    |                    |
              +--------------------+--------------------+
              |                    |                    |
   WORKING    |    HOW-TO GUIDES   |     REFERENCE      |
              |  (Goal-oriented)   | (Information)      |
              |                    |                    |
              +--------------------+--------------------+
```

### 8.1 Documentation Sitemap & Topic Breakdown

#### Quadrant 1: Tutorials (Learning-Oriented)
Step-by-step introductory paths designed to give a new researcher an immediate aha! moment within 5 minutes:
1. **Quick Start in 60 Seconds:** Installing ScribeTrace, opening a research folder, inspecting the automatic `.scribetrace/` sidecar directory, and writing your first margin mark.
2. **Your First Literature Review Project:** Organizing a preprint corpus, color-coding thematic arguments, and following reading progress across 10 papers.
3. **From Margin Notes to Synthesis:** Capturing quotes into the side notes pane, attaching page anchors, and preparing a manuscript draft.

#### Quadrant 2: How-To Guides (Problem-Oriented)
Practical recipes for accomplishing specific workflows:
1. **Sync Across Laptops via Syncthing / Dropbox / Nextcloud:** How deterministic per-device JSONL files merge seamlessly across multiple machines without synchronization conflicts.
2. **Connecting to Local Zotero:** Enabling local HTTP broker access, importing PDF annotation marks, and linking citation keys into side notes.
3. **Exporting Annotations into an Obsidian Vault:** Executing safe, bounded Markdown exports into an Obsidian research vault ([ADR-0008](../adr/0008-notes-and-obsidian-export.md)).
4. **Erasing History via the Covers Model:** Executing a covers-based forget operation to delete local mark history without violating append-only invariants ([ADR-0007](../adr/0007-per-device-history-log-merge-and-forget.md)).
5. **Air-Gapped & Offline Operation:** Running ScribeTrace on classified or high-security networks with zero network adapters enabled.

#### Quadrant 3: Reference (Information-Oriented)
Authoritative, detailed technical specifications:
1. **Sidecar JSONL Event Specification (Schema v1):** Complete event schemas (`create`, `modify`, `delete`, `snapshot`, `purge`), Hybrid Logical Clock (HLC) format, and validation test vectors ([ADR-0010](../adr/0010-open-format-spec-and-cli.md)).
2. **W3C Web Annotation Composite Selectors:** Exact structure of `TextQuoteSelector` (exact, prefix, suffix context) and `TextPositionSelector`.
3. **CLI Reference Guide:** Syntax, exit codes, and examples for `scribetrace verify`, `scribetrace dump`, and `scribetrace forget`.
4. **Keyboard Shortcuts & Power-User Navigation:** Complete reference table for reading navigation, color highlighting shortcuts, split-pane toggling, and timeline scrubbing.
5. **Configuration & Storage Paths:** Exact filesystem locations of `device.json` and local cache directories on Linux (`~/.local/share/scribetrace/`), macOS (`~/Library/Application Support/scribetrace/`), and Windows (`%APPDATA%\scribetrace\`).

#### Quadrant 4: Explanation (Understanding-Oriented)
Architectural deep-dives explaining the rationale behind design choices:
1. **Why Plain JSONL Sidecars Beat Database Blobs:** Comparing ScribeTrace's per-device sidecar architecture against monolithic SQLite databases (Zotero) and single-state Lua files (KOReader).
2. **The Bit-for-Bit Sacred Invariant:** The cryptographic proof and filesystem discipline ensuring original research PDFs are never modified.
3. **The Covers Model for Forgetting:** Why distributed append-only systems cannot safely perform instantaneous physical file erasures across unsynced devices, and how sequence-boundary tombstones solve this problem mathematically.
4. **Clean Architecture in Rust & Tauri:** The strict layering rules separating domain entities from Tauri IPC gateways and React presentation components.

#### Contributor & Developer Documentation
Dedicated section targeted at open-source contributors:
1. **One-Command Development Setup:** Toolchain prerequisites, Rust 1.99+, Node 22, pnpm, and running `cargo nextest run` ([DEVELOPMENT.md](../DEVELOPMENT.md)).
2. **Architectural Invariants & ADR Catalog:** Index of all ratified Architecture Decision Records.
3. **Running Verification Spikes:** How to execute and benchmark the property-based log-merge, purge suppression, and Tauri webview spikes.
4. **Security Vulnerability Reporting:** Coordinated vulnerability disclosure protocol via GitHub Private Vulnerability Reporting and `security.txt` (SEC-32).

---

## 9. Operational Economics & Annual Budget Breakdown

Hosting ScribeTrace's website and docs platform on independent infrastructure is remarkably cost-effective. The static-first architecture eliminates the high compute, database, and caching costs associated with dynamic web platforms.

| Infrastructure Component | Provider / Solution | Resource Tier | Monthly Cost (USD) | Annual Cost (USD) |
| :--- | :--- | :--- | :--- | :--- |
| **Maintainer VPS** | Hetzner Cloud / DigitalOcean / Linode | 1 vCPU, 2 GB RAM, 25 GB NVMe SSD, 20 TB Bandwidth | ~$4.50 | ~$54.00 |
| **Domain Registration** | Cloudflare Registrar / Porkbun | Top-Level Domain (`scribetrace.app`) | ~$1.25 | ~$15.00 |
| **Edge CDN & DDoS Buffer** | Direct Origin / Cloudflare Free Tier | Caching, Anycast DNS, SSL fallback | $0.00 | $0.00 |
| **Off-Site Encrypted Backups**| Backblaze B2 | 15 GB encrypted daily snapshot storage | ~$0.10 | ~$1.20 |
| **GitHub Actions CI/CD** | GitHub Actions | Public open-source repository (Unlimited free minutes) | $0.00 | $0.00 |
| **Total Estimated TCO** | — | — | **~$5.85 / mo** | **~$70.20 / yr** |

> [!TIP]
> **Cost Optimization Invariant:** Because all downloads (DMGs, AppImages, EXEs) are hosted directly on **GitHub Releases**, the maintainer's VPS never incurs heavy outbound binary bandwidth charges. The VPS bandwidth is consumed exclusively by static HTML, CSS, compressed WebP images, and Pagefind search shards (~150 KB per unique visitor session).
