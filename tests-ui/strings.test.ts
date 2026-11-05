import assert from "node:assert/strict";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";

import rawStrings from "../src/strings/en.json" with { type: "json" };
import { t, type StringKey } from "../src/strings.ts";

test("t() returns plain strings correctly", () => {
  assert.equal(t("app.name"), "ScribeTrace");
  assert.equal(t("papers.import.action"), "Import marks");
});

test("t() interpolates parameters", () => {
  const setting = t("zotero.setting_name");
  assert.equal(
    t("setup.step1.action", { zotero_setting: setting }),
    'In Zotero, open Settings, then Advanced, and tick "Allow other applications on this computer to communicate with Zotero".',
  );
  assert.equal(t("setup.connected", { count: 4 }), "Connected, 4 papers.");
});

test("t() throws if a placeholder is missing", () => {
  assert.throws(() => {
    t("setup.connected");
  }, /Missing required placeholder/);
});

test("en.json contains no disallowed marketing phrases per BRAND.md", () => {
  const jsonStr = JSON.stringify(rawStrings);
  assert.doesNotMatch(jsonStr, /verifiable history/i);
  assert.doesNotMatch(jsonStr, /Honest Privacy Invariant/i);
  assert.doesNotMatch(jsonStr, /blockchain|tamper-proof/i);
});

test("en.json all entries are non-empty strings with valid placeholders", () => {
  const entries = Object.entries(rawStrings).filter(([k]) => k !== "_meta");
  assert.ok(entries.length > 50, "en.json should have comprehensive entries");

  for (const [key, value] of entries) {
    assert.equal(typeof value, "string", `Key ${key} must be a string`);
    if (typeof value !== "string") continue;
    assert.ok(value.length > 0, `Key ${key} cannot be empty`);
    // Assert balanced curly braces for placeholders
    const openBraces = (value.match(/\{/g) || []).length;
    const closeBraces = (value.match(/\}/g) || []).length;
    assert.equal(openBraces, closeBraces, `Key ${key} has unbalanced placeholder braces: ${value}`);
  }
});

test("all t() calls in src/ reference valid keys in en.json", () => {
  const keys = new Set(Object.keys(rawStrings).filter((k) => k !== "_meta"));

  function* walk(dir: string): Generator<string> {
    for (const name of readdirSync(dir)) {
      const p = join(dir, name);
      if (statSync(p).isDirectory()) yield* walk(p);
      else if (/\.(ts|tsx)$/.test(name) && !p.includes("strings.ts")) yield p;
    }
  }

  const calledKeys: string[] = [];
  for (const file of walk("src")) {
    const text = readFileSync(file, "utf8");
    const matches = text.matchAll(/\bt\(["']([^"']+)["']/g);
    for (const m of matches) {
      if (m[1]) {
        calledKeys.push(m[1]);
      }
    }
  }

  assert.ok(calledKeys.length > 0, "Expected some t() calls in src");
  for (const key of calledKeys) {
    assert.ok(keys.has(key), `Key "${key}" called in src is missing in en.json`);
  }
});
