// Fails when UI code (src/) reaches around the typed Rpc seam. See docs/ARCHITECTURE.md layering.
// Rules: no Node built-ins, no @tauri-apps outside the adapter, no tauri-adapter import outside
// main.tsx, no network APIs, no timers that poll, no raw HTML or eval.
import { readdirSync, readFileSync, statSync } from "node:fs";
import { builtinModules } from "node:module";
import { join, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";

const ADAPTER = "src/rpc/tauri-adapter.ts";
const ENTRY = "src/main.tsx";
const builtins = new Set(builtinModules.map((m) => m.replace(/^node:/, "").split("/")[0]));

const BANNED_CALLS = [
  [/\bfetch\s*\(/, "network call (fetch)"],
  [/\bXMLHttpRequest\b/, "network call (XMLHttpRequest)"],
  [/\bWebSocket\b/, "network call (WebSocket)"],
  [/\bEventSource\b/, "network call (EventSource)"],
  [/\bsendBeacon\b/, "network call (sendBeacon)"],
  [/\bsetInterval\s*\(/, "polling timer (setInterval)"],
  [/\bdangerouslySetInnerHTML\b/, "raw HTML (dangerouslySetInnerHTML)"],
  [/\.innerHTML\b/, "raw HTML (innerHTML)"],
  [/\beval\s*\(/, "eval"],
  [/\bnew\s+Function\s*\(/, "new Function"],
];

/** @returns {string[]} violations for one file; `file` is a repo-relative path with `/`. */
export function lintSource(file, text) {
  const out = [];
  const specs = [
    ...text.matchAll(/\bfrom\s+["']([^"']+)["']/g),
    ...text.matchAll(/\bimport\s*\(\s*["']([^"']+)["']\s*\)/g),
    ...text.matchAll(/\brequire\s*\(\s*["']([^"']+)["']\s*\)/g),
    ...text.matchAll(/^\s*import\s+["']([^"']+)["']/gm),
  ].map((m) => m[1]);
  for (const spec of specs) {
    const root = spec.replace(/^node:/, "").split("/")[0];
    if (spec.startsWith("node:") || builtins.has(root)) out.push(`${file}: Node built-in import "${spec}"`);
    if (spec.startsWith("@tauri-apps/") && file !== ADAPTER) out.push(`${file}: "${spec}" only allowed in ${ADAPTER}`);
    if (/(^|\/)tauri-adapter(\.[a-z]+)?$/.test(spec) && file !== ENTRY) out.push(`${file}: tauri-adapter only allowed in ${ENTRY}`);
  }
  const code = text.replace(/\/\*[\s\S]*?\*\//g, "").replace(/(^|[^:])\/\/.*$/gm, "$1");
  for (const [re, what] of BANNED_CALLS) if (re.test(code)) out.push(`${file}: ${what}`);
  return out;
}

function* walk(dir) {
  for (const name of readdirSync(dir)) {
    const p = join(dir, name);
    if (statSync(p).isDirectory()) yield* walk(p);
    else if (/\.(ts|tsx|js|jsx|mjs)$/.test(name)) yield p;
  }
}

const root = fileURLToPath(new URL("..", import.meta.url));
if (process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1]) {
  const problems = [];
  let files = 0;
  for (const p of walk(join(root, "src"))) {
    files += 1;
    problems.push(...lintSource(relative(root, p).split(sep).join("/"), readFileSync(p, "utf8")));
  }
  if (problems.length > 0) {
    console.error(problems.join("\n"));
    process.exit(1);
  }
  console.log(`ui-boundaries: ${files} files clean`);
}
