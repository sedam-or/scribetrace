import assert from "node:assert/strict";
import { test } from "node:test";

import { createMockRpc, FIXTURE_MILESTONES } from "../src/rpc/mock.ts";

test("mock milestone fixture lists five milestones in display order", async () => {
  const rpc = createMockRpc();
  const milestones = await rpc.list_milestones?.("6WG6QNHB");

  assert.equal(milestones?.length, 5);
  assert.deepEqual(milestones, FIXTURE_MILESTONES);
  assert.deepEqual(milestones?.map((milestone) => milestone.reason), [
    "import",
    "refresh",
    "session",
    "manual",
    "refresh",
  ]);
});

test("mock milestone diff reports added, removed, edited marks and note lines", async () => {
  const rpc = createMockRpc();
  const diff = await rpc.milestone_diff?.(
    "6WG6QNHB",
    "milestone-refresh",
    "milestone-session",
  );

  assert.equal(diff?.from, "milestone-refresh");
  assert.equal(diff?.to, "milestone-session");
  assert.equal(diff?.added.length, 5);
  assert.equal(diff?.removed.length, 1);
  assert.equal(diff?.edited.length, 1);
  assert.equal(diff?.edited[0]?.key, "mark-010");
  assert.deepEqual(diff?.edited[0]?.noteDiff, [
    { op: "remove", text: "Margin note 10." },
    { op: "add", text: "Margin note 10 revised." },
    { op: "equal", text: "This point is useful context." },
  ]);
  assert.deepEqual(diff?.noteDiff, [
    { op: "equal", text: "Read-through notes: baseline." },
    { op: "equal", text: "Added a question about scope." },
    { op: "add", text: "Second reading clarified the central claim." },
  ]);
});

test("mock milestone diff returns a typed missing-milestone error", async () => {
  const rpc = createMockRpc();
  await assert.rejects(
    rpc.milestone_diff!("6WG6QNHB", "hidden-milestone", "milestone-latest"),
    { code: "milestone_missing" },
  );
});

test("latest mock milestone state contains 200 marks", async () => {
  const rpc = createMockRpc();
  const initial = await rpc.milestone_diff?.("6WG6QNHB", "milestone-import", "milestone-latest");
  assert.equal(initial?.added.length, 20);
  assert.equal(initial?.removed.length, 0);
});
