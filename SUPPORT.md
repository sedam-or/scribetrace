# Support

## Where documentation lives

All project architectural documentation and specifications live in the repository under `docs/`:

- `docs/ARCHITECTURE.md` — technical architecture, storage invariants, and IPC design.
- `docs/PROTOTYPE-PLAN.md` — staged MVP execution plan and technical spikes.
- `docs/STRESS-TEST.md` — pre-mortem risk scenarios and empirical validation gates.
- `docs/DECISION-LOG.md` — ratified architectural and product decision register.
- `docs/SECURITY-REQUIREMENTS.md` — security-by-design requirements (SEC-1 through SEC-32).
- `docs/adr/` — Architectural Decision Records (ADR 0006 through 0013).

`AGENTS.md` defines the operating contract and layering rules for AI coding assistants and automation.

## Troubleshooting

- ScribeTrace operates 100% offline and stores local state in `.scribetrace/`.
- Verify Zotero 7 Local API is enabled: `Zotero → Settings → Advanced → Allow other applications on this computer to communicate with Zotero`.
- Inspect local logs inside `.scribetrace/logs/` if an import fails.
- Never manually edit `.history.jsonl` files; they are append-only.

## Where to ask questions

Prefer public, searchable channels so answers benefit the entire community:

- **GitHub Discussions** — general questions, scholarly reading workflows, and feature design ideas.
- **GitHub Issues** — bug reports and actionable feature requests.

## Where to report bugs

Open a **GitHub Issue** using the bug report template. Please include:

- Operating System & version (Linux, macOS, Windows).
- ScribeTrace version (`scribetrace --version`).
- Zotero version (e.g., Zotero 7.0.x).
- Steps to reproduce.
- Expected vs actual behavior.
- Error logs **without proprietary research data or secrets**.

## What should NOT go into GitHub Issues

- **Security vulnerabilities** — please use the private disclosure flow in `SECURITY.md`. Never post sensitive tokens or exploit details in public issues.
- **Confidential manuscript data** — peer-review papers under embargo, private annotations, or confidential research drafts.
- **Support for external tools** — issues regarding Zotero or Obsidian core functionality should be reported to their respective projects.

## License

ScribeTrace is licensed under the Apache License, Version 2.0. See `LICENSE`.
