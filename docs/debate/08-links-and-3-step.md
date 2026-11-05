# Round 8: Links, the 3-step journey, lightness

Builds on [03-decision-memo](03-decision-memo.md) and [07-tech-decision-memo](07-tech-decision-memo.md). Plain language; all numbers are targets, to be measured in the prototype.

## Part A: "I have a link, not a file"

**What the facts say** (checked 2026-11-05): the [Zotero Connector](https://www.zotero.org/support/adding_items_to_zotero) saves metadata and, on many sites, the PDF "if you have access to it", or an open-access PDF. A page without a PDF becomes a "Web Page" item, with a snapshot only if that setting is on. The docs say nothing about arXiv pages (to test in Stage 0). Zotero's [local interface](https://www.zotero.org/support/dev/web_api/v3/local_api) lists items and gives a `file://` address for an attachment; it does not mention annotations or snapshot content (also a Stage 0 check). Zotero 7 can annotate web snapshots ([forum announcement](https://forums.zotero.org/discussion/106716/available-for-beta-testing-updated-reader-with-epub-snapshot-support-and-new-annotation-types)), but snapshots are web pages, not PDF or Markdown, so they stay outside our MVP.

| # | Option | User value | Effort | Maintenance | Risk | Stage | Verdict |
|---|---|---|---|---|---|---|---|
| 1 | **Do nothing special**: Zotero's Connector saves the link; Palimpsest imports from Zotero | High for Zotero users: one click in the browser they already have, then it shows up in our list | **S** (nothing new; we only explain it) | None of ours | Depends on Zotero saving a PDF; web-only pages give no PDF | 1 | **Recommended** |
| 2a | Paste a **direct PDF link** (`palimpsest add <url>`) | Medium: covers people without Zotero | S | Low | Adds a network use on click; downloaded file is untrusted | 2 | Later, only if validation shows many non-Zotero users |
| 2b | Paste a **web page link**, save as Markdown/PDF | Medium, but this makes us a web clipper | **M–L** (article extraction, scripts, logins) | High: sites change constantly | Large scope creep; poor results on scripted pages | 3 | No, not before the gate |
| 3 | **Own browser extension** | Low extra value: it duplicates the Zotero Connector | **L** (one per browser, store reviews) | High, permanent | New attack surface between browser and app | 3 | No |
| 4 | **Deep link** `palimpsest://open?paper=…&page=…` | Medium: a quote in Obsidian jumps back to the exact page | S–M (register per system) | Low | Any web page can fire it, so it may only open a paper already in the project, after a confirmation | 2 (needs our reader; in Stage 1 links go back to Zotero) | Later |

**Recommendation:** option 1 for the MVP. It solves "I have a link" with no new code. The first-run screen says in one line: *"Found a paper online? Save it with Zotero's browser button, then import it here."*

**What we promise about the live web**

| Situation | What happens | Promise |
|---|---|---|
| Paywall / login-only page | We never log in or fetch for you. Zotero uses your own access in your own browser. | We work with what you saved. No account bypass, ever. |
| Script-heavy page | We do not render web pages in the MVP. | Only PDFs Zotero saved, plus Markdown. |
| Page changes | A changed page saved again is a new file, shown as a new paper, never a silent replacement. | We version **your marks and notes**, not the web page. |
| Page disappears (link rot) | The quoted text is stored with each mark. | Your marks and quotes stay readable even if the source is gone. |

## Part B: the 3-3-3-3 journey

Rule for every future feature: it must name the user goal and fit into one of these steps, or it waits.

| Stage | Step | Target time | Pass condition |
|---|---|---|---|
| **Install** | 1. Download the installer (or use a package manager) | 2 min | One file, no account, no form |
| | 2. Run it | 1 min | No admin-only extras beyond the system's own prompt |
| | 3. Open the app | 30 s | Window usable in under 3 s, nothing else asked |
| **Setup** | 1. Switch on Zotero's local access (the screen shows where) | 1 min | Plain instruction, with a picture |
| | 2. Choose the project folder | 30 s | Folder shown; our hidden folder created on first save |
| | 3. Press "Connect to Zotero" | 30 s | "Connected, N papers", or an error naming the exact setting to change |
| **Onboarding** | 1. Pick one paper from the list | 30 s | Paper list loads in under 2 s |
| | 2. Import its marks (first milestone) | 1 min | **First mark imported within 5 min of install** |
| | 3. Copy one highlight as a quote with citation | 1 min | Quote pasted into notes within 10 min of install; 4 of 5 unassisted testers succeed |
| **Everyday** | 1. Read and mark in Zotero, as before | n/a | No change to the habit |
| | 2. Click Refresh (new milestone) | 5 s | Refresh of a paper with 200 marks under 5 s |
| | 3. See "what changed", then quote or export | 1 min | Timeline opens in under 1 s; Obsidian export on one click |

## Part C: lightness promises

| Promise | Target (to be measured in the prototype) |
|---|---|
| Installer size | Stage 1: 20 MB or less; Stage 2 (with the reader): 35 MB or less |
| Idle memory, window open, no paper | 120 MB or less |
| Memory with a 200-page PDF open (Stage 2) | 250 MB or less |
| Cold start to usable window | 2 s or less on a 4-core, 8 GB computer (same reference as QA's gates) |
| Background process | None: closing the window ends everything; no tray icon, no auto-start, no folder watching |
| Account / sign-in | None, ever |
| Network | None unless the user clicks; only the two clicks named in 07 (Zotero on this computer, update check) |
| Disk for history | 1 MB or less per paper per year of normal use |
| Extra installs | None for the user, except where a system component is missing (Linux web view: unverified) |

Any number we miss is reported honestly before release; we publish only what the prototype proved.

## Part D: Decisions for the user

1. **Answer to "I have a link":** use Zotero's Connector, build nothing in the MVP, say so in the first-run screen. Recommend **yes**. Alternative: our own extension (large, permanent, duplicates Zotero).
2. **Promise on live pages:** we keep your marks and quoted text, not a record of how a web page changed. Recommend **yes**. This makes the limit explicit.
3. **Publish the lightness numbers** as public promises only after the prototype proves them. Recommend **yes**.
4. **Adopt 3-3-3-3 and "no feature without a stated goal"** as a standing review rule for every proposal. Recommend **yes**.
