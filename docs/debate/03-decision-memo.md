# Round 3: PM decision memo

Answers to [02-architect](02-architect.md), [02-security](02-security.md), [02-qa](02-qa.md). Builds on [01-proposal](01-proposal.md). Plain language; thresholds and technical choices come later.

## 1. The debate in one paragraph

All three reviewers accepted the direction (Zotero companion in front, history as the signature, open files as the moat) and attacked the same thing from three sides: **we promised too much, too early, with too much automatic behaviour.** Architect: the plan hides our own PDF reader, the costliest part. Security: quiet background capture looks like spying and "remember forever" is a liability. QA: the visual diff and the live Obsidian mirror cannot be shown to work reliably at launch. I accept most of this. The result is a smaller first release and a clearer story.

## 2. Objections and my answers

| # | From | Objection | Answer | Why |
|---|---|---|---|---|
| 1 | Architect | The bets hide our own PDF reader, the biggest cost and risk. Users ask why they should read here. | **Accept** | Real. Sequence the build, see §3. |
| 2 | Architect | Recorder and history viewer first, reader second. | **Accept the order, keep the reader in the MVP** | Tests "do people look back?" for weeks of work, not months. The user fixed "PDF + Markdown", so the reader stays in the MVP, behind a gate. |
| 3 | Architect | Say who owns a mark. | **Accept** | Marks imported from Zotero are read-only copies that show where they came from. Marks made here are ours. No two-way sync, no write-back to Zotero. |
| 4 | Architect, QA | What is one "version"? Every edit is noise. | **Accept** | A version is a **milestone**: one automatic milestone each time you import or refresh from Zotero and at the end of a reading session, plus any you name ("first skim", "rebuttal"). Every small change is still kept underneath, but the screen shows milestones. |
| 5 | Architect, QA | Cut the diff of marks from the MVP. | **Accept** | MVP ships a plain list of "what changed between two milestones" (marks added, removed, edited, and the notes text). The side-by-side visual diff waits for the validation result. It stays the signature promise, not the first deliverable. |
| 6 | Architect, QA, Security | Obsidian two-way or live mirror is a trap. | **Accept** | MVP is a one-way export on the user's action, into a separate folder they choose and confirm. Live updating and Obsidian edits flowing back: later. |
| 7 | Architect | Do not call the spec stable. | **Accept** | "Documented and versioned, unstable v0" until real outside use. |
| 8 | Architect | Do not let Markdown marking distract. | **Accept** | Markdown stays (user decision) but is not a marketed bet. |
| 9 | Security | Passive capture feels like spying; the take-home script contradicts our no-tracking pledge. | **Accept, with one change** | No background watching, ever, in the MVP. Palimpsest reads Zotero **only when the user clicks** open, import or refresh, and the screen says when. Repeated explicit refreshes still give the history we need. The validation script is run by hand, is short enough to read, has no network use, writes only a local file, and can be deleted at once. We lose automatic usage numbers and rely on interviews. |
| 10 | Security | "Remember forever" is a liability for confidential review. | **Accept** | Add a visible **"forget this paper's history"** control: one click, with a confirmation. The log stays append-only for the app (it never silently rewrites it); the owner decides to destroy it. Story: "tamper-proof by default, yours to erase". |
| 11 | Security | Cut shared bundles from the MVP. | **Accept** | It was already "after validation". |
| 12 | QA | Replace the time slider with milestone snapshots. | **Accept** | Same as #4. Cleaner to test and to explain. |
| 13 | QA | Launch gates: files never modified, no data loss on crash, accurate Zotero import, fast first page, a first-time user succeeds unaided. | **Accept as launch blockers** | Numbers are set in requirements, not here. Files-never-modified and no-data-loss are hard stops. |
| 14 | QA | Cut feature 6 to export only. | **Accept** | See #6. |
| 15 | Architect | "Open from Zotero" is fragile: Zotero must be open and the user must switch a setting on. | **Accept as a risk, check first** | Zotero's local interface only answers when the user enables it in Settings → Advanced ([Zotero docs](https://www.zotero.org/support/dev/web_api/v3/local_api), checked 2026-11-05). First task: one day to confirm that we can read marks, their dates and the PDF location. If not, the plan changes (see §5). |

Rejected: only one point, partly. I **reject storing only named milestones** (QA): we keep every change underneath because it costs little and keeps options open.

## 3. Recommended product

**Positioning:** Zotero's thinking layer in front, history as the signature, open files as the moat.
**USP (one sentence):** *Palimpsest remembers how your reading of a paper changed. It works beside Zotero and Obsidian and keeps everything in open files you own.*
**Signature features (5, scoped down):**
1. **Milestone timeline with restore**, plus a plain "what changed" list between two milestones. The visual side-by-side diff comes after the gate.
2. **Open from Zotero**: explicit import and refresh, read-only copies with their origin, and a link back to the exact page in Zotero.
3. **Quote with citation**: copy a highlight as a ready quote with citation key and link.
4. **Obsidian export**: one-way, on request, into an isolated folder.
5. **Open files and a small tool**: documented, versioned, unstable v0, as a by-product of building 1 to 4.

**Staged MVP (the key change):**
- **Stage 0, weeks: validate.** About 8 interviews, a clickable mock-up with a sign-up page, the hand-run "what changed" script for willing participants, and the one-day Zotero check.
- **Stage 1, recorder and history viewer.** Features 1 to 5 over marks imported from Zotero and Markdown notes. Reading stays in Zotero, which makes the switching cost zero. No reader of our own yet.
- **Gate:** do people open the history on their own, and does the Zotero and Obsidian loop stick?
- **Stage 2, our own PDF and Markdown reader**, so users can mark inside Palimpsest. Built only if the gate is passed.
If the gate fails, we drop the history headline, keep the bridge and the format, and have avoided the largest cost.

## 4. What the MVP must prove (updated)

1. People look back at history without being asked. 2. A Zotero user gets value in minutes, with no setup beyond switching Zotero's local access on. 3. Quote and Obsidian export are used every week. 4. Nothing is ever lost or modified (QA's two hard stops). 5. Researchers with confidential papers trust it enough to try (security's test).

## 5. Business model, sustainability, risks (changes only)

- Model unchanged: free core, no accounts, no telemetry, Apache-2.0 assumed. Smaller staged scope lowers cost and makes a first small grant application (NLnet needs a strong European dimension, [NLnet](https://nlnet.nl/propose/), checked 2026-11-05) more realistic.
- New risks: (a) Zotero's access is switched off by default for users, so onboarding must walk them through it; (b) recorder-first has no reader, so users may ask "where do I mark?" The answer is "in Zotero, as today"; (c) the gate may say "no", and we must accept it.
- **Name (correction to my earlier documents):** the bare name `palimpsest` is already taken on [crates.io](https://crates.io/crates/palimpsest) (a Pijul annotation tool, created July 2026), [npm](https://www.npmjs.com/package/palimpsest) (an image tool) and [PyPI](https://pypi.org/project/palimpsest/), and the GitHub account [`palimpsest`](https://github.com/palimpsest) exists (all checked 2026-11-05). Free today: GitHub names `palimpsest-app` and `palimpsest-reader`, crates `palimpsest-core` and `palimpsest-format`. Trademark clearance is **not checked**.

## 6. Decisions for the user (my recommendation first)

1. **Staged build, recorder first, reader behind a gate?** Recommend **yes**. Alternative: build the full reader at once (months more, same risk on an unproven headline).
2. **No background capture, ever in the MVP; only on explicit click?** Recommend **yes**.
3. **"Forget this paper's history" in the MVP?** Recommend **yes**. It softens "append-only" into "the app never rewrites it silently; you can erase it".
4. **Imported marks are read-only copies; never write back into Zotero in v1?** Recommend **yes**.
5. **Obsidian: one-way export on request, live mirror later?** Recommend **yes**.
6. **A "version" means a milestone (automatic at import and session end, plus named)?** Recommend **yes**.
7. **Name:** keep Palimpsest with prefixed identifiers (`palimpsest-app` and similar) and run a proper trademark search before the first public release? Recommend **yes**; spare names (Marginalia, ScribeTrace) are not checked either.
8. **Who recruits the 8 researchers?** Only the user has the network. Recommend: the user introduces about 10 contacts (labs, PhD friends); I supply the script and the mock-up brief.
9. **Apache-2.0** (assumed so far): confirm.

Next: mail architect "Memo ready" for Round 4 (high-level design). I have not touched `REQUIREMENTS.md`, `BUSINESS.md` or `USP-FEATURES.md`; they are now stale against this memo and need a rewrite once the user decides.
