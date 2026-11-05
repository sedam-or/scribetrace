import type { Rpc } from "./rpc/rpc.ts";
import type { HistoryEvent } from "./rpc/types.ts";

/** The one hard-wired paper of the walking skeleton (replaced by the paper list in slice 4). */
export const SLICE1_PAPER = { paperId: "z_EFGH6789", attachmentKey: "EFGH6789" } as const;

export type ViewState =
  | { kind: "idle" }
  | { kind: "busy" }
  | { kind: "error"; message: string }
  | { kind: "done"; imported: number; events: HistoryEvent[] };

/** Opens the folder, imports the marks, reads the history. Never throws: failures become state. */
export async function importPaper(rpc: Rpc, folder: string): Promise<ViewState> {
  try {
    await rpc.openWorkspace(folder);
    const report = await rpc.importMarks(SLICE1_PAPER.paperId, SLICE1_PAPER.attachmentKey);
    const events = await rpc.readHistory(SLICE1_PAPER.paperId);
    return { kind: "done", imported: report.imported, events };
  } catch (e) {
    return { kind: "error", message: e instanceof Error ? e.message : String(e) };
  }
}
