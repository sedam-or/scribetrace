# ADR 0011: Own reader (Stage 2): PDF.js, Markdown, simple anchoring

**Status:** Proposed, built only after the product gate  
**Date:** 2026-11-05 (slimmed in Round 8) · **Supersedes:** [archived ADR 0002](archive/0002-multi-format-rendering-engines.md), [0003](archive/0003-reflow-resilient-anchoring.md) (EPUB, HTML and the four-tier anchor cascade are Later)

## Context
Stage 2 lets users mark inside ScribeTrace (PDF and Markdown only). The hard problem is keeping a mark attached when the file or the text extraction differs.

## Decision
- **PDF.js** (Apache-2.0) in a worker for render, text layer and selection. *Why:* the standard open PDF renderer; native engines would add a second code base per system.
- **PDF mark** = page + boxes + exact quote with a little text around it. On open: boxes first; if the text there differs, find the quote on that page; else a time-boxed fuzzy search on that page; else **orphaned** (kept, re-attached by hand). Imported Zotero marks keep Zotero's page and boxes as primary. Text offsets are never trusted across tools.
- **Markdown mark** = quote + context + line, anchored to the source text.
- Zotero attachment paths must be ordinary files inside the granted storage folder (no link pointing outside). Source files are opened read-only.
- Validate with spike S2-A before coding ([PROTOTYPE-PLAN](../PROTOTYPE-PLAN.md)).

## Consequences
+ One small rule set, testable on a 10-PDF corpus. − Scanned PDFs without a text layer get box-only anchoring.

Security: PDF parsing is the main untrusted-input surface and appears only here: worker isolation, read-only access, hard time limit on fuzzy search (SEC-5, 17, 19, 20).
