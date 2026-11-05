# Round 1: PM proposal (business and product shape)

Fixed by the user: MVP reads PDF and Markdown only; annotation history is the core; companion to Zotero and Obsidian, not a replacement; open annotation format; local folder; desktop; open source. Plain product language, no technical choices. Facts checked 2026-11-05.

## 1. The problem and who has it most

Researchers read a paper many times over months. Their understanding changes, but every tool keeps only the **latest** state of their marks. Evidence:
- Zotero keeps annotations in its own database, not in the PDF ([Zotero docs](https://www.zotero.org/support/kb/annotations_in_database)). Its docs say nothing about edit history. A user who deleted annotations by mistake had to restore an old database backup ([Zotero forum](https://forums.zotero.org/discussion/comment/458313)).
- Obsidian's PDF plugins keep highlights as links inside notes ([PDF++](https://github.com/RyotaUshio/obsidian-pdf-plus)). That is good for linking, but it is not a reading tool with a history.

Who feels it most: **a PhD student or postdoc who lives in Zotero, writes in Obsidian (or plain Markdown), and returns to papers months later** (literature review, thesis, rebuttal). Second: peer reviewers who must not alter the author's file. Unvalidated: how many of them actually want the history. That is the first thing the MVP must find out.

## 2. Three positioning options

| # | Option | One-sentence USP | Strength | Weakness |
|---|---|---|---|---|
| A | **"Git for your paper annotations"** | See how your reading of a paper changed, and go back to any earlier state. | Clear, memorable, matches the fixed "history is the core". | Value unproven. A "nice to have" if people never look back. |
| B | **"Zotero's thinking layer"** | Open any Zotero paper, keep your marks and notes, and send your thinking to Obsidian. | Near-zero switching cost. Rides on existing habits. | Depends on Zotero and Obsidian. Thin if they improve their own readers. |
| C | **"The open annotation format"** | Your marks and notes live in plain files that any tool can read, now and in ten years. | May be the lasting asset. Attracts tool builders. | Slow adoption. Users do not buy a format. |

**Recommendation: B as the front door, A as the signature, C as the moat.** The one-liner: *"Palimpsest remembers how your reading of a paper changed. It works beside Zotero and Obsidian and keeps everything in open files you own."*

## 3. Candidate value features (plain words)

1. **Time machine:** scrub a paper's annotations back to any date. Restore a deleted or changed mark.
2. **Understanding diff:** compare your marks and notes at two moments, side by side ("what I thought in March vs now").
3. **Open from Zotero:** pick a paper from your Zotero library, with no copying or re-importing, and its existing highlights become the first entries of its history.
4. **Passive capture for Zotero:** quietly record changes you make in Zotero's own reader, so Zotero users get a history without changing habit.
5. **Quote with citation:** copy a highlight as a ready quote with its citation key and a link back to the page.
6. **Obsidian mirror:** your notes and highlights appear as normal Markdown files in your vault, with links back to the exact spot in the paper.
7. **Meaning labels:** tag highlights as claim, method, result, limitation or question, then export a cross-paper table.
8. **Git-friendly folder:** annotation changes show up as readable diffs per commit.
9. **Share a bundle:** send your annotations to a colleague and merge them into theirs.
10. **Open spec and a small command-line tool:** read, check and convert the files without the app.
11. **Re-read reminders:** resurface papers you marked "to re-read".
12. **Carry marks to a new version:** when a paper is replaced by v2, bring the marks along.

**Bets (5):** 1+2 (time machine and diff, the signature), 3 (open from Zotero, the front door), 5 (quote with citation, the daily habit), 6 (Obsidian mirror, the daily habit), 10 (open spec and tool, the moat).
**Why these:** each answers one of the four debate points: unproven history (1+2), switching cost (3, 5, 6), scope (everything else waits), and the format as asset (10).
**Not now:** 4 is the cheapest way to *test* history (see §4), so it may become a test tool before it is a feature. 7, 9, 12 come after history is proven. 8 comes almost free from the folder design. 11 is weak. Two ideas rejected on purpose: writing back into Zotero (risk of damaging the user's library) and AI summaries (privacy, outside the focus).

## 4. What the MVP must prove

1. **People look back.** Users open the history or the diff on their own, more than once, without being asked. This is the kill or continue test for the headline.
2. **Switching cost is near zero.** A Zotero user reads a first paper with their old marks in under a few minutes, with nothing to set up beyond the library.
3. **It sticks in a real workflow.** The quote and Obsidian features are used weekly. If only these are used, history becomes a background feature, not the headline.
4. **The format is good enough that someone else reads it.** At least one outside person builds or runs something on it.

Cheap validation before heavy building: about 8 researcher interviews, a clickable mock-up with a sign-up page, and a small take-home script that records a Zotero user's annotation changes for four weeks and shows them a "what changed" report. If nobody reads the report, we do not build the full history interface. If history fails, we drop the headline promise, not the work: the same foundations still serve the Zotero and Obsidian bridge.

## 5. Business model and sustainability (open source)

- Free forever for the core app, no accounts, no telemetry. Apache-2.0 assumed (patent grant).
- Costs are small if scope is small: build servers, signing certificates, a website.
- Funding, in order of realism: donations (GitHub Sponsors, Open Collective), then grants. NLnet gives €5,000 to €50,000 for first grants to open-licensed projects and needs a strong European dimension ([NLnet](https://nlnet.nl/propose/); the next deadline listed was 2027-01-03). Sovereign Tech Fund, Sloan and CZI: not checked here.
- Possible later income without paywalling the core: paid sync-free "lab support", sponsored integrations. Not for the MVP.
- Adoption signals without telemetry: package-manager installs, release downloads, outside tools using the format, issues and discussions.

## 6. Biggest risks

1. **History is a vitamin, not a painkiller.** Mitigation: validate first; keep the bridge features that stand alone.
2. **Zotero and Obsidian dependence.** Zotero's local API must be switched on by the user and only works while Zotero runs ([Zotero local API](https://www.zotero.org/support/dev/web_api/v3/local_api)). Obsidian could ship better PDF history. Mitigation: the open format keeps us useful to other tools.
3. **Two places for marks.** Users who annotate in both Zotero and Palimpsest will be confused. Mitigation: one-way import first; clear "where is my mark" messaging.
4. **Scope creep back into a reference manager or a note app.** Mitigation: the "companion" rule and the cut list above.
5. **Trust.** A tool that records everything must be clearly local and deletable. (Security to argue.)
6. **Name.** My earlier availability claims were partly wrong. The bare names `palimpsest` on crates.io, npm and PyPI are already taken, and the GitHub account `palimpsest` exists. A project org and prefixed package names are free today. Trademark clearance is unchecked. This goes to the user in Round 3.
