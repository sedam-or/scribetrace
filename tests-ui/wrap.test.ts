import assert from "node:assert/strict";
import { test } from "node:test";

import { ZoteroError } from "../src/rpc/types.ts";
import { wrap } from "../src/rpc/wrap.ts";

async function codeOf(rejection: unknown): Promise<string> {
  try {
    await wrap(Promise.reject(rejection));
  } catch (e) {
    assert.ok(e instanceof ZoteroError, "always a ZoteroError");
    assert.ok(!e.message.includes("/etc"), "no raw text carried");
    return String(e.code);
  }
  throw new Error("wrap must reject");
}

test("wrap passes known { code } rejections through (SEC-39)", async () => {
  assert.equal(await codeOf({ code: "forbidden" }), "forbidden");
  assert.equal(await codeOf({ code: "too_large" }), "too_large");
  assert.equal(await codeOf({ code: "no_doi" }), "no_doi");
  assert.equal(await codeOf({ code: "no_zotero_key" }), "no_zotero_key");
  assert.equal(await codeOf({ code: "zotero_not_running" }), "zotero_not_running");
});

test("wrap turns raw string, Tauri and unknown rejections into internal (SEC-39)", async () => {
  assert.equal(await codeOf("command list_zotero_papers not allowed by ACL"), "internal");
  assert.equal(await codeOf("No such file: /etc/secret"), "internal");
  assert.equal(await codeOf(new Error("boom")), "internal");
  assert.equal(await codeOf({ code: "<script>alert(1)</script>" }), "internal");
  assert.equal(await codeOf(null), "internal");
});

test("wrap resolves values unchanged", async () => {
  assert.deepEqual(await wrap(Promise.resolve([1, 2])), [1, 2]);
});
