# ScribeTrace: Definition of Done

A slice is done only when **every row holds**. "CI" means the `workspace` job in [`ci.yml`](../.github/workflows/ci.yml) on Linux; Windows and macOS are informative until SPIKE-4 makes them required. Rows marked *review* cannot be automated and need a second agent or person.

| # | Rule | Enforced by |
|---|---|---|
| 1 | `cargo fmt --check` clean | CI |
| 2 | `cargo clippy --workspace --all-targets -D warnings` clean | CI |
| 3 | No `unwrap`, `expect`, `panic!`, `todo!`, `dbg!` outside tests; `unsafe` forbidden | workspace lints in [Cargo.toml](../Cargo.toml) (deny) |
| 4 | Typed errors (`thiserror`) in core; the shell turns them into plain-language strings | clippy + *review* |
| 5 | Unit tests, plus a property test for any parser, merge, id or path rule | CI (`cargo test`, `proptest`) |
| 6 | Core line coverage ≥ 80% (`cargo llvm-cov -p scribetrace-core`) | CI (`--fail-under-lines 80`) |
| 7 | UI: `pnpm typecheck`, `pnpm lint`, `pnpm test`, `pnpm build` pass | CI |
| 8 | UI boundary: `src/` has no Node built-ins, no network, no raw HTML, no polling timer; `@tauri-apps` only in `src/rpc/tauri-adapter.ts`, which only `main.tsx` imports | `tools/lint-ui-boundaries.mjs` (self-tested) |
| 9 | Core boundary: `scribetrace-core` does not depend on tauri, a TLS stack, `notify` or a PDF library; `pdfjs-dist` absent from the lockfile | `tools/check-gates.sh` |
| 10 | Budgets: ≤ 15 direct Rust crates, ≤ 6 direct JS packages (`@types/*` excluded: they ship no code), transitive crates ≤ baseline +20% ([tools/transitive-baseline.txt](../tools/transitive-baseline.txt)); installer ≤ 20 MB, idle ≤ 120 MB, cold start ≤ 2 s once SPIKE-4 measures them (slice 14) | `tools/check-gates.sh`; the last three are UNMEASURED until then |
| 11 | `cargo deny check` and `cargo audit` clean. An ignored advisory needs a reason in [deny.toml](../deny.toml) and the same id in the CI audit step | CI |
| 12 | Accessibility WCAG 2.2 AA per [design/components.md](../design/components.md): labelled controls, visible focus, status via `role`, contrast from tokens only | *review* (frontend slices); axe automation arrives with the first real window test |
| 13 | Docs updated in the same change: [ARCHITECTURE](ARCHITECTURE.md) or an ADR if a boundary moved, [CHANGELOG](../CHANGELOG.md) for user-visible change | *review* |
| 14 | Security: any new IPC command, file path, network call or parser input has a SEC row in [SECURITY-REQUIREMENTS](SECURITY-REQUIREMENTS.md) and a test | security *review* |
| 15 | Review by someone other than the author; nothing is done on its author's word. Conventional commit with DCO sign-off ([COMMIT-PROTOCOL](../COMMIT-PROTOCOL.md)) | *review* |

**New IPC command checklist** (rows 8, 14): add it to the command list in `src-tauri/build.rs`, add `allow-<name>` to `src-tauri/capabilities/default.json`, add it to `register` in `src-tauri/src/lib.rs`, add a method to the `Rpc` interface and its Tauri adapter, extend `src-tauri/tests/ipc_slice1.rs`-style test. The capability file is the allowlist; a command missing from it is refused by Tauri.

**Honest limits.** Rows 10 (installer, memory, start-up) and 12 (automated a11y) are not measured yet; no slice may claim them until they are. Linux is the only required platform for Stage 1 work.
