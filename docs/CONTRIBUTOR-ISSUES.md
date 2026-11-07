# ScribeTrace: contributor issues (draft for GitHub)

**Status: final text, not yet posted.** These issues are posted only after [CONTRIBUTING](../CONTRIBUTING.md) §3 is verified from a clean clone and the DCO check is live. Every issue below is safe to work on without touching the format rules, the security rules, the Zotero network behaviour, the release, or the paused marketing and website work.

**Labels to create** (exact names, created before posting): `good first issue`, `help wanted`, `docs`, `validation`, `copy`, `test`, `cli`, `i18n`, `fixtures`, `spec`, `packaging`, `accessibility`.

**Labels used:** `good first issue` (small, self-contained, a new contributor can finish it in an afternoon with the repo docs) and `help wanted` (larger or needs a platform or a decision first).

**Each issue, when posted, ends with:** Sign off your commits with `git commit -s` (see [CONTRIBUTING](../CONTRIBUTING.md)).

---

## 1. Fix broken relative links in README and docs

- **Labels:** `good first issue`, `docs`
- **Area:** docs
- **Difficulty:** easy

**What and why.** Several docs link to each other with relative paths. A broken link sends a new reader to a 404 on GitHub. A quick link check and a fix keeps the reading path working.

**Where.** `README.md`, `docs/*.md`. Do not touch `docs/RELEASE-DRAFT-1.0.md` and `docs/website/` (both are read only for this issue).

**Acceptance criteria.**
1. A link checker run over `README.md` and `docs/*.md` reports no broken relative links, and the run is pasted into the PR description.
2. Each fix changes only the link target; no wording changes in the same PR.
3. Links into `docs/website/` are reported in the PR, not changed.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 2. Write a Windows manual validation report template

- **Labels:** `help wanted`, `docs`, `validation`
- **Area:** validation
- **Difficulty:** easy

**What and why.** The [user guide](USER-GUIDE.md) says Windows and macOS are untested for users. We want a repeatable way for someone with a Windows machine to report what works, without sending personal data. A template makes the report comparable across people.

**Where.** New file `docs/validation/REPORT-TEMPLATE-WINDOWS.md`; link it from the untested note in `docs/USER-GUIDE.md` §2.

**Acceptance criteria.**
1. The template asks for Windows version, Zotero 7 version, ScribeTrace build name and the steps from `docs/USER-GUIDE.md` §3 and §4, each with a pass or fail box.
2. It has no field for names, email addresses or paper titles, and says so in one line.
3. The USER-GUIDE link works and the template renders on GitHub.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 3. Run the Rust and UI tests on macOS and report the result

- **Labels:** `help wanted`, `validation`
- **Area:** validation
- **Difficulty:** medium

**What and why.** Our CI matrix is planned for Linux, Windows and macOS (see [CI-CD](CI-CD.md)). Nobody has yet run the tests on a Mac by hand. A first run tells us what breaks before the pipeline does.

**Where.** `CONTRIBUTING.md` §3 (setup), `src-core/`, `src-cli/`, `tests-ui/`.

**Acceptance criteria.**
1. Follow `CONTRIBUTING.md` §3 on macOS and record each command that fails, with its exact error text.
2. Run `cargo test` for the workspace and `pnpm` UI tests; report pass or fail counts.
3. Post the result as an issue comment; do not change code in this issue. Fixes become new issues.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 4. Add screenshots to the user guide and README

- **Labels:** `help wanted`, `docs`
- **Area:** docs
- **Difficulty:** medium

**What and why.** The user guide describes the setup screen, the paper list and the paper page in words only. Screenshots help a new user match what they see. The guide has no screenshots yet, so this issue adds them from a real run.

**Where.** New folder `docs/screenshots/`; `docs/USER-GUIDE.md` (§3 setup, §4 use); `README.md`.

**Acceptance criteria.**
1. Two PNG screenshots now: setup screen and paper list. Each under 300 KB. The paper page screenshot is a follow-up issue, once the paper page is merged (it depends on #30).
2. Each screenshot is taken on Linux from a build running against the Zotero stand-in fixtures, not a real library; no real names or titles appear.
3. The guide links each image with alt text that names the screen.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 5. Review the wording of error messages in en.json

- **Labels:** `good first issue`, `copy`
- **Area:** copy
- **Difficulty:** easy

**What and why.** Error messages are where a new user gets stuck. Plain words help more than technical ones. The strings are in one file, so a wording review is a small, reviewable change. Wording decisions are made by the maintainers; a contributor proposes, and the maintainers decide.

**Where.** `src/strings/en.json` (the `zotero.error.*` and `setup.*` keys), `tests-ui/strings.test.ts`.

**Acceptance criteria.**
1. Each proposed change keeps every `{placeholder}` exactly as it is in the original key.
2. No key is renamed or removed; `tests-ui/strings.test.ts` passes.
3. The PR body lists each changed key with the old text and the new text, and a one-line reason.
4. The change follows the voice in `docs/BRAND.md` (plain, short sentences, says what to do next).

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 6. Add a second UI locale

- **Labels:** `help wanted`, `i18n`
- **Area:** i18n
- **Difficulty:** medium

**What and why.** The UI reads every string from one English file. A second language makes ScribeTrace usable for more researchers. The loader in `src/strings.ts` currently has only one file.

**Decided.** The locale is chosen at startup from the primary language subtag of the OS locale (for example `de` from `de-DE`). Only locales bundled as static imports are used, listed in one constant in `src/strings.ts`. No runtime fetching and no user setting in v1. A missing key falls back to English. Spanish is the proposed first second locale; the maintainers can change it before work starts.

**Where.** `src/strings.ts`, `src/strings/en.json`, new file `src/strings/<locale>.json`, `tests-ui/strings.test.ts`.

**Acceptance criteria.**
1. The loader follows the decided rule above, and the new locale is one static import in the constant in `src/strings.ts`.
2. Every key in `en.json` exists in the new file, or the loader falls back to English for it; a test checks both.
3. Missing or extra placeholders in the new file fail a test, as they do for English.
4. The translation is reviewed by a native speaker, named in the PR (the name is public, not a private contact).

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 7. Add a test for the Button type attribute

- **Labels:** `good first issue`, `test`
- **Area:** UI tests
- **Difficulty:** easy

**What and why.** The shared `Button` component passes its `type` prop straight to the DOM. A test that pins the rendered attribute protects forms from accidental submits later.

**Where.** `src/components/Button.tsx`, `tests-ui/components.test.tsx`.

**Acceptance criteria.**
1. Read `src/components/Button.tsx` first and list the values the `type` prop accepts.
2. Add one test per accepted value that checks the rendered `type` attribute.
3. The existing tests in `tests-ui/components.test.tsx` still pass without edits.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 8. Check scribetrace-cli exit codes for bad calls

- **Labels:** `good first issue`, `cli`, `test`
- **Area:** scribetrace-cli
- **Difficulty:** easy

**What and why.** The CLI promises exit code 2 for a usage or read error and exit code 1 for bad lines (see [user guide](USER-GUIDE.md) §9). A test per documented exit path keeps that promise visible.

**Where.** `src-cli/src/lib.rs` (command dispatch), `src-cli/tests/cli.rs`.

**Acceptance criteria.**
1. Read `src-cli/tests/cli.rs` first. Already covered: golden checks, clean project exit 0, usage errors exit 2 (`usage_errors_exit_2_with_a_message_on_stderr`), help and version exit 0.
2. Find a documented exit path (`docs/USER-GUIDE.md` §9) that has no test yet, and add a test only for it. If the pass finds none, post "no gap found" in the PR and close without code.
3. `cargo test -p scribetrace-cli` passes, and any new test checks the exit code and that stderr is not empty.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 9. Show exit codes in the scribetrace-cli usage text

- **Labels:** `good first issue`, `cli`, `docs`
- **Area:** scribetrace-cli
- **Difficulty:** easy

**What and why.** The usage text lists the commands but not what the exit codes mean, so a user who reads `--help` cannot tell a bad line from a read error. The guide has the full list; the help text should carry the short version.

**Where.** `src-cli/src/lib.rs` (usage text near the top of the file).

**Acceptance criteria.**
1. The usage text lists exit codes 0, 1 and 2 with one short meaning each, matching `docs/USER-GUIDE.md` §9 word for word in meaning.
2. Commands and their descriptions are unchanged; `timeline` is not added (it is a later slice).
3. `cargo test -p scribetrace-cli` passes.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 10. Add a golden test for exporting a paper with no marks

- **Labels:** `good first issue`, `cli`, `test`
- **Area:** scribetrace-cli
- **Difficulty:** easy

**What and why.** The export command has a golden file for a paper with marks. An empty paper is a common real case (a paper imported with no highlights yet), and its output should be checked, not guessed.

**Where.** `src-cli/tests/cli.rs`, `src-cli/tests/golden/`, `spec/sample-project/` (read only; do not change the sample).

**Acceptance criteria.**
1. Pick a paper id in `spec/sample-project/` with no marks, or report that none exists and stop.
2. Add `src-cli/tests/golden/export_empty.md` with the exact output the command prints today.
3. The test compares the output to the golden file and fails if they differ.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 11. Add a fixture for a Zotero item with no DOI and no PDF

- **Labels:** `help wanted`, `test`, `fixtures`
- **Area:** fixtures
- **Difficulty:** medium

**What and why.** The paper page disables its DOI and PDF buttons when Zotero has no DOI or no PDF attachment. Those states need fixture data, so they can be tested without a real Zotero library.

**Where.** New folder `tests/fixtures/zotero-no-doi/`. Do not add to `tests/fixtures/zotero/`: the existing tests (`standin_replay_test.rs`, the IPC test) expect exactly 3 papers there. `test-support/zotero-standin/src/lib.rs` (read only unless a second fixture root is needed; keep it on 127.0.0.1).

**Acceptance criteria.**
1. A new fixture folder with one paper that has no DOI field and no PDF attachment, using only invented titles and authors.
2. No change to how the stand-in listens (127.0.0.1) and no change to the existing fixture folder.
3. A test checks the `list_papers` output for that paper: no DOI, no PDF. The disabled-button state is not tested here; it waits for the paper page merge (depends on #30).

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 12. Add a test vector for a line cut off in the middle of JSON

- **Labels:** `help wanted`, `test`, `spec`
- **Area:** spec test vectors
- **Difficulty:** medium

**What and why.** A file cut off by a crash or a sync conflict can leave a half-written line. The checker should count it as bad and not crash. A test vector makes that case visible to every implementation.

**Where.** `spec/vectors/bad/`, `spec/vectors/manifest.json`. The existing file `spec/vectors/bad/not_json.jsonl` covers a line that is not JSON at all; this issue adds a different case.

**Acceptance criteria.**
1. One new file `spec/vectors/bad/not_json_truncated.jsonl` holding one line of valid JSON cut before its end. It reuses the existing rule `not_json` (FORMAT §5); no new rule.
2. `manifest.json` lists the new file with the expected result (bad line, rule `not_json`), in the same form as the existing entries.
3. `scribetrace-cli check <folder>` run on a project folder that contains this file exits with code 1 and names the line. (The check command takes a project folder, not a single file.)

**Maintainer note.** This issue adds a test case for an existing rule. It does not change any rule in the format spec.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 13. Draft a Flatpak manifest for local builds

- **Labels:** `help wanted`, `packaging`
- **Area:** packaging
- **Difficulty:** medium

**What and why.** [CI-CD](CI-CD.md) plans a Flatpak build. There is no manifest yet. A manifest that builds on a developer's machine is the first step; submitting it to a store is a separate, later decision.

**Where.** New folder `packaging/flatpak/` (manifest and its readme), `docs/CI-CD.md` (Flatpak lines).

**Acceptance criteria.**
1. `flatpak-builder` builds the app from the manifest on Linux, with the sources declared in the manifest only.
2. The manifest asks only for the file access it needs (the folder the user picks), and the PR lists each permission with a reason.
3. The readme says this is a local build and does not publish anything.

**Scope.** Not phase one: AppImage is the first release (see `docs/DECISION-LOG.md`); the `.deb` builds but is not a gate. Local build files only.

**Maintainer note.** Publishing to any store is out of scope and is not part of this issue.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 14. Draft an AUR package file for local builds

- **Labels:** `help wanted`, `packaging`
- **Area:** packaging
- **Difficulty:** medium

**What and why.** Arch users install from the AUR. A `PKGBUILD` that builds from a tagged source archive lets them try ScribeTrace the same way. Submitting it to the AUR is a release action and is not part of this issue.

**Where.** New folder `packaging/aur/` (`PKGBUILD`, `.SRCINFO`), `docs/CI-CD.md` (the packaging lines).

**Acceptance criteria.**
1. `makepkg` builds the package from a local source archive on an Arch system, and the PR says which Arch version was used.
2. `.SRCINFO` matches the `PKGBUILD` (`makepkg --printsrcinfo` output).
3. Nothing is uploaded to the AUR in this issue.

**Scope.** Not phase one: AppImage is the first release (see `docs/DECISION-LOG.md`); the `.deb` builds but is not a gate. Local build files only.

**Maintainer note.** Publication to the AUR is a release step and is done by the maintainers only.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 15. Check the setup screen with the keyboard only

- **Labels:** `help wanted`, `accessibility`
- **Area:** accessibility
- **Difficulty:** medium

**What and why.** A keyboard-only user must be able to finish setup. We have not checked the three setup steps and the folder picker with the Tab key and a screen reader yet.

**Where.** `src/components/SetupView.tsx`, `src/components/FolderPicker.tsx`, `src/components/Button.tsx`.

**Acceptance criteria.**
1. A written check lists each control on the setup screen, the key that reaches it, and whether its focus outline is visible.
2. Each control has an accessible name; any that does not is listed with the file and line.
3. Fixes for missing names are a separate PR from the report; the report is posted as an issue comment first.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 16. Make the walking-skeleton e2e run reproducible from its README

- **Labels:** `good first issue`, `test`, `docs`
- **Area:** e2e
- **Difficulty:** easy

**What and why.** The end-to-end harness runs on Linux, but its readme does not yet say the order of steps or what a passing run prints. A new contributor should be able to reproduce a pass from the readme alone.

**Where.** `e2e/README.md`, `e2e/run-linux.sh`.

**Acceptance criteria.**
1. The readme lists the prerequisites that `e2e/run-linux.sh` actually checks, and nothing else.
2. The readme gives the one command to run, and shows the printed memory line from a green run on `main` at the current commit as the expected output.
3. Following only the readme on a clean Linux checkout reproduces a pass; the PR says which distribution was used.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 17. Check the troubleshooting table against the real messages

- **Labels:** `good first issue`, `docs`
- **Area:** docs
- **Difficulty:** easy

**What and why.** The troubleshooting section tells a user what they will see and what to do. If a row quotes a message that the app does not show, the user searches in vain. Each row should match a string that exists.

**Where.** `docs/USER-GUIDE.md` §12, `src/strings/en.json`.

**Acceptance criteria.**
1. Each message quoted in §12 appears word for word in `src/strings/en.json` (or the row says it is a general symptom).
2. Any row whose quoted message is missing is listed in the PR body; the row is not rewritten to guess a new message.
3. No wording in `src/strings/en.json` changes in this PR.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## 18. Check that the name ScribeTrace is spelled the same everywhere

- **Labels:** `good first issue`, `docs`
- **Area:** docs
- **Difficulty:** easy

**What and why.** The product name appears in the README, the guide, the release draft and the UI. One spelling and one capitalisation everywhere makes the project look finished and helps search.

**Where.** `README.md`, `docs/*.md` (not `docs/website/`), `src/strings/en.json`, `CHANGELOG.md`.

**Acceptance criteria.**
1. Run a case-sensitive search for `Scribetrace`, `scribetrace` used as a product name (not a command or a path), and `Scribe Trace`; list each hit.
2. Fix each product-name hit to `ScribeTrace`; leave commands, binary names and file paths alone.
3. The PR body lists the files changed and the hits left on purpose.

Sign off your commits with `git commit -s` (see CONTRIBUTING).

---

## Summary

| # | Title | Labels | Difficulty |
|---|---|---|---|
| 1 | Fix broken relative links | good first issue | easy |
| 2 | Windows validation report template | help wanted | easy |
| 3 | Run tests on macOS and report | help wanted | medium |
| 4 | Screenshots for guide and README | help wanted | medium |
| 5 | Review error-message wording | good first issue | easy |
| 6 | Second UI locale | help wanted | medium |
| 7 | Button type attribute test | good first issue | easy |
| 8 | CLI exit codes for bad calls | good first issue | easy |
| 9 | Exit codes in CLI usage text | good first issue | easy |
| 10 | Golden test for empty export | good first issue | easy |
| 11 | Zotero fixture: no DOI, no PDF | help wanted | medium |
| 12 | Test vector: cut-off JSON line | help wanted | medium |
| 13 | Flatpak manifest for local builds | help wanted | medium |
| 14 | AUR package file for local builds | help wanted | medium |
| 15 | Keyboard check of the setup screen | help wanted | medium |
| 16 | Reproducible e2e readme | good first issue | easy |
| 17 | Troubleshooting table vs real messages | good first issue | easy |
| 18 | Product name spelling | good first issue | easy |

Good first issue: 1, 5, 7, 8, 9, 10, 16, 17, 18 (9 issues). Help wanted: 2, 3, 4, 6, 11, 12, 13, 14, 15 (9 issues).
