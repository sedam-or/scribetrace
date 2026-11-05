// Slice 1 acceptance (b): the view, driven by a mock Rpc, shows the mark and the history line.
import assert from "node:assert/strict";
import { test } from "node:test";

import { renderToStaticMarkup } from "react-dom/server";

import { HistoryPanel } from "../src/HistoryPanel.tsx";
import { importPaper } from "../src/model.ts";
import { createMockRpc } from "../src/rpc/mock.ts";
import type { Mark } from "../src/rpc/types.ts";

const mark: Mark = {
  key: "WXYZ2345",
  kind: "highlight",
  text: "Append-only logs make history trustworthy.",
  comment: "Core claim of section 2.",
  color: "#ffd400",
  page_index: 3,
  added: "2026-10-30T09:15:00Z",
  modified: "2026-10-30T09:15:00Z",
};

test("imported mark and its history line are shown", async () => {
  const state = await importPaper(createMockRpc([mark]), "/tmp/project");
  assert.equal(state.kind, "done");
  const html = renderToStaticMarkup(<HistoryPanel state={state} />);
  assert.match(html, /Append-only logs make history trustworthy\./);
  assert.match(html, /mark_import/);
  assert.match(html, /Me/);
  assert.match(html, /Imported 1 mark/);
});

test("hostile mark text is rendered inert", async () => {
  const evil = { ...mark, text: "<img src=x onerror=alert(1)><script>x()</script>" };
  const state = await importPaper(createMockRpc([evil]), "/tmp/project");
  const html = renderToStaticMarkup(<HistoryPanel state={state} />);
  assert.doesNotMatch(html, /<script/);
  assert.doesNotMatch(html, /<img/);
  assert.match(html, /&lt;script&gt;/);
});

test("a failing Rpc becomes an error state, not an exception", async () => {
  const rpc = createMockRpc([mark], { initialWorkspace: null });
  rpc.chooseFolder = () => Promise.reject(new Error("That path is not a folder."));
  const state = await importPaper(rpc, "/nope");
  assert.deepEqual(state, { kind: "error", message: "That path is not a folder." });
  assert.match(renderToStaticMarkup(<HistoryPanel state={state} />), /role="alert"/);
});
