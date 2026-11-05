import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { App } from "./App.tsx";
import { tauriRpc } from "./rpc/tauri-adapter.ts";
import "./main.css";

const root = document.getElementById("root");
if (root === null) throw new Error("missing #root");
createRoot(root).render(
  <StrictMode>
    <App rpc={tauriRpc} />
  </StrictMode>,
);
