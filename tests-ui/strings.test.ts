import assert from "node:assert/strict";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";

import rawStrings from "../src/strings/en.json" with { type: "json" };
import { t } from "../src/strings.ts";

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

/**
 * Keys reserved for slices 4-14 that are not yet referenced in src/.
 * Invariant requested by architect & PM:
 * 1. Fails if any key in en.json is unused AND not in PENDING_UNUSED_KEYS.
 * 2. Fails if any key in PENDING_UNUSED_KEYS is actually used in src/ (forcing the list to shrink as slices land).
 * When all slices are built, this set becomes empty and the test enforces 100% coverage.
 */
const PENDING_UNUSED_KEYS: ReadonlySet<string> = new Set([
  "app.independent",
  "setup.title",
  "setup.intro",
  "setup.step1.title",
  "setup.step1.body",
  "setup.step1.action",
  "setup.step2.body",
  "setup.step3.title",
  "setup.step3.body",
  "setup.step3.action",
  "setup.connected",
  "setup.connected.one",
  "setup.waiting",
  "setup.contact_notice",
  "zotero.setting_name",
  "zotero.error.refused.title",
  "zotero.error.refused.body",
  "zotero.error.forbidden.title",
  "zotero.error.forbidden.body",
  "zotero.error.timeout.title",
  "zotero.error.timeout.body",
  "zotero.error.version.title",
  "zotero.error.version.body",
  "zotero.error.too_large.title",
  "zotero.error.too_large.body",
  "zotero.error.no_retry",
  "zotero.action.check_again",
  "zotero.action.back_to_setup",
  "papers.empty.title",
  "papers.empty.body",
  "papers.empty.action",
  "papers.no_marks",
  "papers.no_marks.help",
  "papers.import.progress",
  "papers.import.done",
  "papers.import.none",
  "papers.refresh.action",
  "papers.refresh.unchanged",
  "history.milestone.created",
  "history.milestone.rename.action",
  "history.changes.title",
  "history.changes.added",
  "history.changes.removed",
  "history.changes.edited",
  "history.changes.none",
  "history.origin.zotero",
  "history.skipped_lines",
  "history.restore.done",
  "quote.copy.action",
  "quote.copy.done",
  "quote.copy.no_key",
  "quote.copy.missing_author",
  "quote.link.confirm.title",
  "quote.link.confirm.body",
  "quote.link.confirm.action",
  "notes.outside_edit",
  "notes.too_large",
  "notes.saved",
  "forget.action",
  "forget.confirm.title",
  "forget.confirm.body",
  "forget.confirm.action",
  "forget.cancel",
  "forget.done",
  "forget.banner",
  "forget.banner.action",
  "forget.banner.dismiss",
  "forget.banner.done",
  "obsidian.export.action",
  "obsidian.export.choose",
  "obsidian.export.done",
  "obsidian.export.skipped",
  "obsidian.export.refused",
  "obsidian.export.oneway",
  "update.check.action",
  "update.check.body",
  "privacy.summary",
  "error.generic",
]);

test("every en.json key is either used in src/ or declared in PENDING_UNUSED_KEYS", () => {
  const allKeys = Object.keys(rawStrings).filter((k) => k !== "_meta");

  function* walk(dir: string): Generator<string> {
    for (const name of readdirSync(dir)) {
      const p = join(dir, name);
      if (statSync(p).isDirectory()) yield* walk(p);
      else if (/\.(ts|tsx)$/.test(name) && !p.includes("strings.ts")) yield p;
    }
  }

  const usedKeys = new Set<string>();
  for (const file of walk("src")) {
    const text = readFileSync(file, "utf8");
    for (const k of allKeys) {
      if (text.includes(`"${k}"`) || text.includes(`'${k}'`)) {
        usedKeys.add(k);
      }
    }
  }

  // 1. Any key in en.json must be used or in PENDING
  const unannouncedUnused: string[] = [];
  for (const k of allKeys) {
    if (!usedKeys.has(k) && !PENDING_UNUSED_KEYS.has(k)) {
      unannouncedUnused.push(k);
    }
  }
  assert.deepEqual(
    unannouncedUnused,
    [],
    `Found unused en.json keys that are not listed in PENDING_UNUSED_KEYS: ${unannouncedUnused.join(", ")}`,
  );

  // 2. Any key in PENDING must NOT be used (forces the list to shrink)
  const obsoletePending: string[] = [];
  for (const k of PENDING_UNUSED_KEYS) {
    if (usedKeys.has(k)) {
      obsoletePending.push(k);
    }
  }
  assert.deepEqual(
    obsoletePending,
    [],
    `Keys in PENDING_UNUSED_KEYS are now used in src/ and must be removed from PENDING: ${obsoletePending.join(", ")}`,
  );
});
