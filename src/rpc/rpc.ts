import type {
  ExternalOpenOutcome,
  HistoryEvent,
  ImportReport,
  Milestone,
  MilestoneDiff,
  PaperItem,
  PaperLink,
  PaperLinkRequest,
  WorkspaceInfo,
} from "./types.ts";

/**
 * The only way the UI reaches the core. Tauri `invoke` is one adapter (tauri-adapter.ts);
 * an HTTP adapter for the `scribetrace serve` fallback would be the second.
 */
export interface Rpc {
  chooseFolder(): Promise<WorkspaceInfo | null>;
  getWorkspace(): Promise<WorkspaceInfo | null>;
  connectZotero(): Promise<PaperItem[]>;
  importMarks(paperId: string): Promise<ImportReport>;
  readHistory(paperId: string): Promise<HistoryEvent[]>;
  list_milestones?(paperId: string): Promise<Milestone[]>;
  milestone_diff?(paperId: string, fromId: string, toId: string): Promise<MilestoneDiff>;
  paper_link?(request: PaperLinkRequest): Promise<PaperLink>;
  open_external(request: PaperLinkRequest): Promise<ExternalOpenOutcome>;
}
