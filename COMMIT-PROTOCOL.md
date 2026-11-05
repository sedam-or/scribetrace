# Commit Protocol

Automated and manual commit protocol for ScribeTrace. Follow this protocol strictly
before creating commits or opening pull requests. Never commit unverified code or broken tests.

## 1. Commit Message Convention

ScribeTrace follows the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification:

```
<type>(<scope>): <short description in present tense>

[optional body explaining WHY, context, and architectural impact]

[optional footer(s), e.g., Closes #123, Refs SEC-12]
```

### Allowed Types
- `feat`: A new user-facing capability or format feature.
- `fix`: A bug fix or error recovery correction.
- `docs`: Documentation changes, design specifications, or ADR updates.
- `refactor`: Code changes that neither fix a bug nor add a feature.
- `perf`: Performance improvements (memory, load times, scanning).
- `test`: Adding or correcting tests; property-based test enhancements.
- `chore`: CI/CD workflows, build configurations, dependency updates.
- `ci`: Changes to GitHub Actions pipelines or automation scripts.

### Scopes
Examples: `core`, `storage`, `importer`, `zotero`, `export`, `ui`, `spec`, `packaging`.

---

## 2. Mandatory Pre-Commit Gates

Before creating any commit, verify the following gates locally:

1. **Working Tree Cleanliness:**
   ```bash
   git status
   git diff
   ```
   Ensure no temporary files, editor artifacts, or assistant plumbing (`.agentsroom/`, `.claude/`) are staged.

2. **Secrets & Credentials Audit:**
   ```bash
   git diff --staged | grep -iE "(api[_-]?key|secret|password|bearer|auth|token)"
   ```
   Verify 0 secrets or environment credentials are being committed.

3. **Format & Static Analysis (when code is present):**
   ```bash
   cargo fmt --check
   cargo clippy --all-targets -- -D warnings
   npm run lint
   npm run typecheck
   ```

4. **Test Suite:**
   ```bash
   cargo test
   npm test
   ```

---

## 3. Delegation & Committer Protocol

- Multi-agent commit policy: Agents commit their own paths signed off (`git commit -s`), except PM whose paths DevOps commits.
- CI stays green on all commits.
- The project owner reviews changes per commit.
- Never force-push (`git push --force`) to shared branches.
- Never commit broken links, unformatted Rust/TS files, or unverified spikes.
