import type { PaperLinkKind, PaperLinkRequest } from "./types.ts";
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
    getUri: rpc.paper_link
      ? (paperId: string, action: PaperLinkKind) => rpc.paper_link!(request(paperId, action))
      : undefined,
    openExternal: (paperId: string, action: PaperLinkKind) => {
      return rpc.open_external(request(paperId, action));
    },
  };
}
