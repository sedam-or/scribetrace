// Wire types. They mirror the serde output of scribetrace-core (snake_case fields).

export interface Mark {
  key: string;
  kind: string;
  text: string | null;
  comment: string | null;
  color: string | null;
  page_index: number | null;
  added: string | null;
  modified: string | null;
}

export interface HistoryEvent {
  v: number;
  id: string;
  device: string;
  seq: number;
  ts: string;
  author: string;
  type: "mark_import";
  payload: Mark;
}

export interface WorkspaceInfo {
  path: string;
}

export interface ImportReport {
  imported: number;
  milestone: string | null;
}

export interface PaperItem {
  id: string;
  title: string;
  authors?: string;
  year?: string | number;
  marksCount: number;
  attachmentKey?: string;
  doi: string | null;
  zotero_key: string | null;
}

export type PaperLinkKind = "doi" | "zotero_pdf" | "zotero_select";
export type PaperLinkTarget = "doi" | "pdf" | "item";

export interface PaperLinkRequest {
  target: PaperLinkTarget;
  paperId: string;
}

export interface PaperLink {
  uri: string;
}

export type ZoteroErrorCode =
  | "refused"
  | "forbidden"
  | "timeout"
  | "version"
  | "too_large"
  | "bad_answer"
  | "no_doi"
  | "no_zotero_key"
  | "zotero_not_running"
  | "internal";

export class ZoteroError extends Error {
  readonly code: ZoteroErrorCode | string;
  readonly detail?: string;
  readonly seconds?: number;

  constructor(code: ZoteroErrorCode | string, detail?: string, seconds?: number) {
    super(detail ? `${code}: ${detail}` : code);
    this.name = "ZoteroError";
    this.code = code;
    this.detail = detail;
    this.seconds = seconds;
  }
}
