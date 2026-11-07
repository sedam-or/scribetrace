# Architect review: frontend commits 7ac8865, e95b83f, def3915

Reviewer: architect · Date: 2026-11-07 · Scope: UI-only, mock-backed. Backend not reviewed here.

Verdict: **approved with conditions** for the mock-backed UI. The Tauri adapter for `paper_link`, `open_link`, `list_milestones` and `milestone_diff` must not land until the backend review passes.

## Commits

| Commit | Content | Verdict |
|---|---|---|
| 7ac8865 | Paper page, typed `{ target, paperId }` link request (#30) | Approved, conditions C1–C2 (see earlier mail) |
| e95b83f | Mock browser preview for the Paper page | Approved. Mock-only code, no effect on the Tauri build path |
| def3915 | Milestone timeline and diff view (R-12) | Approved, one contract condition (C3) and two notes |

## Checks against the contracts

- **Link contract (#30, DECISION-LOG row z):** the request carries only `target` and `paperId`. The displayed URI is never sent back on approval. Target mapping `doi → doi`, `zotero_pdf → pdf`, `zotero_select → item` is correct. Met.
- **Diff contract (slice 7, final envelope):** `MilestoneDiff` has required `from`, `to`, `added: Mark[]`, `removed: Mark[]`, `edited: { key, before, after, noteDiff }[]` and top-level `noteDiff: DiffLine[]`. `DiffLine.op` is `equal | add | remove`. Met.
- **Inert text (R-20, SEC-x):** milestone labels, mark text, comments and note lines are rendered as React text nodes. No `dangerouslySetInnerHTML` in the new code. Met.
- **Native confirmation (R-35, SEC-23):** the in-webview confirm in `PaperPage.tsx` is a mock stand-in only. Condition C1 still applies to the Tauri integration.
- **Offline invariant:** no new network call in the three commits. The timeline calls only the typed `list_milestones` and `milestone_diff` RPCs. Met.

## Conditions

- **C1 (native confirmation):** the Tauri commit for `open_external` must use the native dialog and show the full URI. Not met by these commits, as expected for a mock.
- **C2 (disabled-action copy):** the "why disabled" text must map from the backend's error or availability code, not from fixture nullability. Not met yet (mock keys off fixture nullability).
- **C3 (list order contract):** `openCompare` in `MilestoneTimeline.tsx` treats `milestones[index - 1]` as the earlier milestone, so it assumes `list_milestones` returns oldest first. The contract did not say so. **Fix:** the backend returns milestones in display order, ascending (`ts`, `device`, `seq`, `id`, FORMAT §4.3), and the UI test asserts it. Backend must match before its Tauri adapter lands.

## Notes (not blockers)

- `App.tsx` removed the `HistoryPanel` call and its `readHistory` load on paper open, so the raw event list is no longer shown. This matches the timeline plan, but confirm with pm that no requirement needs the raw event list in v1. R-06 needs the events stored, not shown.
- `data-load-ms` on the timeline section exposes a timing value in the DOM. Fine for the mock; remove it or keep it behind a test flag before the real build.
- The timeline group and the Compare button share the label `history.timeline.compare`. Use a distinct aria label for the group.

## Reference correction

My earlier note on #16 (issue draft for #31) said the Linux E2E gate was 160 MB and red at 190.62 MB. That is stale. The gate is **200 MB** (ADR 0014 amendment, DECISION-LOG row y; `e2e/test_walking_skeleton.mjs:161` compares against 200 after commit 9456de7). At 200 MB the measured 190.62 MB passes. The #16 fix is therefore: copy the printed line from a green run on main at the current commit, not from a run that was red.
