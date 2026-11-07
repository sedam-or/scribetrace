import { useState } from "react";

import type { ExternalOpenOutcome, PaperItem, PaperLinkKind } from "../rpc/types.ts";
import { t } from "../strings.ts";
import { Button } from "./Button.tsx";
import { ErrorBanner } from "./ErrorBanner.tsx";

export interface PaperPageError {
  title: string;
  body: string;
}

export interface PaperPageProps {
  paper: PaperItem;
  onOpenExternal: (paperId: string, kind: PaperLinkKind) => Promise<ExternalOpenOutcome>;
  onShowHistory: (paperId: string) => void;
  error?: PaperPageError | null;
}

export function PaperPage({ paper, onOpenExternal, onShowHistory, error }: PaperPageProps) {
  const [opening, setOpening] = useState(false);

  const requestLink = async (kind: PaperLinkKind) => {
    setOpening(true);
    try {
      await onOpenExternal(paper.id, kind);
    } catch {
      // The parent maps typed core/Tauri codes into the ErrorBanner copy.
    } finally {
      setOpening(false);
    }
  };

  const doiReason = paper.linkAvailability.doi === "no_doi" ? t("paper.missing.no_doi") : null;
  const pdfReason = paper.linkAvailability.pdf === "no_pdf"
    ? t("paper.missing.no_pdf")
    : paper.linkAvailability.pdf === "no_zotero_key"
      ? t("paper.missing.no_zotero_key")
      : null;
  const zoteroReason = paper.linkAvailability.item === "no_zotero_key" ? t("paper.missing.no_zotero_key") : null;

  return (
    <article className="st-paper-page" data-testid="paper-page">
      <header className="st-paper-header">
        <h1 id="paper-page-title">{paper.title}</h1>
        <dl className="st-paper-meta">
          {paper.authors && (
            <div>
              <dt>{t("paper.authors")}</dt>
              <dd>{paper.authors}</dd>
            </div>
          )}
          <div>
            <dt>{t("paper.doi")}</dt>
            <dd>{paper.doiDisplay ?? t("paper.no_doi")}</dd>
          </div>
        </dl>
        <p className="st-paper-marks-count">
          {paper.marksCount === 1
            ? t("paper.marks_count.one")
            : t("paper.marks_count", { count: paper.marksCount })}
        </p>
      </header>

      {error && <ErrorBanner title={error.title} body={error.body} />}

      <section className="st-paper-actions" aria-labelledby="paper-page-title">
        <div className="st-paper-action">
          <Button
            variant="primary"
            disabled={opening || paper.linkAvailability.doi !== "available"}
            aria-describedby={doiReason ? `${paper.id}-doi-unavailable` : undefined}
            onClick={() => void requestLink("doi")}
          >
            {t("paper.open_doi")}
          </Button>
          {doiReason && <p id={`${paper.id}-doi-unavailable`} className="st-paper-disabled-reason">{doiReason}</p>}
        </div>
        <div className="st-paper-action">
          <Button
            variant="secondary"
            disabled={opening || paper.linkAvailability.pdf !== "available"}
            aria-describedby={pdfReason ? `${paper.id}-pdf-unavailable` : undefined}
            onClick={() => void requestLink("zotero_pdf")}
          >
            {t("paper.open_pdf_zotero")}
          </Button>
          {pdfReason && <p id={`${paper.id}-pdf-unavailable`} className="st-paper-disabled-reason">{pdfReason}</p>}
        </div>
        <div className="st-paper-action">
          <Button
            variant="secondary"
            disabled={opening || paper.linkAvailability.item !== "available"}
            aria-describedby={zoteroReason ? `${paper.id}-zotero-unavailable` : undefined}
            onClick={() => void requestLink("zotero_select")}
          >
            {t("paper.show_zotero")}
          </Button>
          {zoteroReason && <p id={`${paper.id}-zotero-unavailable`} className="st-paper-disabled-reason">{zoteroReason}</p>}
        </div>
      </section>

      <Button variant="ghost" onClick={() => onShowHistory(paper.id)}>
        {t("history.timeline.open")}
      </Button>

    </article>
  );
}
