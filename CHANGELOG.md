# Changelog

All notable changes to ScribeTrace are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Initial project scaffolding and open-source community health files (Apache-2.0).
- Product architecture specifications, threat model, and security-by-design requirements (`docs/`).
- Architectural Decision Records (ADR 0006 through 0013) for staged MVP architecture.
- Continuous integration skeletons (`ci.yml`, `spike4.yml`) with SHA-pinned actions.
- Issue and pull request templates for community contributions.
- Business and brand documents: go-to-market plan, validation protocol, brand voice (`docs/GO-TO-MARKET.md`, `docs/VALIDATION-PROTOCOL.md`, `docs/BRAND.md`).
- Single source of user-facing text (`src/strings/en.json`) and a Linux-first draft user guide (`docs/USER-GUIDE.md`).

### Changed
- Decision log now records the pledge (no open core; no account or server), the second-agent review policy and the full-dev roles (decisions o, u, v).

## Release notes draft: 0.1.0 (not released)
Draft for the first release; do not publish until a release exists and the owner has checked the release candidate (decision n).
- **What it is:** a recorder and history viewer for Zotero marks. Import marks, see milestones, compare two, copy a quote with its citation, keep notes, forget a paper's history, export one way to Obsidian.
- **Who it is for:** PhD students and postdocs who use Zotero 7 and write in Markdown or Obsidian.
- **Known limits:** needs Zotero 7 with local access on; no history of changing web pages; PDF and Markdown only; file format is unstable v0; Linux first.
- **Claims to hold back until measured:** installer size, memory, start time, first-mark-in-5-minutes.
