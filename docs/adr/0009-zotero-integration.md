# ADR 0009: Zotero integration through its local interface

**Status:** Accepted, **conditional on SPIKE-3 (day-1 live check, a gate before Stage 1 coding)**  
**Date:** 2026-11-05 (slimmed in Round 8)

## Context
Zotero is the paper source and, in Stage 1, the place users read. Zotero's local interface (`127.0.0.1:23119/api/`, read-only, off by default in Settings → Advanced; [docs](https://www.zotero.org/support/dev/web_api/v3/local_api), read 2026-11-05) is documented for items and files; **not documented**: annotation fields and dates, paging past 100, trashed items. That gap is the spike.

## Decision
- Options considered: local interface; reading Zotero's database file; user-exported files; a Zotero plug-in. **Pick the local interface only.** *Why:* the database file is locked and unversioned; exports and plug-ins add manual steps or a second code base.
- One core component, the **broker**, makes every Zotero request: address `127.0.0.1` only, no redirects, GET only, paths limited to personal-library items and their children, 5 s timeout, 15 MB per response (page by page), API-version header checked, answer parsed strictly, only on a user click. The window cannot open sockets.
- Marks are copied read-only with their origin (item key, annotation key, type, text, comment, colour, page label, position, dates); image and ink marks are listed as "not imported". Never written back.
- Refresh appends a milestone; marks no longer present get `mark_gone`. Link back: `zotero://open-pdf/...` (best effort).
- **Tests:** answers recorded once from a dummy library, replayed by a stand-in server in CI; live check on day 1 and before each release.
- **If SPIKE-3 fails** (marks or dates missing): stop and ask the user (exports, plug-in or database file); nothing is built silently.

## Consequences
+ No change to users' libraries. − Zotero must be running with the setting on; the interface may change; group libraries are out of the MVP.

Security: the only local-network use; blocks SSRF and probing of other local services; Zotero text is untrusted (SEC-8, 21, 33).
