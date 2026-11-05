#!/usr/bin/env bash
# Architecture and budget gates from docs/BUILD-PLAN.md slice 1 and docs/DEFINITION-OF-DONE.md.
# Run from the repo root. Needs: cargo, pnpm lockfile. Exits non-zero on the first broken rule.
set -euo pipefail
cd "$(dirname "$0")/.."
fail() { echo "GATE FAILED: $*" >&2; exit 1; }

# 1. The core must not depend on the shell, a TLS stack, a file watcher or a PDF library.
for forbidden in tauri rustls openssl native-tls notify pdfium-render; do
  if cargo tree -p scribetrace-core -e normal -i "$forbidden" >/dev/null 2>&1; then
    fail "scribetrace-core depends on $forbidden"
  fi
done
echo "ok: core is free of tauri, TLS stacks, notify, pdf libraries"

# 2. No PDF.js in Stage 1.
if grep -q "pdfjs-dist" pnpm-lock.yaml; then fail "pdfjs-dist is in pnpm-lock.yaml (Stage 2 only)"; fi
echo "ok: pdfjs-dist absent from lockfile"

# 3. Budgets. Direct crates <= 15 per ARCHITECTURE; direct JS packages <= 6 (types-only @types/* and
#    tooling for tests are excluded and listed in DEFINITION-OF-DONE).
direct_rust=$(cargo metadata --format-version 1 --no-deps | python3 -c '
import json,sys
m=json.load(sys.stdin); members={p["name"] for p in m["packages"]}
deps=set()
for p in m["packages"]:
    if p["name"]=="zotero-standin": continue
    for d in p["dependencies"]:
        if d["kind"] in (None,"normal","build") and d["name"] not in members: deps.add(d["name"])
print(len(deps))')
[ "$direct_rust" -le 15 ] || fail "direct Rust crates $direct_rust > 15"
echo "ok: direct Rust crates (normal+build, shipped) = $direct_rust (<= 15)"
direct_js=$(node -e '
const p=require("./package.json");
const n=Object.keys({...p.dependencies,...p.devDependencies}).filter(k=>!k.startsWith("@types/"));
console.log(n.length)')
[ "$direct_js" -le 6 ] || fail "direct JS packages $direct_js > 6 (excluding @types/*)"
echo "ok: direct JS packages (excluding @types/*) = $direct_js (<= 6)"

# 4. Transitive guard: record, fail at +20% over the baseline file.
unique=$(cargo tree -p scribetrace-shell -e normal --prefix none | sort -u | wc -l)
base_file=tools/transitive-baseline.txt
if [ -f "$base_file" ]; then
  base=$(cat "$base_file"); limit=$(( base + base / 5 ))
  [ "$unique" -le "$limit" ] || fail "transitive crates $unique > $limit (baseline $base +20%)"
  echo "ok: transitive crates (shell) = $unique (baseline $base, limit $limit)"
else
  echo "note: no $base_file; shell transitive crates = $unique (record it to enable the +20% guard)"
fi
