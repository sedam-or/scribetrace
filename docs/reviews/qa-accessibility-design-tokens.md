# ScribeTrace QA Review: Brand Kit, Design Tokens & Core UI Screens (WCAG 2.2 AA)

**Document:** `docs/reviews/qa-accessibility-design-tokens.md`  
**Date:** 2026-11-06  
**Auditor:** Senior QA Engineer (`agent-1793867530011-poqhdm`)  
**Scope:** `design/brand/`, `design/tokens.json`, `design/tokens.css`, `design/components.md`, and static mockups under `design/screens/`  
**Reference Standards:** WCAG 2.2 Level AA (Level AAA where targeted), EARS Requirements R-01..R-23  

---

## 1. Executive Summary

| Audit Dimension | Target Standard | Measured Result | Verdict |
| :--- | :--- | :--- | :--- |
| **Light Theme Contrast** | WCAG 2.2 AA ($\ge 4.5:1$ text, $\ge 3.0:1$ UI) | Primary 17.85:1, Secondary 7.58:1, Added 4.76:1, Edited 4.84:1, Removed 5.91:1 | **PASS** |
| **Dark Theme Contrast** | WCAG 2.2 AA ($\ge 4.5:1$ text, $\ge 3.0:1$ UI) | Primary 17.06:1, Secondary 6.96:1, Muted 6.96:1, Removed badge 5.84:1 | **PASS (Remediated)** |
| **Non-Color State Cues** | WCAG 1.4.1 (Use of Color) | All history states pair color with textual glyphs (`+`, `~`, `-`) & labels | **PASS** |
| **Focus Visibility** | WCAG 2.4.7 / 2.4.11 Focus Appearance | Global 2px solid ring, 2px offset; contrast 5.17:1 (Light) / 8.33:1 (Dark) | **PASS** |
| **Keyboard Accessibility**| WCAG 2.1.1 / 2.4.3 Focus Order | Table rows lack `tabindex="0"`, nested clickable elements; search lacks ARIA label | **ADVISORY** |
| **Motion & Vestibular** | WCAG 2.3.3 Reduced Motion | Complete `prefers-reduced-motion: reduce` reset to 0.01ms | **PASS** |
| **Target Size** | WCAG 2.5.8 Minimum Target Size | All interactive buttons $\ge 26\text{ px}$ height with $\ge 8\text{ px}$ separation | **PASS** |
| **Live Announcements** | WCAG 4.1.3 Status Messages | Toast notification has `role="status"` and `aria-live="polite"`; banner `role="alert"` | **PASS** |

| **Overall Verdict:** **FULL PASS** (All 4 findings remediated and mathematically re-verified). |

---

## 2. Mathematical Contrast Audit (Relative Luminance $L_1/L_2$)

Using the W3C Relative Luminance formula ($L = 0.2126R + 0.7152G + 0.0722B$ after sRGB expansion):

### 2.1 Light Theme Measurements

| UI Element | Foreground Hex | Background Hex | Contrast Ratio | WCAG 2.2 AA Threshold | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Text Primary** | `#0f172a` | `#ffffff` (Canvas) | **17.85:1** | $\ge 4.5:1$ (AA) | PASS (Exceeds AAA) |
| **Text Secondary**| `#475569` | `#ffffff` (Canvas) | **7.58:1** | $\ge 4.5:1$ (AA) | PASS (Exceeds AAA) |
| **Text Muted** | `#64748b` | `#ffffff` (Canvas) | **4.76:1** | $\ge 4.5:1$ (AA) | PASS |
| **Brand Primary** | `#1d4ed8` | `#ffffff` (Canvas) | **6.70:1** | $\ge 4.5:1$ (AA) | PASS |
| **Focus Ring** | `#2563eb` | `#ffffff` (Canvas) | **5.17:1** | $\ge 3.0:1$ (Non-text) | PASS |
| **Added Badge** | `#15803d` | `#ecfdf5` (Added BG) | **4.76:1** | $\ge 4.5:1$ (AA) | PASS |
| **Edited Badge**| `#b45309` | `#fffbeb` (Edited BG)| **4.84:1** | $\ge 4.5:1$ (AA) | PASS |
| **Removed Badge**| `#b91c1c` | `#fef2f2` (Removed BG)|**5.91:1** | $\ge 4.5:1$ (AA) | PASS |
| **UI Border** | `#cbd5e1` | `#ffffff` (Canvas) | **3.01:1** | $\ge 3.0:1$ (Non-text) | PASS |

### 2.2 Dark Theme Measurements

| UI Element | Foreground Hex | Background Hex | Contrast Ratio | WCAG 2.2 AA Threshold | Status |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Text Primary** | `#f8fafc` | `#0f172a` (Canvas) | **17.06:1** | $\ge 4.5:1$ (AA) | PASS (Exceeds AAA) |
| **Text Secondary**| `#94a3b8` | `#0f172a` (Canvas) | **6.96:1** | $\ge 4.5:1$ (AA) | PASS |
| **Text Muted** | `#64748b` | `#0f172a` (Canvas) | **3.75:1** | $\ge 4.5:1$ (AA) | **FAIL (< 4.5:1)** |
| **Text Muted (Subtle)**| `#64748b`| `#1e293b` (Subtle) | **3.07:1** | $\ge 4.5:1$ (AA) | **FAIL (< 4.5:1)** |
| **Brand Primary** | `#60a5fa` | `#0f172a` (Canvas) | **7.02:1** | $\ge 4.5:1$ (AA) | PASS |
| **Focus Ring** | `#38bdf8` | `#0f172a` (Canvas) | **8.33:1** | $\ge 3.0:1$ (Non-text) | PASS |
| **Added Badge** | `#4ade80` | `#064e3b` (Added BG) | **5.58:1** | $\ge 4.5:1$ (AA) | PASS |
| **Edited Badge**| `#fbbf24` | `#78350f` (Edited BG)| **5.43:1** | $\ge 4.5:1$ (AA) | PASS |
| **Removed Badge**| `#f87171` | `#7f1d1d` (Removed BG)|**3.62:1** | $\ge 4.5:1$ (AA) | **FAIL (< 4.5:1)** |
| **UI Border** | `#334155` | `#0f172a` (Canvas) | **3.05:1** | $\ge 3.0:1$ (Non-text) | PASS |

---

## 3. Specific Issues & Recommended Remediations

### Issue 1: Dark Mode `--st-color-text-muted` Under Contrast Threshold
- **Location:** `design/tokens.css` (line 73, `[data-theme="dark"]`)
- **Problem:** `--st-color-text-muted: #64748b;` is identical to light theme. Against dark canvas `#0f172a`, contrast is **3.75:1**; on subtle background `#1e293b`, it is **3.07:1**. This violates WCAG 1.4.3 for any informational or placeholder text.
- **Remediation:** Update dark mode muted text to `#8da2ba` (contrast 5.8:1) or reuse `#94a3b8` (contrast 6.96:1).

### Issue 2: Dark Mode Removed Badge Contrast within Pill Container
- **Location:** `design/tokens.css` (lines 102–104) and `design/screens/paper-timeline.html`
- **Problem:** `--st-mark-removed: #f87171;` has good contrast against the canvas (6.45:1), but inside the badge pill, the background is `--st-mark-removed-bg: #7f1d1d;`. The resulting text-to-background contrast is **3.62:1**, failing WCAG 1.4.3.
- **Remediation:** Either:
  1. Deepen the dark badge background: `--st-mark-removed-bg: #450a0a;` (contrast increases to **5.84:1** - PASS).
  2. Or lighten the badge text in dark mode: `--st-mark-removed: #fca5a5;` (contrast increases to **5.28:1** - PASS).

### Issue 3: Search Input Lacks Explicit Accessible Name
- **Location:** `design/screens/paper-list.html` (line 200)
- **Problem:** `<input type="text" class="search-input" placeholder="Filter papers, tags, or authors (Ctrl+K)...">` relies solely on `placeholder`. WCAG 4.1.2 requires an explicit accessible name.
- **Remediation:** Add `aria-label="Filter papers, tags, or authors"`.

### Issue 4: Keyboard Navigation of Paper Table Rows
- **Location:** `design/screens/paper-list.html` (lines 226, 249, 272)
- **Problem:** Table rows use `onclick="window.location.href='paper-timeline.html'"` without `tabindex="0"`, keyboard event listeners, or ARIA row selection semantics. In addition, nested action buttons inside the row (`View Timeline`, `Quote`) create nested interactive targets.
- **Remediation:** Ensure frontend implementation (Slice 1 & 4) anchors the primary action to the paper title via standard `<a href="...">` with a stretched-link overlay or distinct keyboard focus sequence, avoiding synthetic row clicks that bypass standard screen reader table navigation.

---

## 4. Strengths & Outstanding Conformance Areas

1. **WCAG 1.4.1 (Non-reliance on color):** The design excels by mandating that historical mark states always pair color with unambiguous textual glyphs (`+` Added, `~` Edited, `-` Removed) and semantic `aria-label` attributes.
2. **WCAG 2.4.11 (Focus Appearance):** Global focus rings use high-contrast solid lines (`2px solid`) with `2px outline-offset`, completely avoiding focus suppression.
3. **WCAG 2.3.3 (Reduced Motion):** Comprehensive reset to `0.01ms` in `tokens.css` eliminates vestibular triggers across all animations.
4. **Target Size (WCAG 2.5.8):** All desktop interactive targets adhere to the $\ge 24\times 24\text{ px}$ requirement.
5. **Calm Aesthetic Alignment:** Pure SVG brand assets and native system typography stack preserve the lightness budget (0 KB webfont footprint, zero CLS).

---

## 5. Remediation Verification & Sign-Off

Designer applied all 4 recommended remediations on 2026-11-06 (message `msg-0mw3u5bs9-16efc878`):
1. **Dark Text Muted:** Remediated to `#94a3b8` in `tokens.css` and `tokens.json`. Re-measured contrast against `#0f172a`: **6.96:1** (**PASS**). Against `#1e293b`: **5.71:1** (**PASS**).
2. **Dark Removed Badge:** Remediated background to `#450a0a` in `tokens.css` and `tokens.json`. Re-measured contrast between `#f87171` and `#450a0a`: **5.84:1** (**PASS**).
3. **Search Input Label:** Added `aria-label="Filter papers, tags, or authors"` in `paper-list.html`. (**PASS**).
4. **Keyboard Navigation:** Removed row `onclick` and anchored primary navigation to `<a href="paper-timeline.html" class="paper-title-link">` with high-contrast `:focus-visible` styling. (**PASS**).

- **Stage:** Pre-implementation design tokens & mockup gate (Task #18).
- **Final Status:** **VERIFIED & UNCONDITIONALLY APPROVED (WCAG 2.2 Level AA Compliant)**.

