import assert from "node:assert/strict";
import { test } from "node:test";

import { createMockRpc, FIXTURE_PAPERS } from "../src/rpc/mock.ts";
import { createPaperLinkAdapter } from "../src/rpc/paper-link-adapter.ts";

test("mock paper_link returns full DOI and Zotero URIs without UI assembly", async () => {
  const rpc = createMockRpc();
  const paperId = FIXTURE_PAPERS[0]!.id;

  assert.equal("attachmentKey" in FIXTURE_PAPERS[0]!, false);
  assert.equal("zotero_key" in FIXTURE_PAPERS[0]!, false);

  assert.deepEqual(await rpc.paper_link?.({ paperId, target: "doi" }), {
    uri: "https://doi.org/10.48550/arXiv.1706.03762",
  });
  assert.deepEqual(await rpc.paper_link?.({ paperId, target: "pdf" }), {
    uri: "zotero://open-pdf/library/items/EFGH6789",
  });
  assert.deepEqual(await rpc.paper_link?.({ paperId, target: "item" }), {
    uri: "zotero://select/library/items/6WG6QNHB",
  });
});

test("mock paper_link returns core error codes when link data is missing", async () => {
  const rpc = createMockRpc();
  await assert.rejects(
    rpc.paper_link!({ paperId: FIXTURE_PAPERS[2]!.id, target: "doi" }),
    { code: "no_doi" },
  );
});

test("mock open_external returns the typed approval outcome", async () => {
  const rpc = createMockRpc();
  assert.equal(await rpc.open_external({ paperId: FIXTURE_PAPERS[0]!.id, target: "doi" }), "opened");
});

test("paper-link adapter maps UI actions to backend targets", async () => {
  const adapter = createPaperLinkAdapter(createMockRpc());
  const paperId = FIXTURE_PAPERS[0]!.id;

  assert.equal((await adapter.getUri!(paperId, "doi")).uri, "https://doi.org/10.48550/arXiv.1706.03762");
  assert.equal((await adapter.getUri!(paperId, "zotero_pdf")).uri, "zotero://open-pdf/library/items/EFGH6789");
  assert.equal((await adapter.getUri!(paperId, "zotero_select")).uri, "zotero://select/library/items/6WG6QNHB");
});

test("paper-link adapter confirms with only the typed target and paperId", async () => {
  const rpc = createMockRpc();
  let received: { target: string; paperId: string } | null = null;
  rpc.open_external = async (request) => {
    received = request;
    return "cancelled";
  };
  const adapter = createPaperLinkAdapter(rpc);

  assert.equal(await adapter.openExternal(FIXTURE_PAPERS[0]!.id, "zotero_pdf"), "cancelled");
  assert.deepEqual(received, { target: "pdf", paperId: FIXTURE_PAPERS[0]!.id });
});
