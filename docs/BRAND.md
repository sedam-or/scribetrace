# ScribeTrace: Brand strategy and voice

**Date:** 2026-11-06 · **Author:** pm · **Status:** draft for the user's review · Related: [GO-TO-MARKET](GO-TO-MARKET.md) §3, [BUSINESS](BUSINESS.md), [NAMING](NAMING.md), [README](../README.md), [DECISION-LOG](DECISION-LOG.md) decisions m to o.
Scope: words and strategy only. Colours, logo and layout belong to the designer (#18). Every claim here must stay inside the do-not-claim list in [GO-TO-MARKET](GO-TO-MARKET.md) §3.

## 1. Purpose, audience, positioning
- **Purpose:** help researchers remember how their reading of a paper changed, and keep that memory in files they own.
- **Audience (first):** PhD students and postdocs in the social sciences and humanities who use Zotero 7 and write in Markdown or Obsidian ([decision m](DECISION-LOG.md)). They are skilled readers, wary of new tools, short of time, and often handling unpublished work.
- **Positioning (from GO-TO-MARKET):** *ScribeTrace remembers how your reading of a paper changed. It works beside Zotero and Obsidian and keeps everything in open files you own.*

## 2. Personality: three traits
| Trait | It **is** | It **is not** |
|---|---|---|
| **Plain** | Short words, concrete nouns: marks, history, milestone, forget | Jargon, buzzwords, "leverage", "AI-powered" |
| **Honest** | Says what is built, what is a target, what is unverified | Apologetic, hedging every line, or promising what the docs do not |
| **Calm** | A careful colleague who respects your time and your confidentiality | Cute, gamified, urgent, emoji-led, productivity-guru |

## 3. Voice and tone rules
1. Lead with what the reader can do, in their words. 2. Name the status of anything unbuilt ("pre-alpha", "target, to be measured"). 3. Numbers only when measured. 4. One idea per sentence; no exclamation marks. 5. Credit Zotero and Obsidian as the places where reading and writing happen. Tone shifts by place: README and docs are factual; forum posts are modest and answer every reply; error messages say what happened and the one thing to do next.

| # | Do | Don't |
|---|---|---|
| 1 | "See which highlights you added or deleted since March." | "Leverage temporal annotation intelligence." |
| 2 | "Pre-alpha: nothing to install yet." | "Coming soon! Join the reading revolution." |
| 3 | "Target: an installer of 20 MB or less, to be measured." | "The lightest reader ever built." |
| 4 | "Forget removes this paper's history on this computer. Copies in sync services or Git may remain." | "Your data is securely erased." |
| 5 | "Keep reading in Zotero. ScribeTrace remembers what changed." | "Zotero forgets everything. We fix it." |

## 4. Messaging hierarchy
**Tagline options (pick: 1).**
| # | Option | For | Against |
|---|---|---|---|
| 1 | **Remember how your reading changed.** | States the one job; matches the positioning and the README; provable by the history view and the diary study | Plain, not catchy |
| 2 | The history of your reading, in files you own. | Joins the job and the open-files proof | Long; "history" fails as a headline if the product gate fails ([BUSINESS §6](BUSINESS.md)) |
| 3 | Your marks have a past. | Memorable | "Marks" is our word, not yet the reader's; vague without the sentence after it |

Pick 1 because a stranger can check it against the product in a minute and it makes no promise beyond the history. If the product gate fails, options 2 and 3 are the first to change.

**One-liner:** ScribeTrace is a small desktop companion to Zotero and Obsidian that keeps the history of your marks and notes in open files you own.

**Three proof points** (each must be shown by the prototype before it is used; status today: not yet shown, see [GO-TO-MARKET](GO-TO-MARKET.md) §3):
1. **First value fast:** import marks and see "what changed" within minutes (3-3-3-3 usability test).
2. **Trust:** your papers and Zotero are never modified, and nothing leaves your computer except a click that talks to Zotero (byte-identical and network tests).
3. **Open files:** you can read your history without the app (`scribetrace-cli check`, published format).

## 5. Naming rules
- **Spelling:** **ScribeTrace**, one word, capital S and T. Never "Scribe Trace", "Scribetrace", "SCRIBETRACE" or "Scribe-Trace" in text. In code, paths and commands use lower case in code style: `scribetrace` (the app), `scribetrace-cli` (the command-line tool), `.scribetrace/`. The codename **Palimpsest** is retired; do not use it in new text.
- **No trademark symbols.** The name is not cleared in the US, EU or WIPO databases ([NAMING](NAMING.md), [DECISION-LOG](DECISION-LOG.md) a); do not write "™", "®" or "our trademark" until a manual search is done.
- **Zotero and Obsidian:** use their names only to say what we work with, in plain text: "a companion to Zotero", "exports to Obsidian". Never "Zotero ScribeTrace" or "ObsidianTrace", no domain, package or repository name containing their names, and no use of their logos, icons or colours. Never say "official", "certified", "partner", "plugin for Zotero/Obsidian" (we are neither), or "recommended by". On the first mention in a page or post add: *ScribeTrace is an independent project, not affiliated with or endorsed by Zotero or Obsidian.*
- **Why:** Zotero's page says its mark "may not be used in connection with any product or service that is not affiliated with the Corporation" and not "in any manner that is likely to cause confusion"; it does not mention compatibility statements either way ([Zotero trademark page](https://www.zotero.org/support/terms/trademark), opened 2026-11-06). Obsidian says its name, logo and app icon are trademarks, asks that the logo not be edited or recoloured, and asks for contact before commercial use ([Obsidian brand page](https://obsidian.md/brand), opened 2026-11-06). Whether naming them to describe compatibility is acceptable is a legal question we have **not** answered; the wording above is the cautious form. Ask each project (or a lawyer) before launch.
- **Our own vocabulary:** *mark* (highlight, underline or note), *history*, *milestone*, *forget*, *project folder*. Avoid "annotation" in the interface; use it in the format documentation because that is the standard term.

## 6. Visual direction keywords (for the designer; no colours or logo here)
- **Feel:** quiet, editorial, text-first, a scholar's margin; layered writing and a trace over time (the name: a scribe's hand and its trace); calm density, like a good reading view rather than a dashboard.
- **Motifs to explore:** margin notes, strata or stacked pages, a thin timeline, ink that fades into earlier layers. A mark readable at 16 px and in one colour.
- **Avoid:** graduation caps, owls, open-book clichés, neon, sparkles or robot imagery (we have no AI feature), gamified badges, and any resemblance to the Zotero or Obsidian marks.
- **Practical:** works in light and dark; high contrast and accessible; native-feeling on Linux, macOS and Windows; fonts and assets under open licences, since the project is Apache-2.0; the wordmark must also work as plain text in a terminal and in a README.
- **Tone check for any text on a design:** it must pass the do/don't table in §3.
