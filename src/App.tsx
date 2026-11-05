import { useEffect, useState } from "react";

import { AppShell } from "./components/AppShell.tsx";
import { Button } from "./components/Button.tsx";
import { EmptyState } from "./components/EmptyState.tsx";
import { PaperListRow } from "./components/PaperListRow.tsx";
import { SetupView } from "./components/SetupView.tsx";
import { HistoryPanel } from "./HistoryPanel.tsx";
import { importPaper, type ViewState } from "./model.ts";
import type { Rpc } from "./rpc/rpc.ts";
import { ZoteroError } from "./rpc/types.ts";
import { setupReducer, type SetupState } from "./setupMachine.ts";
import { t } from "./strings.ts";

export interface AppProps {
  rpc: Rpc;
  initialSetupState?: SetupState;
}

export function App({ rpc, initialSetupState }: AppProps) {
  const [setupState, setSetupState] = useState<SetupState>(
    initialSetupState ?? { kind: "unconfigured" },
  );
  const [selectedPaperId, setSelectedPaperId] = useState<string | null>(null);
  const [historyState, setHistoryState] = useState<ViewState>({ kind: "idle" });

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

  const handleImport = (paperId: string, attachmentKey: string) => {
    setHistoryState({ kind: "busy" });
    const folder = setupState.kind !== "unconfigured" ? setupState.workspacePath : undefined;
    void importPaper(rpc, folder, paperId, attachmentKey).then(setHistoryState);
  };

  const isConnected = setupState.kind === "connected";
  const papers = isConnected ? setupState.papers : [];

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
                active={selectedPaperId === paper.id}
                onSelect={(id) => setSelectedPaperId(id)}
                onRefresh={
                  hasMarks
                    ? () => handleImport(paper.id, paper.attachmentKey ?? "")
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
          {selectedPaperId && (
            <div className="st-detail-toolbar">
              <Button
                variant="primary"
                size="small"
                onClick={() => {
                  const p = papers.find((x) => x.id === selectedPaperId);
                  if (p) handleImport(p.id, p.attachmentKey ?? "");
                }}
                disabled={historyState.kind === "busy"}
                data-testid="import-marks-btn"
              >
                {t("papers.import.action")}
              </Button>
            </div>
          )}
          <HistoryPanel state={historyState} />
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
