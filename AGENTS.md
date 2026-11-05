# AI Agent Operating Contract — ScribeTrace

This document governs the operating contract, boundaries, and conventions for AI coding agents working on the ScribeTrace codebase.

## 1. Architectural Authority & Invariants

All design decisions must align with:
- `docs/ARCHITECTURE.md` (System design, layering rules, and component boundaries).
- `docs/DECISION-LOG.md` (Ratified architectural decisions).
- `docs/SECURITY-REQUIREMENTS.md` (Security constraints SEC-1 through SEC-39).

### Layering Rules
1. **Core Storage & Engine (`src-core/`):** Pure, platform-agnostic Rust library (`scribetrace-core`). Zero UI dependencies. Interacts strictly with `.scribetrace/` sidecar storage.
2. **IPC Gateway (`src-tauri/`):** Strict capability-based allowlists. No raw shell execution. Confined strictly to workspace paths.
3. **Presentation Shell (`src/`):** React + TypeScript UI. Operates over typed IPC RPC. Zero raw filesystem or direct network access.
4. **Offline Invariant:** Zero outbound network calls by default. Loopback (`127.0.0.1:23119`) allowed exclusively on explicit user click to query Zotero Local API.

---

## 2. Verification Protocol

Before declaring any task complete or committing:
- Maintain documentation integrity and update ADRs if architectural invariants shift.
- Follow `COMMIT-PROTOCOL.md` for conventional commit messaging and pre-commit checks.
- Build and test commands will be expanded as spikes graduate into the main codebase.

---

## 3. AgentsRoom Plumbing Boundaries

- The directory `.agentsroom/`, `.claude/`, `.agents/`, and `.mcp.json` are internal harness runtime state.
- **NEVER** stage or commit files from these paths into git.
- **NEVER** edit files in `.agentsroom/` directly except for assigned agent session files.
