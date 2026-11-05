# Round 7: Technical decision memo

Answers to [06-security](06-security.md), [06-qa](06-qa.md), [06-devops](06-devops.md). Options and reasoning: [05-tech-options](05-tech-options.md). Product side: [03-decision-memo](03-decision-memo.md). All reviewers endorsed decisions 1, 3, 5, 7, 8 and the core of 4 (per-device files), so only challenged points are below.

## 1. Answers to objections

| # | From | Objection | Answer | Why / what changes |
|---|---|---|---|---|
| 1 | Sec, QA, DevOps | The erase marker does not erase, can be lost or injected; cross-device delete is untestable. | **Accept the objection, change the design (partly adopting all three)** | "Forget" becomes: (a) delete this paper's history files **on this machine**; (b) write one **purge event** (no content) into this device's own log. It is an ordinary event, not a magic file. Rule when reading: events at or before the purge time are ignored everywhere, which is a pure function and easy to test (QA). (c) Other machines are **asked**, never changed silently: "history was erased on another device; delete the hidden files here?" (d) The screen says plainly that copies in a sync service or Git may remain. No promise of remote erasure. |
| 1a | Security | Overwrite files before deleting; sign the purge file. | **Reject** | Overwriting is unreliable on modern disks and journaling file systems, so we must not promise it. Signing needs identities and keys we do not have; anyone who can write the folder can already edit the logs. Trust model: *the folder is trusted, the files in it are validated*. |
| 2 | Security | The Zotero connection is an open door to other local services. | **Accept in full** | Only the core talks to Zotero, never the window. Fixed address (`127.0.0.1`, Zotero's port), no redirects, no name lookup, only the personal-library items path (read), size and time limits, response checked for Zotero's version header and parsed strictly. Only on a user click, shown on screen. Stage 1 never fetches the PDF file or opens it. Groups libraries: later. |
| 3 | Sec, DevOps | No updates leaves people on vulnerable versions. | **Accept in part** | A **"Check for updates" button** (user click only, no check at start-up, no background). It fetches one small file with the latest version number and notes, over normal secure web. **Notify-only:** the download link is fixed inside the app, never taken from that file, so a faked file cannot send anyone elsewhere. |
| 3a | DevOps | Ship the in-place updater for Windows and macOS. | **Reject for the MVP** | Needs a protected signing key and a path that can replace the program on every machine. Revisit after the first release when a signing routine exists. |
| 3b | Security | Pin the web certificate. | **Reject** | Pins break at certificate renewal. The file holds a version number only and can trigger nothing. |
| 4 | QA | Test the web view in week 1, not at Stage 2. | **Accept** | Cheap now, a rewrite later. **Week-1 spike** on the three systems' web views, before any Stage 1 code. |
| 4a | DevOps | Commit to Tauri, drop the switch to Electron. | **Accept after the spike** | If it passes, there is no later switch; problems are fixed inside the web layer. If it fails on a system, we decide then, when nothing is built yet. |
| 5 | QA, DevOps | Zotero cannot be tested in CI without a stand-in. | **Accept** | Contract-first: record real answers once from a **dummy** Zotero library (no real papers or names), keep them as test files, serve them from a tiny local stand-in. A live check against real Zotero runs by hand before each release. |
| 5a | DevOps | Drop reading Zotero's database file. | **Accept** | Gone. If the one-day check shows the local interface does not give marks, we stop and bring the user a choice (§3, question 4) instead of reaching into Zotero's private files. |
| 6 | Sec, DevOps | Device name must be random, never a computer name. | **Accept the idea, reject the location** | Both put the id file inside the project folder. That folder is synced, so two machines would **share one id and write the same file**: the exact conflict the design avoids. The id lives in **per-user app storage on each machine**, never in the folder. If a file of ours changed in a way we did not write (cloned machine, restored backup), we pick a new id. |
| 7 | Security | Author label defaults to system user name. | **Accept (new)** | Default label "Me", user may set a name. Nothing from the computer goes into the files. |
| 8 | QA | Three early spikes. | **Accept all**, merged into the prototype plan (§4) | SPIKE-3 also checks that merging twice gives the same result. |

## 2. Final technical picks (MVP)

1. **Stand-alone desktop app**, core as a separate library plus a small command-line tool (same code reads and checks the files).
2. **Tauri v2**, confirmed by a week-1 web-view spike; no later switch to Electron.
3. **PDF.js and simplified anchoring** (Stage 2): page and box first, quote repair second, bounded fuzzy search, else *orphaned*, never deleted. Markdown anchors to the source text.
4. **History: one append-only line file per paper per device**; random device id kept outside the folder; read = union ordered by logical time, device, counter; purge event as above. No hash chain, no snapshots, no file watcher in the MVP.
5. **Notes: plain Markdown file + revisions in the log**; outside edits found on open.
6. **Zotero: local interface only**, read-only, brokered by the core, tested against recorded answers.
7. **Obsidian: one-way export on click**, into a marked subfolder; only our own unchanged files are overwritten.
8. **Own documented format, unstable v0**, with schema and sample files; W3C export of current state.
9. **Installers + package managers; manual notify-only update check.** No in-place updater.

Network rule, replacing "zero network": *no traffic unless the user clicks; two exceptions, both on click: the Zotero connection on this computer, and the update check.*

## 3. Decisions for the user

1. **Rust skill (decides the shell).** Does anyone who will maintain this write Rust? Recommend **yes → Tauri**. If no, Electron is the honest choice, and the core is then written in JavaScript.
2. **Update check.** Recommend the manual, notify-only button. Alternatives: no check at all (simplest, riskiest for users), or a full in-place updater (most convenient, needs a protected signing key and more work).
3. **What "forget" promises.** Recommend: erases here, hides everywhere, asks other machines, says clearly that sync copies may remain. Alternatives: local only (QA, simplest), or automatic erase everywhere (Security's first idea, cannot be guaranteed).
4. **If Zotero's local interface does not expose marks and dates** (one-day check): recommend **pause and return to the user** with the options (user-exported file import, small Zotero plug-in, reading Zotero's database). I will not choose silently.
5. **Signing costs** for the first public release, from devops (not verified by me): Apple developer programme about $99/year; Windows signing through a free open-source programme (needs checking). Recommend paying Apple's fee at the first public release and having devops confirm the Windows route.
6. **Author label default** "Me" instead of the computer user name. Recommend yes.

## 4. What the prototype plan must test (pass/fail numbers go there, not here)

(a) Week-1 web-view spike, three systems, large PDF. (b) Zotero one-day contract check on a real Zotero. (c) Merge test: many events over five device files, any order, same result, twice-merged same result. (d) Purge rule as a pure function. (e) One signature feature end to end: import, milestone, "what changed" list.

## 5. Next

I now update [ARCHITECTURE.md](../ARCHITECTURE.md) and the ADRs to the MVP (old ADRs marked superseded, not deleted). Requirements IDs are pm's: the architecture will map to capabilities and old IDs, and flag that the numbering follows pm's rewrite.
