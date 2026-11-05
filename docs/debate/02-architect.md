# Round 2: Architect (product shape and effort)

Plain language, no technology choices. Effort is relative (S/M/L), assuming a very small team, 1–2 part-time maintainers. Reply to [01-proposal](01-proposal.md).

## Where I agree

- **B in front, A as signature, C as moat** is the right order. Keep it.
- **Validate before building.** The four-week "what changed" report is the best idea in the paper. Cheap, honest, and it tests the headline.
- **One-way import from Zotero first.** Never writing back into Zotero is the right safety line.
- Folder-friendly diffs and the format come almost free *if* history is the foundation. That is the real reason to bet on history: it is cheap to store, hard to copy.

## Strongest objection: the bet list is three products, and the biggest cost is not on it

Bets 1+2, 3, 5, 6 and 10 hide **our own PDF reader**: selecting text, putting marks in the right place, keeping them in the right place. That is the largest and riskiest piece of work, and it is exactly where we compete head-on with Zotero's reader. "Companion, not replacement" and "build a third place to read" pull in opposite directions. Users who read in Zotero will ask why they must read here.

Second: **who owns a mark?** If a Zotero highlight is imported, is it a copy, a link, or ours? Without a one-line answer the "two places for marks" risk becomes a support problem. My proposal: imported marks are **read-only copies with their origin shown**; marks made here are ours; no two-way sync, ever, in v1.

Third: **what is one "version"?** Every small edit, or one per reading session? Per-edit history is noise nobody opens; the "people look back" test will fail for UI reasons, not for lack of need. This is a product decision to make before the build, and it matters more than the storage.

## Effort, honestly

| Bet | Effort | Where the cost really is |
|---|---|---|
| Time machine | S–M | Cheap to store, the design of the screen is the work. |
| Understanding diff | M–L | Notes diff is easy; diff of *marks across versions of a file* is hard. Start with notes. |
| Open from Zotero | M | Depends on Zotero being open and set up by the user; fragile, outside our control. |
| Quote with citation | S | Only as good as the data Zotero gives us. |
| Obsidian mirror | S one-way, L two-way | Two-way means merging edits made in two apps. Do one-way for marks, "append as new entry" for notes edited in Obsidian. |
| Open spec + tool | S if a by-product, L if designed for outsiders first | Publish as **unstable v0**. A frozen spec before real use is a liability. |

## What I would cut

1. **Diff of marks** from the MVP; ship diff for notes and the timeline for marks.
2. **Two-way Obsidian sync.**
3. **Rendered-Markdown marking as a signature surface.** Markdown is the cheap format; do not let it distract from PDF risk.
4. Any promise of the spec being "stable". Promise "documented and versioned".

## Alternative I would defend: recorder first, reader second

Keep the fixed MVP (PDF and Markdown), but change the **order**. Ship first a small **recorder + history viewer**: it reads Zotero's marks, keeps their history, shows the timeline, feeds Obsidian. No reader of our own yet. This is pm's four-week test grown into a first release. It takes weeks, not months, and it answers "do people look back?" with real data before we invest in the expensive part. Gate: if people open the history, build the reader so they can mark inside Palimpsest; if not, we saved the largest cost and still have the bridge and the format.

Honest limits: it only sees changes from the day it is installed, and it can miss a change made and undone between two looks. Whether Zotero lets us read marks and their dates reliably is **unverified**; I would check that first.
