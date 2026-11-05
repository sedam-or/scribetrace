# ScribeTrace Trademark Policy & Brand Guidelines

**Effective Date:** 2026-11-06  
**Document Status:** Official Project Policy  
**Repository Path:** `TRADEMARK.md`  

> **Notice:** This document is for informational and governance purposes only and does not constitute formal legal advice.

---

## 1. ScribeTrace Trademarks & Usage Policy

The "ScribeTrace" name, logo, wordmarks, and associated visual design elements represent the ScribeTrace open-source software project. The goal of this policy is to encourage widespread adoption and community contribution while preventing confusion regarding the origin, endorsement, and authenticity of the software.

### 1.1 Permitted Uses
You may use the ScribeTrace name and unaltered logo without prior written permission for the following non-commercial, descriptive purposes:
* **Truthful Commentary & Education:** Writing articles, blog posts, academic papers, tutorials, or reviews about ScribeTrace.
* **Redistribution of Unmodified Binaries:** Distributing official, byte-identical release binaries compiled and published by the ScribeTrace project (`https://github.com/sedam-or/scribetrace`, checked: 2026-11-06).
* **Compatibility Statements:** Stating truthfully that your software, format adapter, or plugin interoperates with ScribeTrace (e.g., *"Plugin for ScribeTrace"*).

### 1.2 Restricted & Prohibited Uses
To avoid misleading users:
* **No Implied Endorsement:** You may not use the ScribeTrace name or logo in any manner that implies official partnership, sponsorship, or certification by the project maintainers.
* **Fork Rebranding:** If you distribute a modified version of ScribeTrace or a downstream fork with substantial alterations, you must remove the official logo and clearly distinguish your project name (e.g., *"Forked from ScribeTrace"* is acceptable; *"ScribeTrace Pro"* or *"ScribeTrace Official Edition"* is prohibited).
* **Domain Names & Social Media:** You may not register domain names, social media handles, or trademark filings incorporating "ScribeTrace" as the primary brand identifier.

---

## 2. Third-Party Trademarks: Zotero and Obsidian

ScribeTrace is an independent companion tool designed to interoperate smoothly with existing research workflows. We strictly respect the intellectual property and trademark guidelines of third-party projects.

### 2.1 Legal Analysis: Nominative Fair Use and Compatibility Statements
A key question for project governance is whether descriptive use—such as *"a companion to Zotero"*, *"works with Zotero"*, or *"exports to Obsidian"*—is legally acceptable:

* **Legal Doctrine (Nominative Fair Use):** Under established trademark law principles (including the U.S. *Nominative Fair Use* doctrine and European Union Trade Mark Regulation (EU) 2017/1001 Article 14(1)(c) regarding referential use to indicate intended purpose or compatibility), descriptive use of another entity's trademark is lawful provided:
  1. The product cannot be practically identified without referencing the trademark (one cannot describe importing from Zotero's local sqlite/API or exporting into Obsidian vault directories without naming them).
  2. Only so much of the mark is used as is reasonably necessary (plain text only; zero use of logos, stylization, or trade dress).
  3. The use does not suggest sponsorship, endorsement, affiliation, or that the product originates from the trademark owner.
* **Analysis of Published Policies:**
  * **Zotero:** The official [Zotero Trademark Policy](https://www.zotero.org/support/terms/trademark) (Corporation for Digital Scholarship; opened: 2026-11-06) states that the mark may not be used *"in connection with any product or service that is not affiliated with the Corporation"* and not *"in any manner that is likely to cause confusion among users"*. Crucially, Zotero's page says nothing explicitly about compatibility statements either way.
  * **Obsidian:** The official [Obsidian Brand Guidelines](https://obsidian.md/brand) and [Developer Policies](https://docs.obsidian.md/Developer+policies) (Dynalist Inc.; opened: 2026-11-06) state that the name, app icon, and logo are trademarks, prohibit logo editing/recoloring, and require reaching out prior to commercial use.
* **Conclusion & Alignment with [docs/BRAND.md §5](docs/BRAND.md):**  
  Because Zotero's published text does not explicitly delineate safe-harbor compatibility wording, relying purely on general nominative fair use carries operational ambiguity. ScribeTrace therefore strictly adopts the **cautious form** established in [docs/BRAND.md §5](docs/BRAND.md):
  1. **Strict Plain-Text Phrasing:** Use third-party names only to describe compatibility in plain text: *"a companion to Zotero"*, *"works with Zotero"*, *"exports to Obsidian"*.
  2. **Prohibited Compound Branding:** Never use *"Zotero ScribeTrace"*, *"ObsidianTrace"*, or include their names in domains, repositories, or package IDs.
  3. **Zero Asset Borrowing:** Never use their logos, app icons, typography, or trademarked brand colors.
  4. **Prohibited Claims:** Never claim or imply that ScribeTrace is *"official"*, *"certified"*, a *"partner"*, a *"plugin for Zotero/Obsidian"* (we are an independent companion, not an in-app plugin), or *"recommended by"* either project.
  5. **Pre-Launch Inquiries:** We recommend contacting both project teams (or consulting trademark counsel) before formal v1.0 public launch.

### 2.2 Mandatory Disclaimer on First Mention
In alignment with [docs/BRAND.md §5](docs/BRAND.md), the following notice must appear on the first mention of Zotero or Obsidian on any documentation page, website, README, or public announcement:
> *ScribeTrace is an independent project, not affiliated with or endorsed by Zotero or Obsidian.*
> *(Zotero is a registered trademark of the Corporation for Digital Scholarship. Obsidian is a trademark of Dynalist Inc.)*

---

## 3. Preliminary Trademark Screening: "ScribeTrace"

An automated screening attempt for the name **"ScribeTrace"** was conducted on **2026-11-06** for Nice Class 9 (computer software) and Class 42 (software services):

### 3.1 Registry Screening Results

| Registry / Database | Official Portal | Automated Screening Status | Action Required |
| :--- | :--- | :--- | :--- |
| **WIPO Global Brand Database** | [branddb.wipo.int](https://branddb.wipo.int) *(Checked: 2026-11-06)* | **NOT CHECKED:** Portal blocked automated access (CloudFront edge protection and client-side single-page application execution). | Owner must search manually for "ScribeTrace" in Classes 9 and 42 prior to launch. |
| **EUIPO TMview** | [tmdn.org/tmview](https://www.tmdn.org/tmview) *(Checked: 2026-11-06)* | **NOT CHECKED:** Portal blocked automated access (BigIP gateway and dynamic session tokens prevent unauthenticated automated queries). | Owner must search manually for "ScribeTrace" in Classes 9 and 42 prior to launch. |
| **USPTO Trademark Search** | [tmsearch.uspto.gov](https://tmsearch.uspto.gov) *(Checked: 2026-11-06)* | **NOT CHECKED:** Portal blocked automated access (AWS WAF protection and dynamic cloud search sessions). | Owner must search manually for "ScribeTrace" in Classes 9 and 42 prior to launch. |

### 3.2 Action Required for Project Owner
Because automated agents cannot bypass interactive challenges, bot protections, or client-side application boundaries (and are strictly forbidden by agent operating rules from doing so), **no automated clearance verdict is asserted**.

In alignment with [docs/BRAND.md §5](docs/BRAND.md), the ScribeTrace name is **not officially cleared**. The project maintainer/owner must perform a manual trademark clearance search in these three databases (or engage qualified trademark counsel) before any commercial distribution, public v1.0 release, or trademark filing. Do not use trademark symbols ("™", "®") until this manual clearance is complete.
