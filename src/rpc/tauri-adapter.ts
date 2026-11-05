import { invoke } from "@tauri-apps/api/core";

import type { Rpc } from "./rpc.ts";
import type {
  HistoryEvent,
  ImportReport,
  PaperItem,
  WorkspaceInfo,
  ZoteroErrorCode,
} from "./types.ts";
import { ZoteroError } from "./types.ts";

function wrap<T>(promise: Promise<T>): Promise<T> {
  return promise.catch((err: unknown) => {
    if (err && typeof err === "object" && "code" in err) {
      const code = String((err as { code: unknown }).code);
      throw new ZoteroError(code as ZoteroErrorCode);
    }
    throw err;
  });
}

/** The single place that talks to Tauri. Lint rule: no other file may import @tauri-apps. */
export const tauriRpc: Rpc = {
  chooseFolder: () => invoke<WorkspaceInfo | null>("choose_folder"),
  getWorkspace: () => invoke<WorkspaceInfo | null>("get_workspace"),
  connectZotero: () => wrap(invoke<PaperItem[]>("list_zotero_papers")),
  importMarks: (paperId, attachmentKey) =>
    wrap(invoke<ImportReport>("import_marks", { paperId, attachmentKey })),
  readHistory: (paperId) => wrap(invoke<HistoryEvent[]>("read_history", { paperId })),
};
