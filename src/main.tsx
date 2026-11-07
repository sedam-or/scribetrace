import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { App } from "./App.tsx";
import { tauriRpc } from "./rpc/tauri-adapter.ts";
import { createMockRpc, FIXTURE_PAPERS } from "./rpc/mock.ts";
import "./main.css";

const root = document.getElementById("root");
if (root === null) throw new Error("missing #root");

const mockPreview = import.meta.env.DEV && new URLSearchParams(window.location.search).get("mock") === "1";
const mockPapers = FIXTURE_PAPERS;

createRoot(root).render(
  <StrictMode>
    <App
      rpc={mockPreview ? createMockRpc() : tauriRpc}
      initialSetupState={mockPreview
        ? { kind: "connected", workspacePath: "/mock/workspace", papers: mockPapers }
        : undefined}
    />
  </StrictMode>,
);
