# ScribeTrace: Go-to-market plan

**Date:** 2026-11-06 · **Author:** pm · **Status:** draft for the user's review · Related: [BUSINESS](BUSINESS.md), [REQUIREMENTS](REQUIREMENTS.md), [DECISION-LOG](DECISION-LOG.md), [STRESS-TEST](STRESS-TEST.md), [VALIDATION-PROTOCOL](VALIDATION-PROTOCOL.md).
Rules: no telemetry, no paid ads, nothing that needs the website server. Facts carry a URL and date; **unverified** means we did not read a page that confirms it. The repo is public, but **we do not promote the product until the gates in §4 are met.**

## 1. Primary segment: win one first
**PhD students and postdocs in the social sciences and humanities who already use Zotero 7 and write in Markdown or Obsidian.**

| Why this segment | Evidence |
|---|---|
| Zotero's users skew toward the social sciences and humanities | 2018 PLOS ONE survey of Mendeley and Zotero users ([abstract](https://econpapers.repec.org/RePEc:plo:pone00:0198033), checked 2026-11-06). Old, and it predates Zotero 7; current user counts: **unverified** |
| Their pain is exactly ours: long reading projects, papers re-read months later, thinking written in notes | Our own problem statement ([BUSINESS §3](BUSINESS.md)); one forum case of a user restoring a database to recover deleted annotations ([Zotero forum](https://forums.zotero.org/discussion/comment/458313), checked 2026-11-05). Frequency of this pain: **unverified**, which is what Stage 0 tests |
| They already glue Zotero to Obsidian, so our companion position is a fit | Zotero Integration plugin for Obsidian exists (GPL-3.0, needs Better BibTeX; [BUSINESS §4](BUSINESS.md)). Size of the overlap: **unverified** |

Not first: peer reviewers (they need the forget control, a second wave), STEM labs and power users (reachable later through the open format).

## 2. First 10 places and people (in order)
| # | Where / who | Why | Source |
|---|---|---|---|
| 1 | An open sign-up invitation posted by the user in their own channels, for 8 diary-study and 8 usability participants (the user does not see who joins, to limit courtesy bias) | People we can recruit ethically before launch | [VALIDATION-PROTOCOL](VALIDATION-PROTOCOL.md) §1; [DECISION-LOG](DECISION-LOG.md) "What needs you" |
| 2 | Their supervisors and lab or reading groups (via #1) | Real reading projects, honest feedback | unverified |
| 3 | University librarians who teach Zotero workshops | They know the pain and the setup problems | unverified (existence of such workshops is common; not checked per site) |
| 4 | Obsidian forum, category **Share & showcase** | The place to show finished plugins and workflows ([forum](https://forum.obsidian.md/), checked 2026-11-06); self-promotion rules are on its guidelines page, **not read yet** | partly verified |
| 5 | Zotero forum | Third-party tools are announced there, but the right category and the posting rules were **not confirmed** ([forum](https://forums.zotero.org/), checked 2026-11-06). Read the rules first | partly verified |
| 6 | Obsidian Zotero Integration plugin's GitHub issues or discussions | Users who connect both tools; ask for feedback, do not advertise | unverified |
| 7 | r/PhD | Large audience of doctoral students; **rules and size not checked** | unverified |
| 8 | r/Zotero and r/ObsidianMD | Same users, narrower; **existence and rules not checked** | unverified |
| 9 | Academic Mastodon / Bluesky tags (for example #AcademicChatter) | Researchers share tools there | unverified |
| 10 | Show HN | Broad developer audience; only after every gate in §4. Rules: **not checked** | unverified |

Rule for #4 to #10: one honest post per place, state the pre-alpha limits, answer every reply, no cross-posting burst.

## 3. Positioning, proof, and what we must not claim
**Positioning:** *ScribeTrace remembers how your reading of a paper changed. It works beside Zotero and Obsidian and keeps everything in open files you own.*

| Proof point | Shown by the prototype how |
|---|---|
| 1. **First value fast:** import marks, see a milestone and "what changed" within minutes | 3-3-3-3 usability test: first mark within 5 min of install ([VALIDATION-PROTOCOL](VALIDATION-PROTOCOL.md)) |
| 2. **Trust:** your papers and Zotero are never modified, and nothing leaves your computer except a click that talks to Zotero | Byte-identical check before and after; network-namespace test with one loopback connection ([SEC-1](SECURITY-REQUIREMENTS.md)) |
| 3. **Open files:** you can read your history without the app | `scribetrace check` reads the sample and real files; published format and sample files |

**Do not claim yet:** "the first or only tool with annotation history" (unverified); any lightness number (until measured); "no data loss" (until the crash test passes); "tamper-proof" or "securely erased" (forget cannot remove sync or Git copies); support for web pages or live-page history; "works with every Zotero" (needs the live check); "stable" format (it is unstable v0); trademark-cleared (it is not); any revenue or user numbers.

## 4. Launch checklist (every box before the first Show HN or forum post)
| Gate | Evidence |
|---|---|
| Live Zotero check done: marks, dates, page locations readable | SPIKE-3 report ([PROTOTYPE-PLAN](PROTOTYPE-PLAN.md)) |
| Shell and webview spike passed on Linux, Windows, macOS | SPIKE-4 report |
| Lightness numbers **measured** and published as measured | CI size and memory results |
| 3-3-3-3 usability test passed: 4 of 5 unassisted | [VALIDATION-PROTOCOL](VALIDATION-PROTOCOL.md) §3 |
| Product gate passed: people look back | [VALIDATION-PROTOCOL](VALIDATION-PROTOCOL.md) §2 |
| Manual US, EU and WIPO trademark search done | Search notes ([NAMING](NAMING.md)) |
| Licence confirmed by the user; README states pre-alpha limits | [DECISION-LOG](DECISION-LOG.md) k |
| Files never modified and crash durability tests pass | QA gates 1 and 2 |

## 5. The 5 metrics that matter (no telemetry)
| Metric | How measured without tracking |
|---|---|
| 1. **Do people look back?** share of study participants who open the history on their own | Diary study counts the participant checks and shares ([VALIDATION-PROTOCOL](VALIDATION-PROTOCOL.md)) |
| 2. **Time to first value** | Observed usability sessions with a stopwatch; after launch, an optional poll in GitHub Discussions |
| 3. **Adoption** | GitHub release download counts, package-manager install counts (unverified availability per manager) |
| 4. **Staying** | Returning authors in issues and discussions; an optional poll "do you still use it?" 30 days after launch, run off-app |
| 5. **Format uptake** | Third-party scripts, tools or forks that read the format (searching GitHub) |
Stars and views are not metrics. A number we cannot measure without tracking users is not measured.

## 6. Sustainability for an Apache-2.0 project
| Option | Cost | Fit with local-first, no account | Recommendation and trigger |
|---|---|---|---|
| **GitHub Sponsors / Open Collective** | Almost none; fiscal-host fees **unverified** | Full | Open after the gates; no income expected, no revenue promise |
| **Grant: NLnet** | Application work only | Full. First grants €5,000 to €50,000, open licence required, "strong European dimension" required, "proposals and projects should be made by humans, not AI"; next deadline listed 2027-01-03 ([NLnet](https://nlnet.nl/propose/), checked 2026-11-06) | Apply only when the gate passes **and** a human applicant with a clear European link writes the proposal. Because AI agents write our first code, say so honestly. Do not rush for 2027-01-03 |
| **Grant: Sovereign Tech Fund** | High | Neutral | **Not a fit now:** funds base technology other software depends on, minimum cost of work above €50,000, user-facing apps not currently sought ([STF](https://www.sovereign.tech/programs/fund), checked 2026-11-06). Revisit only if the format becomes shared infrastructure |
| Zotero-ecosystem funds | n/a | n/a | No such fund found; **unverified**. Ask in the Zotero forum only after launch |
| **Open core** (paid extras on a free core) | Splits the community; legal effort | Weak: the open format is the moat | **No, pledged** ([DECISION-LOG](DECISION-LOG.md) o): every feature of the app stays Apache-2.0 |
| **Optional paid service** (for example hosted sync) | Server, accounts, support | Fits only if never required: the app itself needs no account or server (pledged, decision o) | **Not for v1; no pledge against it ever** (decision o). Revisit after the product gate. It would need its own decision, must never be required, and must not reduce the free app |

## 7. Decisions (all four taken 2026-11-06; see [DECISION-LOG](DECISION-LOG.md) m, n, s, o)
1. **Primary segment:** PhD students and postdocs in the social sciences and humanities with Zotero 7 and Markdown/Obsidian. Recommend **yes**. **Decided: yes** (m).
2. **No promotion until all gates in §4 are met**, even though the repo is public. Recommend **yes**. **Decided: yes** (n).
3. **NLnet:** will a human with a European link apply, and when? Recommend: decide after the product gate. **Decided: after the product gate, human owner applies** (s).
4. **Pledge:** no open core, and the app never needs an account or a server, in writing in the README. **Decided: yes to both; no pledge against optional paid services** (o). This replaces the earlier recommendation, which also ruled out paid sync.
