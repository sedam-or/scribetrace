import { invoke } from "@tauri-apps/api/core";

import type { Rpc } from "./rpc.ts";
import type { HistoryEvent, ImportReport, PaperItem, WorkspaceInfo } from "./types.ts";
import { wrap } from "./wrap.ts";

/** The single place that talks to Tauri. Lint rule: no other file may import @tauri-apps. */
export const tauriRpc: Rpc = {
  chooseFolder: () => wrap(invoke<WorkspaceInfo | null>("choose_folder")),
  getWorkspace: () => wrap(invoke<WorkspaceInfo | null>("get_workspace")),
  connectZotero: () => wrap(invoke<PaperItem[]>("list_zotero_papers")),
  importMarks: (paperId, attachmentKey) =>
    wrap(invoke<ImportReport>("import_marks", { paperId, attachmentKey })),
  readHistory: (paperId) => wrap(invoke<HistoryEvent[]>("read_history", { paperId })),
};
