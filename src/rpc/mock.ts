import type { Rpc } from "./rpc.ts";
import {
  type HistoryEvent,
  type Mark,
  type PaperItem,
  type PaperLinkRequest,
  type WorkspaceInfo,
  type ZoteroErrorCode,
  ZoteroError,
} from "./types.ts";

export interface MockRpcOptions {
  initialWorkspace?: string | null;
  chosenFolder?: string | null;
  papers?: PaperItem[];
  failZoteroCode?: ZoteroErrorCode | string | null;
  zoteroDetail?: string;
  zoteroSeconds?: number;
}

/** Standard fixture papers drawn from tests/fixtures/zotero/ */
export const FIXTURE_PAPERS: PaperItem[] = [
  {
    id: "6WG6QNHB",
    title: "Attention is all you need",
    authors: "Vaswani et al.",
    year: "2017",
    marksCount: 3,
    attachmentKey: "EFGH6789",
    doi: "10.48550/arXiv.1706.03762",
    zotero_key: "6WG6QNHB",
  },
  {
    id: "KBPLWQT5",
    title: "Deep residual learning for image recognition",
    authors: "He et al.",
    year: "2016",
    marksCount: 2,
    attachmentKey: "H3E7BPJW",
    doi: "10.1109/CVPR.2016.90",
    zotero_key: "KBPLWQT5",
  },
  {
    id: "HMIBIMH7",
    title: "Language models are few-shot learners",
    authors: "Brown et al.",
    year: "2020",
    marksCount: 0,
    attachmentKey: "BROWN01",
    doi: null,
    zotero_key: "HMIBIMH7",
  },
];

/** In-memory Rpc for tests and for running the UI without the shell. */
export function createMockRpc(
  marks: Mark[] = [],
  options: MockRpcOptions = {},
): Rpc {
  const history: HistoryEvent[] = [];
  let currentWorkspace: string | null = options.initialWorkspace !== undefined ? options.initialWorkspace : "/mock/workspace";
  const papers = options.papers ?? FIXTURE_PAPERS;

  return {
    chooseFolder: () => {
      const selected = options.chosenFolder !== undefined ? options.chosenFolder : "/home/user/research";
      currentWorkspace = selected;
      return Promise.resolve(selected ? { path: selected } : null);
    },
    getWorkspace: () => {
      return Promise.resolve(currentWorkspace ? { path: currentWorkspace } : null);
    },
    connectZotero: () => {
      if (options.failZoteroCode) {
        return Promise.reject(
          new ZoteroError(
            options.failZoteroCode,
            options.zoteroDetail,
            options.zoteroSeconds,
          ),
        );
      }
      return Promise.resolve([...papers]);
    },
    importMarks: (_paperId, _attachmentKey) => {
      if (!currentWorkspace) {
        return Promise.reject(new Error("No project folder is open."));
      }
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
      return Promise.resolve({ imported: marks.length, milestone: marks.length ? "mock-milestone" : null });
    },
    readHistory: () => Promise.resolve([...history]),
    paper_link: ({ paperId, target }: PaperLinkRequest) => {
      const paper = papers.find((item) => item.id === paperId);
      if (!paper) return Promise.reject(new ZoteroError("internal"));
      if (target === "doi") {
        if (!paper.doi) return Promise.reject(new ZoteroError("no_doi"));
        return Promise.resolve({ uri: `https://doi.org/${paper.doi}` });
      }
      if (!paper.zotero_key) return Promise.reject(new ZoteroError("no_zotero_key"));
      if (target === "pdf" && !paper.attachmentKey) {
        return Promise.reject(new ZoteroError("no_zotero_key"));
      }
      return Promise.resolve({
        uri: target === "pdf"
          ? `zotero://open-pdf/library/items/${paper.attachmentKey}`
          : `zotero://select/library/items/${paper.zotero_key}`,
      });
    },
    open_link: () => Promise.resolve(),
  };
}
