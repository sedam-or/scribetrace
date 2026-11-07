import { useEffect, useState } from "react";

import { AppShell } from "./components/AppShell.tsx";
import { Button } from "./components/Button.tsx";
import { EmptyState } from "./components/EmptyState.tsx";
import { PaperListRow } from "./components/PaperListRow.tsx";
import { SetupView } from "./components/SetupView.tsx";
import { HistoryPanel } from "./HistoryPanel.tsx";
import { PaperPage, type PaperPageError } from "./components/PaperPage.tsx";
import { MilestoneTimeline } from "./components/MilestoneTimeline.tsx";
import { importPaper, type ViewState } from "./model.ts";
import type { Rpc } from "./rpc/rpc.ts";
import { ZoteroError, type PaperLinkKind } from "./rpc/types.ts";
import { createPaperLinkAdapter } from "./rpc/paper-link-adapter.ts";
import { setupReducer, type SetupState } from "./setupMachine.ts";
import { t } from "./strings.ts";

function getPaperLinkError(error: unknown): PaperPageError {
  const code = error instanceof ZoteroError ? error.code : "internal";
  const bodyKey = code === "no_doi"
    ? "paper.error.no_doi"
    : code === "no_zotero_key"
      ? "paper.error.no_zotero_key"
      : code === "no_pdf"
        ? "paper.missing.no_pdf"
      : code === "refused" || code === "zotero_not_running"
        ? "paper.error.zotero_not_running"
        : null;

  return {
    title: t("paper.error.title"),
    body: bodyKey ? t(bodyKey) : t("error.generic", { detail: code }),
  };
}

export interface AppProps {
  rpc: Rpc;
  initialSetupState?: SetupState;
}

export function App({ rpc, initialSetupState }: AppProps) {
  const paperLinks = createPaperLinkAdapter(rpc);
  const [setupState, setSetupState] = useState<SetupState>(
    initialSetupState ?? { kind: "unconfigured" },
  );
  const [selectedPaperId, setSelectedPaperId] = useState<string | null>(null);
  const [historyState, setHistoryState] = useState<ViewState>({ kind: "idle" });
  const [showHistory, setShowHistory] = useState(false);
  const [showAllEvents, setShowAllEvents] = useState(false);
  const [paperError, setPaperError] = useState<PaperPageError | null>(null);

  useEffect(() => {
    if (initialSetupState) return;
    void rpc.getWorkspace().then((ws) => {
      if (ws) {
        setSetupState((prev) =>
          prev.kind === "unconfigured"
            ? setupReducer(prev, { type: "CHOOSE_FOLDER", workspacePath: ws.path })
            : prev,
        );
      }
    });
  }, [rpc, initialSetupState]);

  const handleChooseFolder = async () => {
    try {
      const chosen = await rpc.chooseFolder();
      if (chosen) {
        setSetupState((prev) =>
          setupReducer(prev, { type: "CHOOSE_FOLDER", workspacePath: chosen.path }),
        );
      }
    } catch (e) {
      const msg = e instanceof Error ? e.message : String(e);
      setSetupState((prev) =>
        setupReducer(prev, { type: "CONNECT_ERROR", errorCode: "internal", detail: msg }),
      );
    }
  };

  const handleConnect = async () => {
    setSetupState((prev) => setupReducer(prev, { type: "CONNECT_START" }));
    try {
      const papers = await rpc.connectZotero();
      setSetupState((prev) =>
        setupReducer(prev, { type: "CONNECT_SUCCESS", papers }),
      );
      if (papers.length > 0 && !selectedPaperId) {
        setSelectedPaperId(papers[0]?.id ?? null);
      }
    } catch (e) {
      if (e instanceof ZoteroError) {
        setSetupState((prev) =>
          setupReducer(prev, {
            type: "CONNECT_ERROR",
            errorCode: e.code,
            detail: e.detail,
            seconds: e.seconds,
          }),
        );
      } else {
        const msg = e instanceof Error ? e.message : String(e);
        setSetupState((prev) =>
          setupReducer(prev, {
            type: "CONNECT_ERROR",
            errorCode: "refused",
            detail: msg,
          }),
        );
      }
    }
  };

  const handleBackToSetup = () => {
    setSetupState((prev) => setupReducer(prev, { type: "BACK_TO_SETUP" }));
  };

  const handleImport = (paperId: string) => {
    setHistoryState({ kind: "busy" });
    const folder = setupState.kind !== "unconfigured" ? setupState.workspacePath : undefined;
    void importPaper(rpc, folder, paperId).then(setHistoryState);
  };

  const handlePaperLink = async (paperId: string, kind: PaperLinkKind) => {
    setPaperError(null);
    try {
      if (!paperLinks.getUri) throw new ZoteroError("internal");
      const link = await paperLinks.getUri(paperId, kind);
      return link.uri;
    } catch (e) {
      setPaperError(getPaperLinkError(e));
      throw e;
    }
  };

  const handleOpenExternal = async (paperId: string, kind: PaperLinkKind) => {
    setPaperError(null);
    try {
      return await paperLinks.openExternal(paperId, kind);
    } catch (e) {
      setPaperError(getPaperLinkError(e));
      throw e;
    }
  };

  const handleShowHistory = (paperId: string) => {
    setSelectedPaperId(paperId);
    setShowHistory(true);
    setShowAllEvents(false);
  };

  const handleShowAllEvents = async (paperId: string) => {
    if (showAllEvents) {
      setShowAllEvents(false);
      return;
    }

    setShowAllEvents(true);
    setHistoryState({ kind: "busy" });
    try {
      const events = await rpc.readHistory(paperId);
      setHistoryState({ kind: "done", imported: 0, events });
    } catch (e) {
      setHistoryState({ kind: "error", message: e instanceof Error ? e.message : String(e) });
    }
  };

  const isConnected = setupState.kind === "connected";
  const papers = isConnected ? setupState.papers : [];
  const activePaperId = selectedPaperId ?? papers[0]?.id ?? null;
  const activePaper = papers.find((paper) => paper.id === activePaperId) ?? null;
  const importUnavailableReason = activePaper?.linkAvailability.pdf === "no_pdf"
    ? t("paper.missing.no_pdf")
    : activePaper?.linkAvailability.pdf === "no_zotero_key"
      ? t("paper.missing.no_zotero_key")
      : null;

  const navContent = isConnected ? (
    <>
      <div className="st-nav-status" data-testid="connected-status">
        {papers.length === 1
          ? t("setup.connected.one")
          : t("setup.connected", { count: papers.length })}
      </div>

      {papers.length === 0 ? (
        <EmptyState
          title={t("papers.empty.title")}
          body={t("papers.empty.body")}
          actionLabel={t("papers.empty.action")}
          onAction={handleConnect}
        />
      ) : (
        <div className="st-nav-list" role="list" aria-label={t("app.nav.papers")} data-testid="paper-list">
          {papers.map((paper) => {
            const hasMarks = paper.marksCount > 0;
            return (
              <PaperListRow
                key={paper.id}
                id={paper.id}
                title={paper.title}
                authors={paper.authors}
                year={paper.year}
                marksCount={paper.marksCount}
                statusText={!hasMarks ? t("papers.no_marks") : undefined}
                active={selectedPaperId === paper.id || (selectedPaperId === null && paper.id === papers[0]?.id)}
                onSelect={(id) => {
                  setSelectedPaperId(id);
                  setShowHistory(false);
                  setShowAllEvents(false);
                  setPaperError(null);
                }}
                onRefresh={
                  hasMarks
                    ? () => handleImport(paper.id)
                    : undefined
                }
              />
            );
          })}
        </div>
      )}
    </>
  ) : (
    <div className="st-nav-list" role="list" aria-label={t("app.nav.papers")}>
      <EmptyState
        title={t("setup.title")}
        body={t("setup.waiting")}
      />
    </div>
  );

  return (
    <AppShell navContent={navContent}>
      {isConnected ? (
        <div className="st-history-view">
          {activePaper && !showHistory ? (
            <>
              <div className="st-detail-toolbar">
                <Button
                  variant="primary"
                  size="small"
                  onClick={() => handleImport(activePaper.id)}
                  disabled={historyState.kind === "busy" || activePaper.linkAvailability.pdf !== "available"}
                  data-testid="import-marks-btn"
                >
                  {t("papers.import.action")}
                </Button>
                {importUnavailableReason && <p className="st-paper-disabled-reason">{importUnavailableReason}</p>}
              </div>
              <PaperPage
                paper={activePaper}
                onPaperLink={paperLinks.getUri ? handlePaperLink : undefined}
                onOpenExternal={handleOpenExternal}
                onShowHistory={handleShowHistory}
                error={paperError}
              />
            </>
          ) : showHistory && activePaper ? (
            <>
              <div className="st-detail-toolbar">
                <Button variant="ghost" size="small" onClick={() => setShowHistory(false)}>
                  {activePaper.title}
                </Button>
                <Button variant="secondary" size="small" onClick={() => void handleShowAllEvents(activePaper.id)}>
                  {showAllEvents ? t("history.timeline.open") : t("history.events.open")}
                </Button>
              </div>
              {showAllEvents
                ? <HistoryPanel state={historyState} />
                : <MilestoneTimeline rpc={rpc} paperId={activePaper.id} />}
            </>
          ) : null}
        </div>
      ) : (
        <SetupView
          state={setupState}
          onChooseFolder={handleChooseFolder}
          onConnect={handleConnect}
          onBackToSetup={handleBackToSetup}
        />
      )}
    </AppShell>
  );
}
