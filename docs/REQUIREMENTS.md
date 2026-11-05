# ScribeTrace: Requirements (v1.1)

**Date:** 2026-11-05 · **Author:** pm · **Status:** draft for the user's review · **Replaces:** v1.0 (F1–F7, wider scope; superseded, see [DECISION-LOG](DECISION-LOG.md))
**Name:** ScribeTrace (formerly Palimpsest, codename clickme). Trademark databases are **unchecked**; a manual US/EU/WIPO search is needed before any public launch ([NAMING](NAMING.md)). Some linked documents still say "Palimpsest"/`.palimpsest/` until the rename lands; this file uses `.scribetrace/` and CLI `scribetrace-cli` (the app binary is `scribetrace`).

## 1. Scope in one paragraph
ScribeTrace remembers how your reading of a paper changed. It is a **companion** to Zotero and Obsidian (no reference management), keeps history in open files in a local `.scribetrace/` folder, and is lightweight. Desktop; PDF and Markdown only; open source (Apache-2.0).
**Stage 0** validates (about 8 interviews, mock-up, hand-run script, one-day Zotero check). **Stage 1** is a recorder and history viewer: reading and marking stay in Zotero. **Product gate:** at least 4 of 8 testers open the history unprompted at least twice (target, [STRESS-TEST](STRESS-TEST.md) ST-01). **Stage 2**, only if the gate passes: our own PDF and Markdown reader.

Source tags: **U** user decision · **D1/D3/D7/D8** [debate 01](debate/01-proposal.md) / [03](debate/03-decision-memo.md) / [07](debate/07-tech-decision-memo.md) / [08](debate/08-links-and-3-step.md) (with row or part) · **ST** [STRESS-TEST](STRESS-TEST.md). **Cap** = capability row in [ARCHITECTURE §6](ARCHITECTURE.md). **SEC** = [SECURITY-REQUIREMENTS](SECURITY-REQUIREMENTS.md). MoSCoW: **M** must, **S** should, **C** could (all for the stage named).

## 2. Stage 1 requirements (recorder and history viewer)

| ID | Requirement (EARS) | Pri | User goal | Cap | SEC | Source |
|---|---|---|---|---|---|---|
| R-01 | WHEN the user clicks Import for a paper, the system SHALL read that paper's marks, their dates and its page locations from the local Zotero and store them as **read-only copies** that show their origin. | M | See my existing marks without retyping | Import | 8, 33 | U, D3-3 |
| R-02 | The system SHALL contact Zotero only on a user click, SHALL say on screen when it does, and SHALL start no background process, folder watcher or polling. | M | Trust that nothing is recorded behind my back | Import, Lightness | 1 | D3-9, D7 |
| R-03 | The system SHALL NOT write to Zotero or read Zotero's database file; IF the local interface does not expose marks and dates, THEN work SHALL pause and return to the user with options. | M | Never damage my Zotero library | Import | 33 | D7-5a, D7 §3.4, ST-02 |
| R-04 | IF Zotero is not running or its local access is off, THEN the system SHALL show one plain message that names the exact Zotero setting to change, and SHALL NOT retry in the background. | M | Fix the connection myself in a minute | Import | 33 | D3-15, ST-03 |
| R-05 | WHEN first started, the system SHALL show a three-step setup (switch on Zotero access, choose the project folder, press Connect) with a live "Connected, N papers" check. | M | First mark imported within 5 minutes of install | Import | 1 | D8-B, ST-03 |
| R-06 | The system SHALL append every import, refresh, note change and forget as an event to **this device's own** history file for that paper, and SHALL NOT rewrite or delete existing lines (except through R-16). | M | Never lose how my reading changed | History | 10 | U, D3 |
| R-07 | WHEN several machines hold the same project folder, the system SHALL show the union of all device files with the same result in any order, and merging twice SHALL give the same result as once. | M | Work on two computers without conflicts | History | 10 | D7-4, D7-8 |
| R-08 | The system SHALL create a random device id in per-user application storage (never in the project folder), and IF a history file was changed in a way this device did not write, THEN it SHALL use a new id. | M | Cloned or restored machines do not corrupt history | Clones | 34 | D7-6 |
| R-09 | IF a history line is invalid or too large, THEN the system SHALL skip it, count it in the window and in `scribetrace-cli check`, and leave all files unchanged. | M | One bad line never hides my history | Corrupt line | 12, 13 | D7, QA-D2 |
| R-10 | After a crash or forced stop, the system SHALL recover with no malformed history line and at most the last unfinished action lost. | M | Trust it with my thinking | Clones | 13 | QA gate 2 |
| R-11 | WHEN the user imports or refreshes, the system SHALL create one **milestone**, except when nothing differs from the last one; the user MAY name any milestone ("first skim") and the system SHALL create one when a work session in which notes changed ends. | M | Compare "then" and "now" | Milestones | 10 | D3-4, D3-12 |
| R-12 | The system SHALL list milestones on a timeline and SHALL list, between any two, the marks added, removed or edited and the changed note text. | M | Answer "what changed since March?" | Milestones | 8 | D3-5, U |
| R-13 | WHEN the user restores a milestone, the system SHALL show or recreate our copy as of that point by adding a new event, and SHALL leave Zotero and all earlier events untouched. | M | Undo without fear | Milestones | 10 | D3-1 |
| R-14 | The system SHALL keep notes for a paper as a plain Markdown file plus every saved revision in the history; IF the file was changed outside the app, THEN it SHALL keep that text as a new revision when the paper is opened (last writer wins, both kept). | M | Notes I can read without the app | Notes | 7, 9 | D7-5, D3 |
| R-15 | WHEN the user copies a mark as a quote, the system SHALL produce text with the paper's citation key (or, when none is available, author, year and title) and a link back to the exact page in Zotero, and SHALL ask for confirmation before opening any web or Zotero link. | M | Paste a ready quote into my writing | Notes | 8, 23, 24 | D3 feat. 3 |
| R-16 | WHEN the user confirms "Forget this paper's history", the system SHALL delete that paper's history files on this machine and add a purge event that hides earlier events from every device; it SHALL warn plainly that copies in sync services or Git may remain, and SHALL show other machines a banner offering to delete the hidden files. | M | Erase a confidential review's trail | Forget | 11 | D3-10, D7-1, U |
| R-17 | WHEN the user clicks Export to Obsidian, the system SHALL write only into a marked subfolder of a folder the user chose, and SHALL overwrite only files it generated that the user has not edited. | M | Get my notes into my vault safely | Obsidian | 18, 36 | D3-6, D7 |
| R-18 | The system SHALL document the file format with schema and sample files as **unstable v0**, and SHALL ship a small command-line tool, `scribetrace-cli`, that checks and reads the files with the same code as the app. | M | Own my data; scripts can read it | Open format | 8, 12, 14, 21 | U, D3-7 |
| R-19 | The system SHALL NOT modify source papers, Zotero files or any file outside the project folder, the chosen export folder and its own app storage. | M | My originals stay untouched | Import | 17, 20 | U, QA gate 1 |
| R-20 | The system SHALL show all text from files, Zotero and Markdown as inert text, never as executable markup. | M | A shared folder cannot attack me | Notes, Corrupt line | 2, 7, 8, 9 | D7 |
| R-21 | The default author label SHALL be "Me"; nothing from the computer or its user name SHALL be written into the files. | M | Share a folder without leaking my identity | History | 35 | D7-7 |
| R-22 | WHEN the user clicks "Check for updates", the system SHALL show the installed version and open the fixed release page in the default browser, and SHALL make no request itself. | S | Know when a fix exists | Lightness | 28 | D7-3, U |
| R-23 | The system SHALL meet the lightness and journey targets in §4, each published only after the prototype measures it. | M | A light, fast tool | Lightness | 1, 2 | D8-C, U |

## 3. Stage 2 and later (only after the product gate)

| ID | Requirement (EARS) | Pri | Cap | SEC | Source |
|---|---|---|---|---|---|
| R-30 | WHERE the product gate is passed, the system SHALL open PDFs read-only in a reader of its own, with marks stored as ours. | M (S2) | Reader | 5, 19 | D3-2 |
| R-31 | WHERE the reader exists, the system SHALL place a mark by page and box, repair it by quoted text, and IF it cannot be placed THEN mark it *orphaned*, never delete it. | M (S2) | Reader | 5 | D7-3 |
| R-32 | WHERE the reader exists, the system SHALL open Markdown papers and anchor marks to the source text. | M (S2) | Reader | 7 | U, D3-8 |
| R-33 | WHERE Stage 2 is adopted and users ask for it, the system SHALL show two milestones side by side, with marks added, removed and edited highlighted on the page. | C | Milestones | 2 | D3-5 |
| R-34 | WHERE a later release adds them and the trigger in [ARCHITECTURE §7](ARCHITECTURE.md) is met, the system SHALL provide a reading-session log, a W3C JSON-LD export, `scribetrace://` links to a paper and page, and adding a paper from a direct PDF link, each asking for the user's confirmation before it acts. | C (later) | Open format | 22 | D8-A, ARCH §7 |

## 4. Success metrics: journey and lightness (targets, to be measured in the prototype)

| Measure | Target | Source |
|---|---|---|
| Install, setup, onboarding, everyday use: at most 3 steps each | 12 steps in total | D8-B |
| First mark imported after install | within 5 min; 4 of 5 unassisted testers | D8-B, ST-03 |
| Quote pasted into notes | within 10 min of install | D8-B |
| Refresh of a paper with 200 marks / timeline opens / paper list loads | ≤ 5 s / ≤ 1 s / ≤ 2 s | [ARCH budget](ARCHITECTURE.md) |
| Installer: Stage 1 / Stage 2 | ≤ 20 MB / ≤ 35 MB | ARCH budget |
| Idle memory (window open) / with a 200-page PDF (Stage 2) | ≤ 120 MB / ≤ 250 MB | ARCH budget |
| Cold start to usable window | ≤ 2 s on 4 cores, 8 GB | ARCH budget |
| Background processes, accounts, telemetry | none, none, none | U, D8-C |
| Network | only Zotero on this computer, only on click | SEC-1 |
| Launch blockers (hard stops) | sources byte-identical; no history lost on a crash | QA gates 1–2 |

## 5. Non-goals (explicit)
Reference management, cloud service, accounts, AI summaries, editing source papers, live web-page capture or history of a changing web page (we version **your marks and quotes**, not the page), our own browser extension, writing back into Zotero, live Obsidian mirror or edits flowing back, collaborative bundle merging, group libraries, an in-app updater, EPUB and HTML (Later), tamper-evidence chain (until asked). Links: save them with Zotero's Connector ([ADR 0013](adr/0013-links-via-zotero-connector.md), D8-A).

## 6. Open points
Zotero's local interface may not expose marks or dates (Stage 0 check, R-03); citation-key availability is unverified; trademark clearance unchecked. All are tracked in [DECISION-LOG](DECISION-LOG.md) and [STRESS-TEST](STRESS-TEST.md).
