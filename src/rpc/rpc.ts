import type { HistoryEvent, ImportReport, WorkspaceInfo } from "./types.ts";

/**
 * The only way the UI reaches the core. Tauri `invoke` is one adapter (tauri-adapter.ts);
 * an HTTP adapter for the `scribetrace serve` fallback would be the second.
 */
export interface Rpc {
  openWorkspace(path: string): Promise<WorkspaceInfo>;
  importMarks(paperId: string, attachmentKey: string): Promise<ImportReport>;
  readHistory(paperId: string): Promise<HistoryEvent[]>;
}
