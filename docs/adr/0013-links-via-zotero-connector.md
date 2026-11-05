# ADR 0013: "I have a link, not a file": no capture component

**Status:** Accepted (follows pm's recommendation, option 1 in [debate/08](../debate/08-links-and-3-step.md))  
**Date:** 2026-11-05

## Context
Users find papers as links. Zotero's browser Connector already saves the item and, where the user has access, the PDF; ScribeTrace imports from Zotero ([ADR 0009](0009-zotero-integration.md)). Anything we add (a URL downloader, a web clipper, a browser extension) is network code, untrusted downloaded files and permanent maintenance, for no value Zotero does not already give.

## Decision
- **No new component.** Capability traced to "Import / refresh Zotero marks" ([ARCHITECTURE §6](../ARCHITECTURE.md)); the only work is behaviour already implied by ADR 0009 plus one line of UI text on the first-run screen: *"Found a paper online? Save it with Zotero's browser button, then import it here."*
- Items without a PDF (web pages) appear in the list with "no marks to import"; nothing is fetched or opened by us.
- A page that changes and is saved again is a new Zotero item, so it is a new paper; we version marks and notes, never web pages. Each mark stores its quote, so marks stay readable if the source disappears.
- **No browser extension, no web clipper, no login or paywall handling, ever.**
- **Later (triggers):** direct-PDF-URL add via the CLI (`scribetrace add <url>`) only if validation shows many users without Zotero; `scribetrace://` deep links only after Stage 2 and only for papers already in the project, after a confirmation (ADR 0012 keeps the network rule intact until then).

## Consequences
+ Zero new code, tests or attack surface; the network rule stays "Zotero on this computer only". − Users without Zotero cannot start from a link; Zotero may save no PDF for web-only pages (to test in Stage 0, per pm).

Security: no new input path (SEC-1, 33 unchanged). If the direct-URL option is ever added, it needs its own review (downloads are untrusted files; SEC-19, 20).
