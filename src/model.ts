import type { Rpc } from "./rpc/rpc.ts";
import type { HistoryEvent } from "./rpc/types.ts";

/** The one hard-wired paper of the walking skeleton (replaced by the paper list in slice 4). */
export const SLICE1_PAPER = { paperId: "z_EFGH6789", attachmentKey: "EFGH6789" } as const;

export type ViewState =
  | { kind: "idle" }
  | { kind: "busy" }
  | { kind: "error"; message: string }
  | { kind: "done"; imported: number; events: HistoryEvent[] };

/** Imports marks and reads history for the selected paper. Never throws: failures become state. */
export async function importPaper(
  rpc: Rpc,
  _folder?: string,
  paperId: string = SLICE1_PAPER.paperId,
  attachmentKey: string = SLICE1_PAPER.attachmentKey,
): Promise<ViewState> {
  try {
    const ws = await rpc.getWorkspace();
    if (!ws) {
      const chosen = await rpc.chooseFolder();
      if (!chosen) {
        return { kind: "error", message: "No project folder is open." };
      }
    }
    const report = await rpc.importMarks(paperId, attachmentKey);
    const events = await rpc.readHistory(paperId);
    return { kind: "done", imported: report.imported, events };
  } catch (e) {
    return { kind: "error", message: e instanceof Error ? e.message : String(e) };
  }
}
