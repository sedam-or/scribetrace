import { invoke } from "@tauri-apps/api/core";

import type { Rpc } from "./rpc.ts";
import type { HistoryEvent, ImportReport, PaperItem, WorkspaceInfo } from "./types.ts";

/** The single place that talks to Tauri. Lint rule: no other file may import @tauri-apps. */
export const tauriRpc: Rpc = {
  chooseFolder: () => invoke<WorkspaceInfo | null>("choose_folder"),
  getWorkspace: () => invoke<WorkspaceInfo | null>("get_workspace"),
  connectZotero: () => invoke<PaperItem[]>("list_zotero_papers"),
  importMarks: (paperId, attachmentKey) =>
    invoke<ImportReport>("import_marks", { paperId, attachmentKey }),
  readHistory: (paperId) => invoke<HistoryEvent[]>("read_history", { paperId }),
};
