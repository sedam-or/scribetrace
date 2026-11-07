import assert from "node:assert/strict";
import { test } from "node:test";

import { createMockRpc, FIXTURE_PAPERS } from "../src/rpc/mock.ts";
import { createPaperLinkAdapter } from "../src/rpc/paper-link-adapter.ts";
import type { PaperLinkRequest } from "../src/rpc/types.ts";

test("Paper DTO keeps parent and attachment keys out of the UI", () => {
  assert.equal("attachmentKey" in FIXTURE_PAPERS[0]!, false);
  assert.equal("zotero_key" in FIXTURE_PAPERS[0]!, false);
  assert.equal("paper_link" in createMockRpc(), false);
});

test("mock open_external returns opened or cancelled without a URI", async () => {
  const opened = createMockRpc();
  const request: PaperLinkRequest = { paperId: FIXTURE_PAPERS[0]!.id, target: "doi" };
  assert.equal(await opened.open_external(request), "opened");

  const cancelled = createMockRpc([], { externalOutcome: "cancelled" });
  assert.equal(await cancelled.open_external(request), "cancelled");
});

test("adapter sends only typed target and paperId for every Paper action", async () => {
  const rpc = createMockRpc();
  const calls: PaperLinkRequest[] = [];
  rpc.open_external = async (request) => {
    calls.push(request);
    return "opened";
  };
  const adapter = createPaperLinkAdapter(rpc);
  const paperId = FIXTURE_PAPERS[0]!.id;

  await adapter.openExternal(paperId, "doi");
  await adapter.openExternal(paperId, "zotero_pdf");
  await adapter.openExternal(paperId, "zotero_select");

  assert.deepEqual(calls, [
    { target: "doi", paperId },
    { target: "pdf", paperId },
    { target: "item", paperId },
  ]);
  assert.ok(calls.every((request) => !("uri" in request)));
});

test("mock open_external reports backend availability codes", async () => {
  const rpc = createMockRpc();
  const paperId = FIXTURE_PAPERS[2]!.id;

  await assert.rejects(rpc.open_external({ paperId, target: "doi" }), { code: "no_doi" });
  await assert.rejects(rpc.open_external({ paperId, target: "pdf" }), { code: "no_zotero_key" });
  await assert.rejects(rpc.open_external({ paperId, target: "item" }), { code: "no_zotero_key" });
});
