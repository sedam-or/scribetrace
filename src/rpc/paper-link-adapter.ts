import { ZoteroError, type PaperLinkKind, type PaperLinkRequest } from "./types.ts";
import type { Rpc } from "./rpc.ts";

const TARGET_BY_ACTION: Record<PaperLinkKind, PaperLinkRequest["target"]> = {
  doi: "doi",
  zotero_pdf: "pdf",
  zotero_select: "item",
};

/** Keep UI action names isolated from the backend's target/paperId command contract. */
export function createPaperLinkAdapter(rpc: Rpc) {
  const request = (paperId: string, action: PaperLinkKind): PaperLinkRequest => ({
    target: TARGET_BY_ACTION[action],
    paperId,
  });

  return {
    getUri: (paperId: string, action: PaperLinkKind) => {
      if (!rpc.paper_link) return Promise.reject(new ZoteroError("internal"));
      return rpc.paper_link(request(paperId, action));
    },
    open: (paperId: string, action: PaperLinkKind) => {
      if (!rpc.open_link) return Promise.reject(new ZoteroError("internal"));
      return rpc.open_link(request(paperId, action));
    },
  };
}
