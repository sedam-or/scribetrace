import { useState } from "react";

import { AppShell } from "./components/AppShell.tsx";
import { Button } from "./components/Button.tsx";
import { FolderPicker } from "./components/FolderPicker.tsx";
import { PaperListRow } from "./components/PaperListRow.tsx";
import { HistoryPanel } from "./HistoryPanel.tsx";
import { importPaper, SLICE1_PAPER, type ViewState } from "./model.ts";
import type { Rpc } from "./rpc/rpc.ts";
import { t } from "./strings.ts";

export function App({ rpc }: { rpc: Rpc }) {
  const [folder, setFolder] = useState("");
  const [state, setState] = useState<ViewState>({ kind: "idle" });
  const [selectedPaper, setSelectedPaper] = useState<string>(SLICE1_PAPER.paperId);

  const onImport = () => {
    setState({ kind: "busy" });
    void importPaper(rpc, folder).then(setState);
  };

  const navContent = (
    <>
      <div className="st-nav-toolbar">
        <FolderPicker
          id="folder"
          label={t("setup.step2.title")}
          value={folder}
          onChange={setFolder}
          placeholder="/path/to/project"
          browseLabel={t("setup.step2.action")}
        />
        <Button
          variant="primary"
          onClick={onImport}
          disabled={folder === "" || state.kind === "busy"}
        >
          {t("papers.import.action")}
        </Button>
      </div>

      <div className="st-nav-list" role="list" aria-label="Paper list">
        <PaperListRow
          id={SLICE1_PAPER.paperId}
          title="Attention Is All You Need"
          authors="Vaswani et al."
          year="2017"
          marksCount={state.kind === "done" ? state.imported : 0}
          active={selectedPaper === SLICE1_PAPER.paperId}
          onSelect={(id) => setSelectedPaper(id)}
          onRefresh={folder !== "" ? () => onImport() : undefined}
        />
      </div>
    </>
  );

  return (
    <AppShell navContent={navContent}>
      <HistoryPanel state={state} />
    </AppShell>
  );
}
