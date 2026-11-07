import type { HistoryEvent, ImportReport, PaperItem, PaperLink, PaperLinkRequest, WorkspaceInfo } from "./types.ts";

/**
 * The only way the UI reaches the core. Tauri `invoke` is one adapter (tauri-adapter.ts);
 * an HTTP adapter for the `scribetrace serve` fallback would be the second.
 */
export interface Rpc {
  chooseFolder(): Promise<WorkspaceInfo | null>;
  getWorkspace(): Promise<WorkspaceInfo | null>;
  connectZotero(): Promise<PaperItem[]>;
  importMarks(paperId: string, attachmentKey: string): Promise<ImportReport>;
  readHistory(paperId: string): Promise<HistoryEvent[]>;
  paper_link?(request: PaperLinkRequest): Promise<PaperLink>;
  open_link?(request: PaperLinkRequest): Promise<void>;
}
