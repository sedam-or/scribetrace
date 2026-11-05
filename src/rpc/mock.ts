import type { Rpc } from "./rpc.ts";
import type { HistoryEvent, Mark } from "./types.ts";

/** In-memory Rpc for tests and for running the UI without the shell. */
export function createMockRpc(marks: Mark[]): Rpc {
  const history: HistoryEvent[] = [];
  let open = false;
  return {
    openWorkspace: (path) => {
      open = true;
      return Promise.resolve({ path });
    },
    importMarks: (_paperId, _attachmentKey) => {
      if (!open) return Promise.reject(new Error("No project folder is open."));
      for (const mark of marks) {
        history.push({
          v: 1,
          id: `mock-${history.length + 1}`,
          device: "local-1",
          seq: history.length + 1,
          ts: "2026-11-06T10:00:00Z",
          author: "Me",
          type: "mark_import",
          payload: mark,
        });
      }
      return Promise.resolve({ imported: marks.length });
    },
    readHistory: () => Promise.resolve([...history]),
  };
}
