# Round 2: Security Review (Trust, Privacy, and Reputational Risk)

**Author:** Security Lead  
**Context:** Review of `docs/debate/01-proposal.md` for the PDF + Markdown MVP. Plain product language; no technical IDs or framework choices.

---

## 1. Where I Agree

* **Zero-account, local-first is our strongest sales pitch:** In academic research, papers are often confidential manuscripts, grant proposals, or embargoed peer reviews. Closed cloud tools (Readwise, LiquidText) are non-starters for labs under ethics agreements or NDAs. Being strictly local and transparent is our primary competitive trust moat.
* **Refusing to write back into Zotero:** PM's deliberate choice to reject writing into Zotero's database or touching original PDFs is essential. Corrupting a researcher's five-year Zotero library or invalidating a digitally signed PDF would destroy Palimpsest's reputation overnight. One-way read-only import is the only defensible path.
* **The "Zotero Thinking Layer" positioning (Option B):** Keeping Zotero as the sacred library of record and positioning Palimpsest as the focused desk where thinking happens keeps the trust boundary clean.

---

## 2. Strongest Objection: The "Quiet Sniffer" and the Phantom History Trap

My sharpest objection is to **Feature 4 ("Passive capture for Zotero: quietly record changes")** and the proposed **take-home script that records annotations for four weeks**.

1. **The Spyware Accusation:** Academic communities are fiercely protective of workflow privacy. If Palimpsest ships a background daemon that monitors Zotero's local files or quietly records background activity, researchers will brand it as spyware. Academic IT departments and institutional review boards will prohibit installation on university-managed laptops.
2. **The "Phantom History" Liability in Confidential Review:** In academic peer review (Persona 2), confidentiality is legally binding. If a reviewer highlights an identifying remark on a double-blind manuscript, realizes their mistake, and deletes it in Zotero, they reasonably expect that remark to be gone. If Palimpsest "quietly captured" it into an append-only log that persists forever on disk, Palimpsest has created an accidental leak hazard.
3. **Validation Risk:** Distributing an untracked local script to "record a user's Zotero changes for four weeks" contradicts our core pledge of zero telemetry. Even if the data stays local, asking researchers to install an unverified script that logs their daily reading habits will trigger immediate backlash.

---

## 3. What I Would Cut

1. **Cut Feature 4 (Passive capture) entirely:** Delete monitoring and background scraping. Palimpsest must only ingest annotations when the user explicitly clicks "Open from Zotero" or runs a conscious import. Trust requires that the user always knows when recording starts.
2. **Cut Feature 9 (Share a bundle) from MVP:** Merging annotation files from colleagues sounds innocent, but importing external files into an immutable local history is how hostile payloads enter a user's workspace. Until our data format has a mature validation and sandboxing story, collaborative bundle merging introduces unnecessary risk.
3. **Narrow Feature 6 (Obsidian mirror) to user-initiated export:** Writing live, uncontained files directly into an external Obsidian vault folder creates sync collision hazards and risks leaking confidential reading notes if that vault is synced to a public GitHub repo or shared cloud. The MVP should write to a dedicated, isolated subfolder only after explicit confirmation.

---

## 4. One Alternative I Would Defend: The "Verifiable Clean-Room" & The Right to Forget

PM proposes that "everything is remembered forever." For researchers bound by confidentiality agreements, **unconditional immutability is a liability, not a feature.**

I defend adding an explicit **"Purge History" (Clean-Room / Right to Forget)** control:
* **Why it matters:** Peer reviewers, medical researchers handling clinical trial preprints, and patent attorneys cannot adopt a tool that makes it impossible to scrub a document's history. When a review cycle ends or a manuscript is rejected, the user must be able to securely purge a document's historical log with one click, leaving no ghost records behind.
* **Product framing:** We don't weaken the append-only value; we frame it as **"Tamper-proof by default, sovereign by design."** You own the timeline, which means you have the authority to destroy it when ethics or institutional rules require it.
* **Positioning impact:** This turns our greatest risk (accidental retention of confidential notes) into a major institutional selling point that closed cloud readers can never match.
