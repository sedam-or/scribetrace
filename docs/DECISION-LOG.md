# ScribeTrace: Decision log

**Date:** 2026-11-05 · **Author:** pm · Decisions taken under the user's delegation ("run end to end overnight, decide with reasons"). Each can be reversed unless noted. Related: [REQUIREMENTS](REQUIREMENTS.md), [BUSINESS](BUSINESS.md), [ARCHITECTURE](ARCHITECTURE.md), [STRESS-TEST](STRESS-TEST.md).

## What needs you
1. **Recruit about 10 researchers** (labs, PhD friends) for the product gate. Only you have the network. I supply the script and mock-up brief.
2. **Trademark search**, manual, USPTO + EUIPO + WIPO, for ScribeTrace before any public launch ([NAMING](NAMING.md)).
3. **Live Zotero check**: install Zotero 7, switch on local access, run the one-day check (SPIKE-3). Not installed on this machine, so marks, dates and page locations are unverified.
4. **Make the repository public**, when you decide. `sedam-or/scribetrace` stays private until then.
5. **Rust maintenance risk**: your other repositories are Laravel, Vue, TypeScript and Python. AI agents write the first code, but someone must maintain it. Say whether you accept Rust, or want Electron and TypeScript instead.

## Decisions

| | Decision | Reason | Alternative | Reversible? | Risk I see |
|---|---|---|---|---|---|
| a | **Name ScribeTrace**, folder `.scribetrace/`, CLI `scribetrace` (your choice) | Namespaces free on all registries checked; Palimpsest's are taken ([NAMING](NAMING.md)) | GlossTrace, ChronoMargin | Yes now; costly after a public release | Trademark databases **unchecked**: a manual US/EU/WIPO search is still needed before launch |
| b | **Staged build, recorder first**, reader after a product gate (D3-2, ST-18) | The reader is the costliest part and is not needed to show history; tests "do people look back?" in weeks | Full reader at once (months, same unproven headline) | Yes | The gate may say no; we must accept it |
| c | **Tauri v2 + Rust core, React UI: provisional until SPIKE-4 passes** ([ADR 0006](adr/0006-staged-mvp-and-delivery-shape.md)) | Smallest installer and memory, which serves "lightweight" | Electron (about 100 MB and more, ADR 0006) with a TypeScript core | Yes, cheap before Stage 1 code, costly after | **Rust maintenance (ST-13)**: your other repos are Laravel/Vue/TS/Python; AI agents write the first code; a stalled project is the main failure |
| d | **Imported Zotero marks are read-only copies; no write-back** (D3-3, D7-5a) | Cannot damage your Zotero library; much simpler | Two-way sync | Yes, but writing back is a large, risky feature | Users may ask "why can't I edit here?" before Stage 2 |
| e | **Forget = covers-only**: delete local files, add a purge event, plain warning in the dialog; offline events survive until forget is run again (D7-1, SEC-11) | Simple, testable, honest; no promise we cannot keep | Automatic erase everywhere (cannot be guaranteed), local-only without hiding | Yes | Copies in sync services or Git may remain; an offline machine keeps its events until forget is repeated |
| f | **"Check for updates" opens the release page**; no in-app request (D7-3, SEC-28) | One-sentence privacy story, no updater to attack | Notify-only fetch; signed in-place updater | Yes | Users stay on old versions |
| g | **Author label "Me"** (D7-7, SEC-35) | Nothing from the computer goes into shared files | System user name | Yes, but old files keep their label | Mixed labels if several people share a folder |
| h | **Links via the Zotero Connector; nothing built.** Live-page promise: marks and quotes only (D8-A, [ADR 0013](adr/0013-links-via-zotero-connector.md)) | Solves "I have a link" with no new code, extension or attack surface | Own extension; page-to-Markdown clipper; deep link | Yes; each is a later option with a stage | Zotero may not save a PDF for web-only pages (arXiv behaviour unverified) |
| i | **3-3-3-3 journey and lightness numbers are a standing review rule**, published only after the prototype proves them (D8-B/C) | Makes "lightweight" checkable; blocks feature creep | Qualitative promise | Yes | Numbers are targets; some may be missed |
| j | **W3C JSON-LD export moved to Later** (ARCH §7) | Not needed to prove the product; adds format surface to test | Ship in Stage 1 | Yes (trigger: an outside tool maintainer asks) | Weakens the "open standard" story at launch |
| k | **Licence Apache-2.0** | Patent grant for an open format others may embed; Rust/Tauri norm († not re-checked) | MIT: shorter, but no patent grant | Hard once outside contributions arrive | **Differs from your MIT choice in Kinevo**; confirm you want a different licence here |
| l | **GitHub repo `sedam-or/scribetrace` created private**, stays private until you decide | Nothing is public before the name, licence and trademark are settled | Public now | Yes | None; delays outside feedback |

## Also decided in this round
- Requirements rewritten from scratch (R-01 to R-34) in the staged scope; old F1–F7 numbering is retired ([REQUIREMENTS](REQUIREMENTS.md)).
- Product gate: at least 4 of 8 testers open the history unprompted at least twice (target from ST-01; to be confirmed after Stage 0 interviews).
