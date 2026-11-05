import { invoke } from "@tauri-apps/api/core";

import type { Rpc } from "./rpc.ts";
import type { HistoryEvent, ImportReport, WorkspaceInfo } from "./types.ts";

/** The single place that talks to Tauri. Lint rule: no other file may import @tauri-apps. */
export const tauriRpc: Rpc = {
  openWorkspace: (path) => invoke<WorkspaceInfo>("open_workspace", { path }),
  importMarks: (paperId, attachmentKey) =>
    invoke<ImportReport>("import_marks", { paperId, attachmentKey }),
  readHistory: (paperId) => invoke<HistoryEvent[]>("read_history", { paperId }),
};
