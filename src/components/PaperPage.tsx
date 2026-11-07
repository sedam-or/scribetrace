import { useEffect, useRef, useState } from "react";

import type { PaperItem, PaperLinkKind } from "../rpc/types.ts";
import { t } from "../strings.ts";
import { Button } from "./Button.tsx";
import { ErrorBanner } from "./ErrorBanner.tsx";

export interface PaperPageError {
  title: string;
  body: string;
}

interface PendingLink {
  kind: PaperLinkKind;
  uri: string;
}

export interface PaperPageProps {
  paper: PaperItem;
  onPaperLink: (paperId: string, kind: PaperLinkKind) => Promise<string>;
  onOpenLink: (paperId: string, kind: PaperLinkKind) => Promise<void>;
  onShowHistory: (paperId: string) => void;
  error?: PaperPageError | null;
}

export function PaperPage({ paper, onPaperLink, onOpenLink, onShowHistory, error }: PaperPageProps) {
  const [pendingLink, setPendingLink] = useState<PendingLink | null>(null);
  const [opening, setOpening] = useState(false);
  const dialogRef = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    const dialog = dialogRef.current;
    if (pendingLink && dialog && !dialog.open) dialog.showModal();
    if (!pendingLink && dialog?.open) dialog.close();
  }, [pendingLink]);

  const requestLink = async (kind: PaperLinkKind) => {
    try {
      const uri = await onPaperLink(paper.id, kind);
      setPendingLink({ kind, uri });
    } catch {
      // The parent maps typed core/Tauri codes into the ErrorBanner copy.
    }
  };

  const confirmLink = async () => {
    if (!pendingLink) return;
    setOpening(true);
    try {
      await onOpenLink(paper.id, pendingLink.kind);
      setPendingLink(null);
    } catch {
      setPendingLink(null);
    } finally {
      setOpening(false);
    }
  };

  const doiReason = !paper.doi ? t("paper.missing.no_doi") : null;
  const pdfReason = !paper.attachmentKey ? t("paper.missing.no_pdf") : null;
  const zoteroReason = !paper.zotero_key ? t("paper.missing.no_zotero_key") : null;

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
            <dd>{paper.doi ?? t("paper.no_doi")}</dd>
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
            disabled={!paper.doi}
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
            disabled={!paper.attachmentKey}
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
            disabled={!paper.zotero_key}
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

      <dialog
        ref={dialogRef}
        className="st-link-dialog"
        aria-labelledby="link-confirm-title"
        onCancel={(event) => {
          event.preventDefault();
          setPendingLink(null);
        }}
      >
        <h2 id="link-confirm-title">{t("link.confirm.title")}</h2>
        <p>{t("link.confirm.body")}</p>
        <output className="st-link-uri">{pendingLink?.uri}</output>
        <div className="st-link-dialog-actions">
          <Button variant="secondary" disabled={opening} onClick={() => setPendingLink(null)}>
            {t("link.confirm.cancel")}
          </Button>
          <Button variant="primary" disabled={opening} onClick={() => void confirmLink()}>
            {t("link.confirm.open")}
          </Button>
        </div>
      </dialog>
    </article>
  );
}
