# ScribeTrace

**ScribeTrace remembers how your reading of a paper changed.** It is a small desktop companion to Zotero and Obsidian that keeps the history of your marks and notes in open files you own.

> **Status: pre-alpha, design phase. Nothing to install yet.** This repository holds the plan, not a working app. Everything below describes what we intend to build; none of it exists yet.

## Why

You highlight a paper, come back months later, and the page shows only today's marks. What did you think the first time? Which highlight did you delete, and why? Zotero keeps its annotations in its own database, and its documentation says nothing about a history of them ([Zotero](https://www.zotero.org/support/kb/annotations_in_database), checked 2026-11-05).

- **History of your reading.** Every import from Zotero and every note change is kept. You name milestones ("first skim", "rebuttal") and compare any two: marks added, removed, edited, and what changed in your notes.
- **A companion, not a replacement.** You keep reading and marking in Zotero and writing in Obsidian. ScribeTrace never writes into Zotero and never modifies your papers.
- **Lightweight.** Targets, to be measured: an installer of 20 MB or less, 120 MB of memory when idle, a start in 2 seconds or less, no background process, no account, no telemetry. No network traffic unless you click, and then only to Zotero on your own computer.
- **Files you own.** Plain, documented files in a `.scribetrace/` folder next to your papers, readable without the app, plus a small command-line tool, `scribetrace-cli`. The format is **unstable v0** until real outside use.
- **Yours to erase.** A "forget this paper's history" button for confidential material, with a plain warning that copies in sync services or Git may remain.

## Quick start (planned)

1. **Install.** Download the installer for your system, run it, open the app. No account.
2. **Set up.** Switch on Zotero's local access (Settings → Advanced), choose your project folder, press Connect.
3. **Use.** Pick a paper, import its marks, copy a highlight as a quote with its citation. Later, press Refresh to record a new milestone and see what changed.

Our target is a first mark imported within 5 minutes of install ([journey targets](docs/debate/08-links-and-3-step.md)).

## Honest limits

- **Needs Zotero 7 with local access switched on**, and Zotero running when you import. Whether Zotero's local interface gives us marks, dates and page locations reliably is **not yet verified**; that check comes first.
- **No history of a changing web page.** We keep your marks and the quoted text, not the page. To save a link, use Zotero's browser connector and import from Zotero.
- PDF and Markdown only. EPUB and HTML come later, if at all.
- In the first stage you read and mark in Zotero; ScribeTrace's own reader is built only if people actually use the history.
- The name is not trademark-cleared yet ([naming notes](docs/NAMING.md)).

## Our pledge

- **No open core.** Every feature of the app stays Apache-2.0.
- **The app never needs an account or a server.** If an optional paid service ever exists, it will not be required and will not reduce the free app ([decision o](docs/DECISION-LOG.md)).

## Documents

| | |
|---|---|
| [Requirements](docs/REQUIREMENTS.md) | What we build, in stages |
| [Business and strategy](docs/BUSINESS.md) | Positioning, competitors, licence, risks |
| [Decision log](docs/DECISION-LOG.md) | Decisions taken and what still needs the owner |
| [Architecture](docs/ARCHITECTURE.md) | How it fits together, lightness budget |
| [Security](docs/SECURITY-REQUIREMENTS.md) · [Threat model](docs/THREAT-MODEL.md) | Trust and privacy rules |
| [Stress test](docs/STRESS-TEST.md) | What could make this fail |
| [Prototype plan](docs/PROTOTYPE-PLAN.md) | Spikes that must pass before real code |
| [Design debate](docs/debate/) | How we got here |

## Licence

Planned: Apache-2.0 (not yet confirmed by the owner, see the [decision log](docs/DECISION-LOG.md)).
