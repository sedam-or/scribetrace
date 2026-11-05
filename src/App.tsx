import { useState } from "react";

import { HistoryPanel } from "./HistoryPanel.tsx";
import { importPaper, type ViewState } from "./model.ts";
import type { Rpc } from "./rpc/rpc.ts";

export function App({ rpc }: { rpc: Rpc }) {
  const [folder, setFolder] = useState("");
  const [state, setState] = useState<ViewState>({ kind: "idle" });

  const onImport = () => {
    setState({ kind: "busy" });
    void importPaper(rpc, folder).then(setState);
  };

  return (
    <main>
      <h1>ScribeTrace</h1>
      <label htmlFor="folder">Project folder</label>
      <input id="folder" value={folder} onChange={(e) => setFolder(e.target.value)} />
      <button type="button" onClick={onImport} disabled={folder === "" || state.kind === "busy"}>
        Import marks
      </button>
      <HistoryPanel state={state} />
    </main>
  );
}
