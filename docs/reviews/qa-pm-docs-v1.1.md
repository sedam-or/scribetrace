# QA Independent Review: PM Documentation v1.1 (ScribeTrace)

**Reviewer:** Senior QA Engineer (`agent-1793867530011-poqhdm`)  
**Target Documents:** [`docs/REQUIREMENTS.md`](../REQUIREMENTS.md), [`docs/BUSINESS.md`](../BUSINESS.md), [`README.md`](../../README.md), [`docs/DECISION-LOG.md`](../DECISION-LOG.md)  
**Baseline References:** [`docs/ARCHITECTURE.md`](../ARCHITECTURE.md) (v3.0), [`docs/SECURITY-REQUIREMENTS.md`](../SECURITY-REQUIREMENTS.md), [`docs/debate/08-links-and-3-step.md`](../debate/08-links-and-3-step.md), [`docs/adr/0006-0013`](../adr/)  
**Overall Verdict:** **PASS (with 2 minor textual fixups)**

---

## Executive Summary & Checks Matrix

| Check # | Verification Area | Verdict | Evidence & Key Finding |
|---|---|---|---|
| **Check 1** | **R-ID Testability & Traceability** | **PASS** | All 28 R-IDs (R-01..R-23, R-30..R-34) are testable. All 11 `Cap` tags map to real capabilities in `ARCHITECTURE.md §6`. All 24 referenced SEC numbers map to real items in `SECURITY-REQUIREMENTS.md` (zero dangling IDs). |
| **Check 2** | **Numbers & Lightness Targets** | **PASS** | All 3-3-3-3 journey targets and lightness figures (installer size, idle RSS, cold start, latencies) match `08-links-and-3-step.md` and `ARCHITECTURE.md` budgets verbatim and are explicitly labelled as targets. |
| **Check 3** | **Unverified Claims & README Integrity** | **PASS** | Trademark clearance is honestly declared "unchecked"; competitor claims carry `(†)` unverified flags; Zotero API limits are disclosed; `README.md` contains zero fake badges and has an "Honest limits" section. |
| **Check 4** | **DECISION-LOG Consistency (a–l)** | **PASS** | Items a–l align with `ARCHITECTURE.md` v3.0 and ADRs 0006–0013 (staged reader gate, covers-only purge, browser-based update check, link connector delegation). |
| **Check 5** | **Relative Link Resolution** | **PASS** | Automated filesystem audit confirmed 64 of 64 markdown links resolve to valid disk paths (zero broken links). |

---

## Detailed Findings & Concrete Fixes

### 1. Check 1: R-ID Testability & Traceability — PASS
* **Traceability Invariants:**
  * **Capabilities (`Cap`):** 100% of capabilities cited (`Import`, `History`, `Clones`, `Corrupt line`, `Milestones`, `Notes`, `Obsidian`, `Forget`, `Open format`, `Reader`, `Lightness`) exist in [`docs/ARCHITECTURE.md §6`](../ARCHITECTURE.md#L56-L70).
  * **Security Requirements (`SEC`):** All cited IDs (SEC-1, 2, 5, 7, 8, 9, 10, 11, 12, 13, 14, 17, 18, 19, 20, 21, 22, 23, 24, 28, 33, 34, 35, 36) match verified items in [`docs/SECURITY-REQUIREMENTS.md`](../SECURITY-REQUIREMENTS.md). Zero dangling references.
* **Concrete Fix (Minor Syntax Correction):**
  * `docs/REQUIREMENTS.md#L47`: `R-33` lacks an EARS action clause (`WHERE Stage 2 is adopted, a side-by-side visual comparison...`). *Fix:* Change to: `WHERE Stage 2 is adopted, the system SHALL provide a side-by-side visual comparison of two milestones.`
  * `docs/REQUIREMENTS.md#L48`: `R-34` is a bare noun phrase. *Fix:* Change to: `WHERE post-v1 capabilities are requested, the system SHALL support a reading-session log, W3C JSON-LD export, scribetrace:// links, and direct PDF URLs.`

### 2. Check 2: Lightness & Journey Numbers — PASS
* **3-3-3-3 Journey:** 12 steps total; 5-min first mark import; 10-min quote paste; 4 of 5 tester pass rate. Matches [`docs/debate/08-links-and-3-step.md §B`](../debate/08-links-and-3-step.md#L28-L46) verbatim.
* **Lightness Budgets:** Installer (≤20 MB / ≤35 MB), idle memory (≤120 MB / ≤250 MB), cold start (≤2 s), paper refresh (≤5 s), timeline open (≤1 s) match [`docs/ARCHITECTURE.md#lightness-budget`](../ARCHITECTURE.md#L73-L86) and are explicitly scoped as targets to be measured in the prototype.

### 3. Check 3: Claim Verification & README Checks — PASS
* **Trademark:** Clear, repetitive disclaimers that USPTO, EUIPO, and WIPO searches are unchecked ([`docs/REQUIREMENTS.md#L4`](../REQUIREMENTS.md#L4), [`docs/BUSINESS.md#L18`](../BUSINESS.md#L18), [`README.md#L31`](../../README.md#L31)).
* **Competitor Matrix:** 10 tools evaluated with explicit unverified markers `(†)` and URL check dates ([`docs/BUSINESS.md#L22`](../BUSINESS.md#L22)).
* **Zotero Integration:** Cites official docs and forum threads; accurately notes undocumented annotation endpoints.
* **Badges & Hype:** `README.md` is clean, concise, contains no vanity build/coverage badges, and explicitly warns: *"Status: pre-alpha, design phase. Nothing to install yet."*

### 4. Check 4: DECISION-LOG Consistency — PASS
* Decisions a–l correspond directly with the lean v3.0 architecture:
  * Decision e (Purge = covers-only): Matches ADR 0007.
  * Decision f (Update check opens release page): Matches ADR 0012 and ARCH v3.0 simplification log.
  * Decision h (Links via Zotero Connector): Matches ADR 0013.
* **Concrete Fix (Typo):**
  * `docs/DECISION-LOG.md#L16`: In row `a` Reason column, text states *"ScribeTrace's are taken ([NAMING](NAMING.md))"*. *Fix:* Change to *"Palimpsest's are taken ([NAMING](NAMING.md))"*.

### 5. Check 5: Relative Link Resolution — PASS
* Executed shell link verification across all 4 documents. 64 / 64 relative markdown links resolve to valid files. Zero 404s.
