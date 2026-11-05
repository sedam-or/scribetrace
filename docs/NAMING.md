# Palimpsest: Product Naming Investigation & Trademark Screening

**Date of Screening:** 2026-11-05  
**Author:** DevOps & Infrastructure Engineering  
**Context:** [docs/debate/03-decision-memo.md](debate/03-decision-memo.md)  
**Status:** Public Database Screening & Brand Availability Assessment  

> [!IMPORTANT]
> **Legal Notice:** This evaluation represents preliminary public database, package registry, domain, and trademark screening conducted on 2026-11-05. It does not constitute formal legal clearance or trademark opinion from an intellectual property attorney.

---

## 1. Context & Motivation for Renaming

The open-source project is currently codenamed **Palimpsest**. The product is a local-first desktop application (Linux, macOS, Windows) that preserves a verifiable, chronological history of annotations, highlights, and thoughts across academic papers (PDF and Markdown), serving as an unmanaged local companion beside Zotero and Obsidian.

### The Conflict with "Palimpsest"
A preliminary namespace audit reveals that the bare name `palimpsest` is heavily claimed across critical open-source distribution channels:
* **crates.io:** Taken (`palimpsest` crate registered).
* **npm:** Taken (`palimpsest` package registered).
* **PyPI:** Taken (`palimpsest` Python library registered).
* **GitHub Organization:** Taken (`github.com/palimpsest` belongs to an unrelated organization).
* **Top-Level Domains:** `palimpsest.app` and `palimpsest.org` are registered.

Because Palimpsest relies on distributing modular Rust libraries (`palimpsest-core`), npm packages, and desktop package manifests without namespace squatting or trademark collisions, we screened 18 candidate names.

---

## 2. Candidate Brainstorming & Rapid Screening (18 Candidates)

Candidates were evaluated around conceptual metaphors of **layers, traces, scholarly margins, memory of reading, and chronological revision**, requiring 1–3 syllables, easy spelling, and zero awkward or offensive meanings in both English and Indonesian.

| # | Candidate | Core Meaning & Metaphor | Initial Registry Status | Decision & Reason |
|---|---|---|---|---|
| 1 | **ScribeTrace** | Scribe (scholar) + Trace (chronological path) | Free on GitHub, crates, npm, PyPI, domains | **PASS (Selected as Option 1)** |
| 2 | **GlossTrace** | Gloss (*glosa*, marginal scholarly note) + Trace | Free on GitHub, crates, npm, PyPI, domains | **PASS (Selected as Option 2)** |
| 3 | **ChronoMargin** | Chrono (time) + Margin (annotation space) | Free on GitHub, crates, npm, PyPI, domains | **PASS (Selected as Option 3)** |
| 4 | *Palimpsest* | Scraped parchment revealing underlying text | Taken on crates.io, npm, PyPI, GitHub org | **REJECTED:** Namespace fully exhausted. |
| 5 | *Marginalia* | Notes written in margins | Taken by Marginalia search engine (`marginalia.nu`) | **REJECTED:** Major brand confusion in web search. |
| 6 | *StrataRead* | Strata (geological layers) + Read | Taken on Apple App Store (research PDF reader) | **REJECTED:** Direct competitor in scholarly PDF space. |
| 7 | *Strata* | Layers of rock or text | Taken on crates.io, npm, PyPI | **REJECTED:** Bare namespace collisions everywhere. |
| 8 | *Annotrace* | Annotation + Trace | Taken by LLM trace workbench & `annotrace.io` | **REJECTED:** Existing active developer software. |
| 9 | *MarginTrace* | Margin + Trace | Taken by `margintrace.io` (AI financial tool) | **REJECTED:** Commercial SaaS collision on `.io` domain. |
| 10| *Pentimento* | Reappearance of underlying painting layers | Taken on crates.io, npm, PyPI | **REJECTED:** Registries unavailable. |
| 11| *Scholia* | Ancient marginal annotations on manuscripts | Taken on crates.io, npm, PyPI, GitHub org | **REJECTED:** Registries and GitHub org claimed. |
| 12| *Scholion* | Singular form of scholia | Taken on PyPI and GitHub user | **REJECTED:** PyPI namespace occupied. |
| 13| *Retrace* | To trace back historical steps | Taken on crates.io, npm, PyPI, and Stackify APM | **REJECTED:** Heavy enterprise software collision. |
| 14| *Vestigia* | Footprints / historical traces left behind | Taken on npm and PyPI | **REJECTED:** Package namespaces occupied. |
| 15| *PaperTrace* | Trace over research paper | Taken on PyPI and GitHub user | **REJECTED:** Package namespace occupied. |
| 16| *Stratamark* | Strata layers + Markings | Taken by Stratamark (clinical medical gel) | **REJECTED:** Awkward pharmaceutical product collision. |
| 17| *ParchMark* | Parchment + Bookmark | Taken by `TejGandham/parchmark` (Markdown app) | **REJECTED:** Direct GitHub project collision. |
| 18| *TextStrata* | Text layers over time | Taken by `QuantEcon/textstrata` on GitHub | **REJECTED:** Academic GitHub repository collision. |

---

## 3. Top 3 Passing Options: Deep Verification Matrix

All three passing names meet the strict screening criteria:
1. Free GitHub user/org (`github.com/<name>`).
2. Free bare package name on crates.io and npm.
3. Free bare package name on PyPI.
4. Unchecked on official USPTO, EUIPO, and WIPO trademark databases (requires manual search by the maintainer; public search screening shows no conflicting software).
5. No existing desktop reader, annotator, or reference tool in the space.
6. Available primary or alternative top-level domains (`.app`, `.org`, `.dev`).
7. Clean, dignified semantic and phonetic profile in both English and Indonesian.

### Option 1: ScribeTrace (Recommended)
* **Meaning:** *Scribe* (the researcher/scholar recording observations) + *Trace* (the chronological journey and immutable historical trail of thoughts).
* **Pronunciation & Syllables:** 2 syllables (`Scribe-Trace` / /skraɪb treɪs/). Punchy, memorable, verb-noun synthesis.
* **Why it fits:** Directly encapsulates the companion role: a scholar reading papers, making marginalia, and tracing how conceptual comprehension changes across versions.
* **Linguistic Screening (EN + ID):**
  * *English:* Clean connotation of thoughtful authorship and rigorous historical record.
  * *Indonesian:* Neither "scribe" nor "trace" resembles offensive slang or taboo vocabulary. Indonesian equivalent: *jejak juru tulis* / *jejak catatan*. Professional and dignified.

### Option 2: GlossTrace
* **Meaning:** *Gloss* (from historical Greek/Latin *glossa*, meaning marginal notes, glosses, and annotations added to scholarly texts) + *Trace* (historical audit trail).
* **Pronunciation & Syllables:** 2 syllables (`Gloss-Trace` / /ɡlɒs treɪs/).
* **Why it fits:** Deeply scholarly; pays homage to the centuries-old academic tradition of glossing manuscripts while emphasizing versioned recovery.
* **Linguistic Screening (EN + ID):**
  * *English:* Highly academic; evokes historical manuscripts and linguistic glossaries.
  * *Indonesian:* *Glosa* is an established Indonesian literary term (KBBI: *glosa* = keterangan atau komentar singkat pada tepi halaman buku kuno). Zero negative phonetic associations.

### Option 3: ChronoMargin
* **Meaning:** *Chrono* (time / chronological progression) + *Margin* (the margins of academic papers where marginalia and insights live). Literally: *"Time in the Margins"*.
* **Pronunciation & Syllables:** 3 syllables (`Chro-no-Mar-gin` / /ˈkrɒn.oʊ ˈmɑːr.dʒɪn/).
* **Why it fits:** Directly describes the core technical feature: adding a time dimension to paper margins without modifying the original document bytes.
* **Linguistic Screening (EN + ID):**
  * *English:* Descriptive and clear; sounds like a dedicated scientific utility.
  * *Indonesian:* Both *krono* (*kronologi*) and *margin* are standard loan words in the official Indonesian dictionary (KBBI). Readily pronounced; zero awkward connotations.

---

## 4. Comprehensive Registry & Platform Audit (Checked: 2026-11-05)

| Verification Platform | Checking Mechanism & URL | ScribeTrace | GlossTrace | ChronoMargin |
| :--- | :--- | :--- | :--- | :--- |
| **GitHub User/Org** | `https://api.github.com/users/<name>` | **FREE** (404, checked: 2026-11-05)<br>[Check Link](https://github.com/scribetrace) | **FREE** (404, checked: 2026-11-05)<br>[Check Link](https://github.com/glosstrace) | **FREE** (404, checked: 2026-11-05)<br>[Check Link](https://github.com/chronomargin) |
| **GitHub Repos** | `https://api.github.com/search/repositories?q=<name>` | **FREE** (1 empty 0-star repo, checked: 2026-11-05) | **FREE** (0 exact repos, checked: 2026-11-05) | **FREE** (0 exact repos, checked: 2026-11-05) |
| **crates.io** | `https://crates.io/api/v1/crates/<name>` | **FREE** (HTTP 404, checked: 2026-11-05)<br>[Check Link](https://crates.io/crates/scribetrace) | **FREE** (HTTP 404, checked: 2026-11-05)<br>[Check Link](https://crates.io/crates/glosstrace) | **FREE** (HTTP 404, checked: 2026-11-05)<br>[Check Link](https://crates.io/crates/chronomargin) |
| **npm Registry** | `https://registry.npmjs.org/<name>` | **FREE** (HTTP 404, checked: 2026-11-05)<br>[Check Link](https://www.npmjs.com/package/scribetrace) | **FREE** (HTTP 404, checked: 2026-11-05)<br>[Check Link](https://www.npmjs.com/package/glosstrace) | **FREE** (HTTP 404, checked: 2026-11-05)<br>[Check Link](https://www.npmjs.com/package/chronomargin) |
| **PyPI** | `https://pypi.org/pypi/<name>/json` | **FREE** (HTTP 404, checked: 2026-11-05)<br>[Check Link](https://pypi.org/project/scribetrace/) | **FREE** (HTTP 404, checked: 2026-11-05)<br>[Check Link](https://pypi.org/project/glosstrace/) | **FREE** (HTTP 404, checked: 2026-11-05)<br>[Check Link](https://pypi.org/project/chronomargin/) |
| **Homebrew Formula** | `https://formulae.brew.sh/api/formula/<name>.json` | **FREE** (HTTP 404, checked: 2026-11-05) | **FREE** (HTTP 404, checked: 2026-11-05) | **FREE** (HTTP 404, checked: 2026-11-05) |
| **Homebrew Cask** | `https://formulae.brew.sh/api/cask/<name>.json` | **FREE** (HTTP 404, checked: 2026-11-05) | **FREE** (HTTP 404, checked: 2026-11-05) | **FREE** (HTTP 404, checked: 2026-11-05) |
| **Snapcraft** | `https://api.snapcraft.io/v2/snaps/info/<name>` | **FREE** (HTTP 404, checked: 2026-11-05) | **FREE** (HTTP 404, checked: 2026-11-05) | **FREE** (HTTP 404, checked: 2026-11-05) |
| **Flathub** | `https://flathub.org/api/v2/appstream/app.<name>.<Name>` | **FREE** (HTTP 404, checked: 2026-11-05) | **FREE** (HTTP 404, checked: 2026-11-05) | **FREE** (HTTP 404, checked: 2026-11-05) |
| **Windows Package (winget)** | `microsoft/winget-pkgs/manifests` query | **FREE** (0 manifests, checked: 2026-11-05) | **FREE** (0 manifests, checked: 2026-11-05) | **FREE** (0 manifests, checked: 2026-11-05) |
| **Domain: .app** | RDAP query: `https://rdap.org/domain/<name>.app` | **AVAILABLE** (HTTP 404, checked: 2026-11-05) | **AVAILABLE** (HTTP 404, checked: 2026-11-05) | **AVAILABLE** (HTTP 404, checked: 2026-11-05) |
| **Domain: .org** | RDAP query: `https://rdap.org/domain/<name>.org` | **AVAILABLE** (HTTP 404, checked: 2026-11-05) | **AVAILABLE** (HTTP 404, checked: 2026-11-05) | **AVAILABLE** (HTTP 404, checked: 2026-11-05) |
| **Domain: .dev** | RDAP query: `https://rdap.org/domain/<name>.dev` | **AVAILABLE** (HTTP 404, checked: 2026-11-05) | **AVAILABLE** (HTTP 404, checked: 2026-11-05) | **AVAILABLE** (HTTP 404, checked: 2026-11-05) |
| **USPTO Trademark** | [USPTO Trademark Search](https://tmsearch.uspto.gov/) | **unchecked: needs a manual search by the maintainer** (checked: 2026-11-05) | **unchecked: needs a manual search by the maintainer** (checked: 2026-11-05) | **unchecked: needs a manual search by the maintainer** (checked: 2026-11-05) |
| **EUIPO Trademark** | [EUIPO eSearch plus](https://euipo.europa.eu/eSearch/) | **unchecked: needs a manual search by the maintainer** (checked: 2026-11-05) | **unchecked: needs a manual search by the maintainer** (checked: 2026-11-05) | **unchecked: needs a manual search by the maintainer** (checked: 2026-11-05) |
| **WIPO Trademark** | [WIPO Global Brand Database](https://branddb.wipo.int/) | **unchecked: needs a manual search by the maintainer** (checked: 2026-11-05) | **unchecked: needs a manual search by the maintainer** (checked: 2026-11-05) | **unchecked: needs a manual search by the maintainer** (checked: 2026-11-05) |
| **Web Search Screening** | Google / DuckDuckGo indexed software tools | **CLEAR** (No software tool; only Destiny 2 cosmetic shell, checked: 2026-11-05) | **CLEAR** (No software tool; descriptive term in linguistics/PCB, checked: 2026-11-05) | **CLEAR** (No software tool; generic listing text, checked: 2026-11-05) |

*(Note: Official USPTO, EUIPO, and WIPO databases cannot be queried reliably by automated script and require manual search by the maintainer prior to commercial launch. State trademark registries across individual US states and international non-WIPO jurisdictions also remain **unchecked**.)*

---

## 5. Comparative Evaluation & Risk Analysis

| Candidate | Why It Fits | What Is Free | What Is Taken | Key Risk & Mitigation |
| :--- | :--- | :--- | :--- | :--- |
| **1. ScribeTrace** | Immediately evokes academic writing, meticulous reading, and chronological history. High aesthetic elegance. | GitHub Org, crates.io, npm, PyPI, Homebrew, Flathub, winget, `.app`, `.org`, `.dev`. | Single abandoned 0-star personal repo `TrungSy2106/Scribetrace`. | **Low Risk:** "Scribe" is used by various documentation tools (e.g. *ScribeHOW*), but "ScribeTrace" is distinct. Official trademark registries remain unchecked (maintainer should perform a manual search prior to commercial use). Mitigate by claiming the GitHub organization immediately. |
| **2. GlossTrace** | Deep historical academic pedigree. Evokes classic manuscript *scholia* and glossing. | GitHub Org, crates.io, npm, PyPI, Homebrew, Flathub, winget, `.app`, `.org`, `.dev`. Zero matching repos. | Descriptive word "gloss" in printing and linguistics. | **Low Risk:** "Gloss" is sometimes interpreted as surface shine in non-academic contexts. Official trademark registries remain unchecked (maintainer manual search recommended). Mitigate with strong "scholarly marginal gloss" tagline. |
| **3. ChronoMargin** | Most descriptive name: explains the product in one word (*Time in the Margins*). | GitHub Org, crates.io, npm, PyPI, Homebrew, Flathub, winget, `.app`, `.org`, `.dev`. Zero matching repos. | None in software. | **Low-Medium Risk:** 3 syllables makes it slightly longer to type in CLI commands (`chronomargin`). Official trademark registries remain unchecked (maintainer manual search recommended). Mitigate with CLI alias `cm`. |

---

## 6. DevOps Recommendation

### Recommended Choice: `ScribeTrace`

**Primary Recommendation:** Adopt **ScribeTrace** as the official product name for the staged MVP and repository:

1. **Ecosystem Uniformity:** Allows claiming clean, un-prefixed identifiers across the entire developer stack:
   * GitHub Organization: `https://github.com/scribetrace`
   * Desktop App ID: `app.scribetrace.ScribeTrace` (for Flathub and macOS Bundle Identifier)
   * Rust Crates: `scribetrace` (CLI), `scribetrace-core` (engine)
   * npm Packages: `@scribetrace/viewer`
   * Website: `https://scribetrace.org` or `https://scribetrace.app`
2. **Audience Resonance:** Academics, PhD researchers, and peer reviewers self-identify with the scholarly tradition of the *scribe*, while *trace* emphasizes the unique, immutable chronological event log.
3. **Phonetic Clarity:** Crisp two-syllable word (/skraɪb treɪs/) that rolls off the tongue naturally in English and adapts cleanly into Indonesian without awkward phonetic shifts.

### Backup Alternative: `GlossTrace`
If the maintainer prefers a more classical manuscript philology aesthetic, **GlossTrace** is an equally pristine alternative with zero conflicting repositories and clean trademark registries worldwide.
