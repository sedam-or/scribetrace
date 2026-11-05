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
}
