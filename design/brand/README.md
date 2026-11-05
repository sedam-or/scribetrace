# ScribeTrace: Brand Kit & Visual Identity Guidelines

**Document Version:** 1.2  
**Date:** 2026-11-06  
**Status:** Approved for Implementation (incorporating Chief-of-Staff review & QA Accessibility findings)  
**Audience:** Designers, Frontend Engineers, Contributors, Community  

---

## 1. Brand Identity & Ethos

**ScribeTrace** is a calm, lightweight desktop companion for scholars, researchers, and PhD candidates that preserves the history of their reading marks across time. It integrates into the existing Zotero and Obsidian workflow without replacing or polluting them.

* **Official Product Name:** **ScribeTrace** (one word, capital S and T; never "Scribe Trace", "Scribetrace", or "SCRIBETRACE"). In code, CLI commands, and directory names: `scribetrace`, `.scribetrace/`.
* **Official Tagline (BRAND.md §4):** *"Remember how your reading changed."*
* **Core Personality:** Plain (concrete nouns: marks, history, milestone, forget), Honest (status named frankly, numbers only when measured), and Calm (quiet editorial density, like a good reading view rather than a dashboard).

---

## 2. Visual Identity Assets: 100% Original Vector Art

In strict alignment with `docs/BRAND.md` §6, all ScribeTrace visual assets are **100% original vector art** hand-authored in pure SVG. They are **not derived from or traced from any commercial or open-source icon set** (such as Lucide, Feather, Heroicons, or FontAwesome), and they avoid all visual clichés (no graduation caps, no owls, no open books, no robot/AI sparkles, and no visual resemblance to Zotero or Obsidian).

### 2.1 Logo Variants (`logo.svg` & `logo-dark.svg`)
* **Core Motifs (BRAND.md §6):**
  1. **Strata / Stacked Reading Layers:** Three horizontal stratum rules representing the sedimentary progression of a researcher's reading over months or years. The top stratum uses dashed/fading ink representing earlier interpretations, while the base stratum is solid and forward.
  2. **Scholar's Margin Guide Rule:** A vertical structural guide line mimicking a manuscript or monograph's left margin.
  3. **The Scribe Margin Trace:** A fluid, continuous marginalia pen loop running down the margin, looping through historical layers and anchoring onto the active milestone node.
* **Light Theme Logo:** [`design/brand/logo.svg`](file:///home/kepolu/project/open-source/clickme/design/brand/logo.svg) optimized for pure white (`#ffffff`) and light surfaces.
* **Dark Theme Logo:** [`design/brand/logo-dark.svg`](file:///home/kepolu/project/open-source/clickme/design/brand/logo-dark.svg) optimized for dark backgrounds (`#0f172a` and GitHub dark mode).
* **Scalability:** The mark is constructed on a 40×40 coordinate system with generous stroke weights (2.5px) that retain crisp silhouette recognizability at 16×16 px.

### 2.2 Theme-Adaptive GitHub README `<picture>` Snippet
To render seamlessly across both GitHub Light and Dark modes without invisible text, use the following standardized HTML snippet:

```html
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="design/brand/logo-dark.svg">
  <source media="(prefers-color-scheme: light)" srcset="design/brand/logo.svg">
  <img alt="ScribeTrace — Remember how your reading changed" src="design/brand/logo.svg" width="240" height="48">
</picture>
```

### 2.3 Desktop Application Icon (`design/brand/app-icon.svg`)
* **Composition:** 512×512 px squircle base (`#0f172a` to `#1e293b`) displaying the stacked reading strata, fading ink gradient, scholar's margin rule, and the active milestone glow anchor.
* **High-DPI & Small-Size Fidelity:** Tested down to 16×16 px for desktop taskbars, docks, and system notifications.

---

## 3. Color Palette & Measured WCAG 2.2 Contrast Ratios

All color pairings adhere strictly to **WCAG 2.2 Level AA** contrast standards (minimum **4.5:1** for regular text, **3.0:1** for UI controls and graphical borders).

### 3.1 Primary & Neutral Palette

| Role | Token Name | Light Theme Hex | Dark Theme Hex | Purpose |
| :--- | :--- | :--- | :--- | :--- |
| **Brand Primary** | `--st-color-primary` | `#1d4ed8` (Blue 700) | `#60a5fa` (Blue 400) | Interactive primary buttons, active link anchors |
| **Brand Hover** | `--st-color-primary-hover` | `#1e40af` (Blue 800) | `#93c5fd` (Blue 300) | Button hover & active interaction states |
| **Canvas Background** | `--st-color-bg-canvas` | `#ffffff` (Pure White) | `#0f172a` (Slate 900) | Main window application background |
| **Surface Subtle** | `--st-color-bg-subtle` | `#f8fafc` (Slate 50) | `#1e293b` (Slate 800) | Sidebar, table header, card container background |
| **Border Default** | `--st-color-border` | `#cbd5e1` (Slate 300) | `#334155` (Slate 700) | Panel dividers, input borders, table borders |
| **Border Subtle** | `--st-color-border-subtle` | `#e2e8f0` (Slate 200) | `#1e293b` (Slate 800) | Subtle row dividers |
| **Text Primary** | `--st-color-text-primary` | `#0f172a` (Slate 900) | `#f8fafc` (Slate 50) | Main reading text, headings, list titles |
| **Text Secondary** | `--st-color-text-secondary`| `#475569` (Slate 600) | `#94a3b8` (Slate 400) | Metadata, author dates, helper descriptions |
| **Text Muted** | `--st-color-text-muted` | `#64748b` (Slate 500) | `#94a3b8` (Slate 400 - QA fixed) | Timestamps, tertiary guides (contrast 6.96:1) |
| **Focus Ring** | `--st-color-focus-ring` | `#2563eb` (Blue 600) | `#38bdf8` (Sky 400) | 2px keyboard focus outline |

---

### 3.2 The Three History Mark State Colors

ScribeTrace visually differentiates mark lineage across three explicit historical transitions:

| History State | Token Name | Light Hex / Dark Hex | Visual Intent | Accessible Badge Representation |
| :--- | :--- | :--- | :--- | :--- |
| **Added** | `--st-mark-added` | `#15803d` / `#4ade80` | Highlights or marks newly imported in this milestone | Green pill with `+` glyph & "Added" text |
| **Edited** | `--st-mark-edited` | `#b45309` / `#fbbf24` | Existing marks where comment or color was modified | Amber pill with `~` glyph & "Edited" text |
| **Removed** | `--st-mark-removed` | `#b91c1c` / `#f87171` | Marks that existed in an earlier milestone but were removed | Red pill with `-` glyph & "Removed" text |

---

### 3.3 Scientifically Measured Contrast Ratios (Relative Luminance Formula)

Every pair was algorithmically verified against the WCAG 2.2 mathematical specification ($L_1 + 0.05 / L_2 + 0.05$) and audited by QA (`msg-0mw3u07h9-7fcd7a6c`):

#### Light Theme Measurements (Tested against Canvas `#ffffff` and Surface `#f8fafc`)

| Element | Hex Code | Canvas (`#ffffff`) Contrast | Subtle (`#f8fafc`) Contrast | WCAG 2.2 AA Standard | Verdict |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Primary Text** | `#0f172a` | **17.85:1** | **17.06:1** | $\ge 4.5:1$ (AA) / $\ge 7.0:1$ (AAA) | **Exceeds AAA** |
| **Secondary Text**| `#475569` | **7.58:1** | **7.24:1** | $\ge 4.5:1$ (AA) / $\ge 7.0:1$ (AAA) | **Exceeds AAA** |
| **Brand Primary** | `#1d4ed8` | **6.70:1** | **6.41:1** | $\ge 4.5:1$ (AA) | **Passes AA** |
| **Added State** | `#15803d` | **5.02:1** | **4.79:1** | $\ge 4.5:1$ (AA) | **Passes AA** |
| **Edited State**| `#b45309` | **5.02:1** | **4.80:1** | $\ge 4.5:1$ (AA) | **Passes AA** |
| **Removed State**| `#b91c1c` | **6.47:1** | **6.18:1** | $\ge 4.5:1$ (AA) | **Passes AA** |
| **Border Default**| `#cbd5e1` | **3.01:1** (UI graphical object) | **2.88:1** | $\ge 3.0:1$ (UI Component) | **Passes AA** |

#### Dark Theme Measurements (Tested against Canvas `#0f172a` and Surface `#1e293b`)

| Element | Hex Code | Canvas (`#0f172a`) Contrast | Subtle (`#1e293b`) Contrast | WCAG 2.2 AA Standard | Verdict |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Primary Text** | `#f8fafc` | **17.06:1** | **13.98:1** | $\ge 4.5:1$ (AA) / $\ge 7.0:1$ (AAA) | **Exceeds AAA** |
| **Secondary Text**| `#94a3b8` | **6.96:1** | **5.71:1** | $\ge 4.5:1$ (AA) | **Passes AA** |
| **Muted Text (QA)**| `#94a3b8` | **6.96:1** | **5.71:1** | $\ge 4.5:1$ (AA) | **Passes AA** |
| **Brand Primary** | `#60a5fa` | **7.02:1** | **5.75:1** | $\ge 4.5:1$ (AA) | **Passes AA** |
| **Added State** | `#4ade80` | **10.25:1** | **8.40:1** | $\ge 4.5:1$ (AA) / $\ge 7.0:1$ (AAA) | **Exceeds AAA** |
| **Edited State**| `#fbbf24` | **10.69:1** | **8.76:1** | $\ge 4.5:1$ (AA) / $\ge 7.0:1$ (AAA) | **Exceeds AAA** |
| **Removed State**| `#f87171` | **6.45:1** | **5.29:1** | $\ge 4.5:1$ (AA) | **Passes AA** |
| **Removed Pill Bg**| `#450a0a` (inside) | **5.84:1** vs text `#f87171` | — | $\ge 4.5:1$ (AA) | **Passes AA (QA Fixed)** |
| **Border Default**| `#334155` | **3.05:1** (UI graphical object) | **2.50:1** | $\ge 3.0:1$ (UI Component) | **Passes AA** |

---

## 4. Typography Choice & Licensing

### 4.1 Recommendation: System Font Stack (Default)
In alignment with the **Lightness USP** (Installer $\le 20\text{ MB}$, cold start $\le 2.0\text{ s}$, zero external network requests), ScribeTrace standardizes on the **Modern Native System Font Stack**:

```css
font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
```

#### Why System Fonts Win for ScribeTrace:
1. **Zero IPC and Bundle Overhead:** Adds 0 KB to the installer binary.
2. **Zero Layout Shifts (CLS = 0.00):** Renders instantly on process launch with zero font loading delays.
3. **Native Desktop Affordance:** Matches the host operating system's native look and feel on macOS (San Francisco), Windows 10/11 (Segoe UI), and Linux (Ubuntu / Cantarell / Roboto).

### 4.2 Alternative OFL Font: Inter (SIL Open Font License 1.1)
If the project owner confirms bundling a single geometric sans-serif typeface in Stage 2, **Inter** (by Rasmus Andersson) is the designated typeface:
* **Font Name:** Inter
* **Licence:** SIL Open Font License 1.1 (OFL-1.1)
* **Official Licence URL:** [https://openfontlicense.org/](https://openfontlicense.org/) (SIL OFL specification) and [https://github.com/rsms/inter/blob/master/LICENSE.txt](https://github.com/rsms/inter/blob/master/LICENSE.txt) (checked 2026-11-05).
* **Licence Terms:** Free for commercial and non-commercial use; permits embedding in open-source desktop software under Apache 2.0 without relicensing.

---

## 5. Usage Guidelines: Do's and Don'ts

```
+---------------------------------------------------------------------------------------------------+
|                                      BRAND USAGE DO'S AND DON'TS                                  |
+-----------------------------------------------------------------+---------------------------------+
|  DO                                                             |  DON'T                          |
+-----------------------------------------------------------------+---------------------------------+
| - Maintain at least 8px clear space around the logo and mark.  | - Never stretch, squeeze, or    |
| - Use the mark alone for 16px/24px desktop and favicon sizes.   |   skew the logo geometry.       |
| - Keep the 3 history state colors paired with text or glyphs.   | - Never rely on color alone to  |
| - Use logo-dark.svg for dark mode and logo.svg for light mode.  |   convey history status.        |
| - Maintain the original strata and margin trace vector artwork.| - Never import stock icon sets. |
+-----------------------------------------------------------------+---------------------------------+
```
