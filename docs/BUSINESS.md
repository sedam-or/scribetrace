# ScribeTrace: Business Requirements & Strategy (v1.1)

**Date:** 2026-11-05 · **Author:** pm · **Status:** draft for the user's review · **Replaces:** v1.0 (a Palimpsest document with wider scope)
**Product name:** ScribeTrace (formerly Palimpsest, codename clickme) · **Folder:** `.scribetrace/` · **CLI:** `scribetrace`
Updated for the debate outcome ([03](debate/03-decision-memo.md), [08](debate/08-links-and-3-step.md)). Facts carry a URL and check date; anything not backed by a page we read is marked **unverified** (†).

## 1. Positioning
**One line:** *ScribeTrace remembers how your reading of a paper changed. It works beside Zotero and Obsidian and keeps everything in open files you own.*
**Position:** Zotero's thinking layer in front, history as the signature, open files as the moat ([01](debate/01-proposal.md), [03](debate/03-decision-memo.md)). It is a **companion, not a replacement**: no reference management, no reader to switch to in Stage 1, no accounts, no telemetry.
**Fixed by the user:** PDF + Markdown only; history is the core; companion to Zotero and Obsidian; open annotation format; local `.scribetrace/` folder; desktop; open source.

## 2. Name and trademark caveat
Palimpsest was rejected because its bare name is taken on crates.io, npm, PyPI and as a GitHub account (checked 2026-11-05, [NAMING](NAMING.md) §1). **ScribeTrace** was chosen by the user from three screened candidates (GlossTrace and ChronoMargin were the others).

| Check (2026-11-05, [NAMING §4](NAMING.md)) | Result |
|---|---|
| GitHub user/org `scribetrace`, crates.io, npm, PyPI, Homebrew, Flathub, winget, domains `.app` `.org` `.dev` | Free at the time of checking (one empty 0-star repo `TrungSy2106/Scribetrace` exists) |
| **USPTO, EUIPO, WIPO trademark databases** | **Unchecked.** They cannot be queried reliably by script; the maintainer must search manually before any public launch. State registries and non-WIPO countries also unchecked |
| Web search | No software product found; this is a screening, not legal clearance |

The earlier claim that "Palimpsest" was trademark-clear is withdrawn: it was never properly sourced. The repository `sedam-or/scribetrace` is private until the user decides otherwise ([DECISION-LOG](DECISION-LOG.md) l).

## 3. Problem and who has it
A researcher who lives in Zotero, writes in Obsidian or plain Markdown and returns to a paper months later cannot answer *"how did my reading of this paper change?"* Zotero stores annotations in its database, not in the PDF ([Zotero KB](https://www.zotero.org/support/kb/annotations_in_database), checked 2026-11-05); its documentation says nothing about an edit history of marks (absence of a statement, not proof). One user restored an old database to recover deleted annotations ([Zotero forum](https://forums.zotero.org/discussion/comment/458313), checked 2026-11-05).
- **P1, researcher / PhD / postdoc** (main user): wants the timeline of marks and notes, a ready quote with citation, notes in Obsidian, and files they own.
- **P2, peer reviewer:** confidential manuscripts; needs the **forget** control, no cloud, nothing changed in the original.
- **P3, power user / research software engineer:** wants an open, documented, scriptable format.

## 4. Competition (facts as of 2026-11-05)
Zotero, Obsidian and KOReader are partners or neighbours here rather than rivals; the table shows where each stores marks and whether it keeps a history. † = not backed by a page we read.

| Tool | Licence | Formats (relevant) | Where marks live | History of marks | Source |
|---|---|---|---|---|---|
| **ScribeTrace** | Apache-2.0 (planned) | PDF, Markdown | Open files in `.scribetrace/` | Yes, the core | this project |
| Zotero 7 | AGPL-3.0 | PDF, EPUB, snapshots | Zotero database | Not documented | [reader](https://www.zotero.org/support/reader), [KB](https://www.zotero.org/support/kb/annotations_in_database) |
| KOReader | AGPL-3.0 | PDF, EPUB and more | Sidecar files per book (†Lua tables) | No† | [GitHub](https://github.com/koreader/koreader) |
| Hypothesis | BSD-2 (client) | Web, online PDF | Their server | Minimal† | [web.hypothes.is](https://web.hypothes.is/) |
| Sioyek | GPL-3.0 | PDF only | Local database† | No† | [sioyek.info](https://sioyek.info/) |
| Okular | GPL† | PDF, EPUB and more (Markdown not on the formats page) | Not verified† | No† | [okular.kde.org](https://okular.kde.org/) |
| Obsidian + PDF++ | Core closed† / PDF++ MIT | PDF, Markdown | Markdown notes with links; PDF unmodified by default | Only through your own Git† | [PDF++](https://github.com/RyotaUshio/obsidian-pdf-plus) |
| Readwise Reader | Proprietary | Web, PDF, EPUB | Their cloud | No† | [pricing](https://readwise.io/read): $7.99–$9.99/month, billed yearly |
| MarginNote | Proprietary | PDF, EPUB (Apple platforms) | Own database† | No† | [marginnote.com](https://www.marginnote.com/) |
| LiquidText | Proprietary | PDF, Office† | Unverified† | Unverified† | [liquidtext.net](https://www.liquidtext.net/) |

**Integration facts we rely on** (all to be re-verified in Stage 0 on a real Zotero): Zotero's local interface at `127.0.0.1:23119` is off by default, read-only before Zotero 10, lists items and gives a `file://` address for attachments, and **does not document annotations** ([local API docs](https://www.zotero.org/support/dev/web_api/v3/local_api), checked 2026-11-05). The Zotero Connector saves a PDF "if you have access to it" or a web-page item ([Zotero docs](https://www.zotero.org/support/adding_items_to_zotero), checked 2026-11-05). Zotero Integration for Obsidian is GPL-3.0 and needs Better BibTeX († licence of Better BibTeX not checked).

**Differentiation:** none of the tools above, as far as we could verify, offers a milestone history of marks in open files that is independent of the reader; the claim "nobody has this" is **unverified** and the moat risk is listed in §8.

## 5. Open source, licence and funding
- **Licence: Apache-2.0** (decision k). Reasons: explicit patent grant and retaliation clause (§3) for a format others may embed; explicit contribution terms (§5); trademark paragraph (§6); compatible with GPLv3 tools, incompatible with GPLv2-only. Rust and many Tauri projects use Apache-2.0, often dual with MIT (†not re-checked). Alternative: MIT (shorter, no patent grant; the user uses MIT in another project, see DECISION-LOG).
- **Governance:** maintainer-led until real outside contributors exist; format changes by written proposal with a public review period; Contributor Covenant ([text](https://www.contributor-covenant.org/version/2/1/code_of_conduct/), checked 2026-11-05). Community files are owned by devops.
- **Funding, in order of realism:** GitHub Sponsors and Open Collective (small); **NLnet** (€5,000 to €50,000 for a first grant, open licence required, strong European dimension required, next deadline listed 2027-01-03; [NLnet](https://nlnet.nl/propose/), checked 2026-11-05: a European angle is unproven for us, so this is uncertain); Sovereign Tech Fund, Sloan, CZI: **eligibility not verified**. Free core forever, no paywalled research features.

## 6. Success measures (aspirations, not forecasts)
- **Product gate (Stage 1):** at least 4 of 8 testers open the history unprompted at least twice ([STRESS-TEST](STRESS-TEST.md) ST-01). Fail means: drop the history headline, keep Zotero import, quote, Obsidian export and the format, build no reader.
- **Journey and lightness:** the 3-3-3-3 journey and the lightness budget in [REQUIREMENTS §4](REQUIREMENTS.md), measured in the prototype before they are published.
- **Adoption without telemetry:** release downloads, package-manager install counts, issues and discussions, and scripts the community writes against the format. The old star and download goals of v1.0 (500 stars at launch, 5 labs by month 12) were guesses and are dropped until Stage 0 shows demand.

## 7. Roadmap (MoSCoW, staged)
| Stage | Content | Priority |
|---|---|---|
| 0 Validate | About 8 interviews, clickable mock-up, hand-run "what changed" script, one-day Zotero check | Must, first |
| 1 Recorder and history viewer | Import and refresh from Zotero, milestones and "what changed", notes with history, quote with citation, Obsidian export, forget, open format and CLI | Must |
| Gate | See §6 | Decision point |
| 2 Own reader | PDF and Markdown reading and marking inside ScribeTrace | Must, only after the gate |
| Later | EPUB, HTML, side-by-side diff, W3C export, `scribetrace://` links, direct-PDF add, live Obsidian mirror, group libraries, in-app updater | Could, each with a trigger in [ARCHITECTURE §7](ARCHITECTURE.md) |
| Never (v1) | Cloud, accounts, reference management, AI summaries, source editing, live web-page history | Won't |

## 8. Risks (full list: [STRESS-TEST](STRESS-TEST.md))
| Risk | Ref | Mitigation |
|---|---|---|
| Nobody looks back at history | ST-01 | Validate first; stage the build; fallback bridge plus format |
| Zotero's local interface lacks marks or breaks | ST-02 | One-day check; if it fails, stop and ask the user |
| First run takes too long | ST-03 | Guided setup, live connection check, usability test |
| Zotero or Obsidian ships history itself | ST-07 | Neutral open format across tools, local-first trust |
| Maintainer burnout; Rust skills missing | ST-13 | Small scope, boring tools, clear contributor docs, grant search |
| Name and trademark | ST-14 | Namespaced identifiers; manual trademark search before launch |
| Confidential history leaks through Git or sync | ST-06 | Forget control, ignore file, clear warnings |
| Scope creep re-inflating Stage 1 | ST-18 | No reader code until the gate passes |

## 9. Decisions
Decisions taken under delegation, with reasons and the list of what still needs the user: [DECISION-LOG](DECISION-LOG.md).
