import assert from "node:assert/strict";
import { test } from "node:test";

import { lintSource } from "./lint-ui-boundaries.mjs";

const bad = (file, text) => lintSource(file, text).length > 0;

test("flags Node built-ins and fs", () => {
  assert.ok(bad("src/a.ts", `import { readFileSync } from "node:fs";`));
  assert.ok(bad("src/a.ts", `import fs from "fs";`));
  assert.ok(bad("src/a.ts", `const x = require("child_process");`));
  assert.ok(bad("src/a.ts", `await import("path");`));
});
test("tauri only in the adapter, adapter only in main", () => {
  assert.ok(bad("src/App.tsx", `import { invoke } from "@tauri-apps/api/core";`));
  assert.ok(!bad("src/rpc/tauri-adapter.ts", `import { invoke } from "@tauri-apps/api/core";`));
  assert.ok(bad("src/App.tsx", `import { tauriRpc } from "./rpc/tauri-adapter.ts";`));
  assert.ok(!bad("src/main.tsx", `import { tauriRpc } from "./rpc/tauri-adapter.ts";`));
});
test("flags network, polling, raw HTML, eval", () => {
  for (const code of ["fetch('/x')", "new WebSocket('ws://x')", "setInterval(f, 1)", "el.innerHTML = x", "eval('1')", "<div dangerouslySetInnerHTML={x} />"]) {
    assert.ok(bad("src/a.tsx", code), code);
  }
});
test("ignores comments and clean code", () => {
  assert.ok(!bad("src/a.ts", `// we never call fetch( here\nimport { useState } from "react";`));
});
