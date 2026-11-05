# ScribeTrace Component Specifications & WCAG 2.2 AA Guidelines

**Document Version:** 1.0  
**Date:** 2026-11-06  
**Status:** Approved for Implementation  
**Audience:** Frontend Developers, QA Engineers, Accessibility Reviewers  

---

## 1. Design Tokens Scale Reference

The ScribeTrace component library is built exclusively on the CSS custom properties defined in `design/tokens.css`.

### 1.1 Spacing Scale (4px Base Grid)
Dense desktop layouts require compact vertical rhythm:
* `--st-space-1`: `4px` (pill padding, icon margins)
* `--st-space-2`: `8px` (button vertical padding, compact row gaps)
* `--st-space-3`: `12px` (standard card padding, input horizontal padding)
* `--st-space-4`: `16px` (panel padding, section margins)
* `--st-space-5`: `20px` (modal margins)
* `--st-space-6`: `24px` (container gutters)
* `--st-space-8`: `32px` (empty state vertical spacing)

### 1.2 Type Hierarchy
* `--st-font-size-xs` (`11px`): Badges, pill counters, metadata timestamps.
* `--st-font-size-sm` (`12px`): Helper text, table column headers, citation keys.
* `--st-font-size-base` (`13px`): Primary paper list items, table body, standard button text.
* `--st-font-size-md` (`14px`): Panel headers, modal dialog titles.
* `--st-font-size-lg` (`16px`): Page headings, section titles.
* `--st-font-size-xl` (`18px`): First-run setup title, major milestone headers.

---

## 2. Core Component Specifications

```
+---------------------------------------------------------------------------------------------------+
|                                      SCRIBETRACE COMPONENT MATRIX                                 |
+--------------------------+--------------------------+-----------------------+---------------------+
|  1. Button               |  2. Input & Picker       |  3. Paper List Row    |  4. Timeline Item   |
|  - Primary / Secondary   |  - Folder / Text input   |  - Dense 36px row     |  - Vertical node    |
|  - Ghost / Destructive   |  - Clear focus ring      |  - Arrow nav support  |  - Milestone badge  |
+--------------------------+--------------------------+-----------------------+---------------------+
|  5. Diff Badge           |  6. Empty State          |  7. Error Banner      |  8. Toast Notice    |
|  - Added (+N), Edit (~N) |  - Actionable guidance   |  - Specific settings  |  - aria-live polite |
|  - Color-blind safe      |  - 0-mark diagnostic     |  - No background poll |  - 3-second auto-out|
+--------------------------+--------------------------+-----------------------+---------------------+
```

---

### Component 1: Button (`.st-btn`)

Buttons provide clear visual affordances, compact desktop density, and unambiguous keyboard focus states.

#### Variants & Classes
1. **Primary (`.st-btn-primary`):** Main affirmative actions (e.g., "Connect to Zotero", "Import Marks", "Copy Quote").
   * Background: `--st-color-primary`
   * Text: `--st-color-text-inverse` (`#ffffff`)
   * Hover: `--st-color-primary-hover`
2. **Secondary (`.st-btn-secondary`):** Standard neutral actions (e.g., "Refresh", "Browse...", "Cancel").
   * Background: `--st-color-bg-surface`
   * Border: `1px solid var(--st-color-border)`
   * Text: `--st-color-text-primary`
   * Hover: Background `--st-color-bg-subtle`, border `--st-color-border-strong`
3. **Ghost (`.st-btn-ghost`):** Low-emphasis inline actions (e.g., "Dismiss", "Copy ID").
   * Background: `transparent`
   * Text: `--st-color-text-secondary`
   * Hover: Background `--st-color-bg-subtle`, text `--st-color-text-primary`
4. **Destructive (`.st-btn-destructive`):** High-risk actions (e.g., "Forget Paper History" R-16).
   * Background: `transparent`
   * Border: `1px solid var(--st-mark-removed)`
   * Text: `--st-mark-removed`
   * Hover: Background `--st-mark-removed-bg`

#### Specs & States
* **Height:** Small `28px` (padding `4px 10px`), Standard `32px` (padding `6px 14px`).
* **Border Radius:** `--st-radius-md` (`5px`).
* **Focus Visible:** `outline: 2px solid var(--st-color-focus-ring); outline-offset: 2px;`
* **Disabled:** `opacity: 0.5; cursor: not-allowed; pointer-events: none;` (`aria-disabled="true"`).

```html
<button type="button" class="st-btn st-btn-primary">Connect to Zotero</button>
<button type="button" class="st-btn st-btn-secondary">Refresh</button>
```

---

### Component 2: Input & Folder Picker (`.st-input`, `.st-picker`)

Used in first-run configuration and search filtering.

#### Specs & States
* **Height:** `32px`.
* **Border:** `1px solid var(--st-color-border)`.
* **Background:** `--st-color-bg-canvas`.
* **Padding:** `6px 10px`.
* **Typography:** `--st-font-size-base` (`13px`).
* **Placeholder:** `--st-color-text-muted`.
* **Focus State:** `border-color: var(--st-color-focus-ring); outline: 2px solid var(--st-color-focus-ring); outline-offset: 1px;`.
* **Folder Picker Affordance:** Read-only path input combined with a trailing "Browse..." secondary button.
* **Accessibility:** Always paired with a visible `<label for="...">` element.

```html
<div class="st-form-group">
  <label for="project-folder" class="st-label">Project Folder</label>
  <div class="st-picker-group">
    <input id="project-folder" type="text" class="st-input" readonly value="~/Documents/QuantumComputing2026" />
    <button type="button" class="st-btn st-btn-secondary">Browse...</button>
  </div>
</div>
```

---

### Component 3: Paper List Row (`.st-list-row`)

The primary desktop navigation row displaying imported papers. Calm, dense, and scannable.

#### Specs & States
* **Height:** `44px` (dense layout allowing 12–15 papers in standard 768px height viewports).
* **Grid Layout:**
  * Title & Authors (flex grow, bold title with muted year)
  * Milestone timestamp / status ("Refreshed 2h ago")
  * Total Marks Pill (`14 marks`)
  * Action Button ("View History" / "Refresh")
* **Divider:** `border-bottom: 1px solid var(--st-color-border-subtle)`.
* **Hover State:** Background `--st-color-bg-subtle`.
* **Selected / Active State:** Background `--st-color-primary-subtle`, border-left `3px solid var(--st-color-primary)`.
* **Keyboard Accessibility:** `tabindex="0"`, activates on `Enter` or `Space`, supports `ArrowDown` / `ArrowUp` list navigation.

---

### Component 4: Timeline Item (`.st-timeline-item`)

Displays chronological milestones for a paper (R-11, R-12).

#### Specs & States
* **Structure:** Vertical connecting rule (`2px solid var(--st-color-border-subtle)`) with milestone circular node (`12px` dot).
* **Content:**
  * Milestone Title (e.g. `v3 - Rebuttal pass`, or default `Imported from Zotero`)
  * Timestamp (e.g. `Nov 5, 2026, 14:32`) & Device Origin (`Me (laptop)`)
  * History Diff Badges (`+4 Added`, `~2 Edited`, `-1 Removed`)
  * "Restore to this point" or "Inspect Diff" button.

---

### Component 5: Diff Badge / Pill (`.st-diff-badge`)

Indicates what changed between milestones. Must adhere strictly to WCAG 1.4.1 (not relying on color alone).

#### Specs & Color Coding
1. **Added:**
   * Text: `--st-mark-added` (`#15803d` Light / `#4ade80` Dark)
   * Background: `--st-mark-added-bg` (`#ecfdf5` Light / `#064e3b` Dark)
   * Border: `1px solid var(--st-mark-added-border)`
   * Content: `+4 Added` (Explicit `+` glyph and screen-reader label)
2. **Edited:**
   * Text: `--st-mark-edited` (`#b45309` Light / `#fbbf24` Dark)
   * Background: `--st-mark-edited-bg` (`#fffbeb` Light / `#78350f` Dark)
   * Border: `1px solid var(--st-mark-edited-border)`
   * Content: `~2 Edited` (Explicit `~` glyph and screen-reader label)
3. **Removed:**
   * Text: `--st-mark-removed` (`#b91c1c` Light / `#f87171` Dark)
   * Background: `--st-mark-removed-bg` (`#fef2f2` Light / `#7f1d1d` Dark)
   * Border: `1px solid var(--st-mark-removed-border)`
   * Content: `-1 Removed` (Explicit `-` glyph and screen-reader label)

```html
<span class="st-diff-badge st-diff-added" aria-label="4 annotations added">
  <span aria-hidden="true">+4</span> Added
</span>
```

---

### Component 6: Empty State (`.st-empty-state`)

Shown when a folder or connection has no items yet (R-05, R-01).

#### Specs & Structure
* **Container:** Centered card with `--st-space-8` padding.
* **Icon:** Calm, monochromatic line illustration (`48×48 px`, stroke `var(--st-color-text-secondary)`).
* **Heading:** `--st-font-size-md` (`14px`), bold.
* **Body:** Max 2 lines of clear instruction, no jargon.
* **Primary Action:** Clear, self-explanatory button (e.g., "Import Marks from Zotero").
* **Diagnostic Notice:** If 0 marks were found, explains why without blaming the user: *"No annotations found in Zotero for this paper. Highlight a sentence in Zotero, then click Refresh here."*

---

### Component 7: Error Banner / Inline Notice (`.st-banner`)

Used for actionable error reporting, such as when Zotero is unreachable (R-04).

#### Specs & Invariants
* **Background:** `--st-color-error-bg`.
* **Border:** `1px solid var(--st-color-error-border)`.
* **Text:** `--st-color-error-text`.
* **Icon:** `AlertCircle` warning icon (`18×18 px`).
* **Content:**
  * Title: Plain description (e.g. `Zotero local access is not responding`).
  * Body: Specific setting to change: `Open Zotero → Settings → Advanced, and check "Allow other applications on this computer to communicate with Zotero".`
  * Action: "Check Again" button (zero background polling).

---

### Component 8: Toast Notification (`.st-toast`)

Transient feedback for non-blocking actions (e.g., "Quote copied with citation to clipboard").

#### Specs & A11y Invariants
* **Position:** Fixed bottom-right `24px` from edges.
* **Background:** `--st-color-text-primary` (High-contrast inverse popover).
* **Text:** `--st-color-text-inverse`.
* **Padding:** `8px 14px`.
* **Border Radius:** `--st-radius-md`.
* **Shadow:** `--st-shadow-modal`.
* **Accessibility:** Formatted with `role="status"` and `aria-live="polite"`. Auto-dismisses in 3000 ms. Can be dismissed with `Esc`.

---

## 3. WCAG 2.2 AA Accessibility Rules & Invariants

```
+---------------------------------------------------------------------------------------------------+
|                                 WCAG 2.2 AA ACCESSIBILITY AUDIT CHECKLIST                         |
+--------------------------+--------------------------+-----------------------+---------------------+
|  Contrast Ratios         |  Focus Indicators        |  Keyboard Navigation  |  Motion & Timing    |
|  - Text: >= 4.5:1        |  - 2px solid ring        |  - Complete Tab cycle |  - Reduced motion   |
|  - UI borders: >= 3.0:1  |  - 2px outline offset    |  - Esc dismisses      |  - No auto-retries  |
|  - Badges: >= 5.0:1      |  - Visible on all themes |  - Arrow key lists    |  - No timeout traps |
+--------------------------+--------------------------+-----------------------+---------------------+
```

1. **Contrast Standard (WCAG 1.4.3 / 1.4.11):**
   * Primary text achieves **17.85:1** (Light) and **17.06:1** (Dark).
   * Secondary text achieves **7.58:1** (Light) and **6.96:1** (Dark).
   * All three diff badge colors achieve $\ge 5.02:1$ against canvas and $\ge 4.79:1$ against surface backgrounds.
2. **Focus Visibility (WCAG 2.4.7 / 2.4.11 / 2.4.13):**
   * `:focus-visible` is globally styled with `2px solid var(--st-color-focus-ring)` and `2px outline-offset`. Focus rings are never suppressed (`outline: none` without replacement is forbidden).
3. **Keyboard Navigability (WCAG 2.1.1 / 2.1.2):**
   * Every interactive element can be reached via `Tab` and activated via `Enter` or `Space`.
   * Modals trap focus and close cleanly on `Esc`.
   * No keyboard traps anywhere in the interface.
4. **Target Size (WCAG 2.5.8 Minimum Target Size):**
   * All interactive clickable elements meet the minimum $24 \times 24\text{ px}$ target area, with at least $8\text{ px}$ spacing between adjacent touch targets.
5. **Prefers-Reduced-Motion (WCAG 2.3.3):**
   * When `prefers-reduced-motion: reduce` is detected in the operating system, all CSS transitions and animations are reduced to `0.01ms`, eliminating vestibular disorientation.
