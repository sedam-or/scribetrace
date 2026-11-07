import assert from "node:assert/strict";
import { test } from "node:test";
import { renderToStaticMarkup } from "react-dom/server";

import { App } from "../src/App.tsx";
import { SetupView } from "../src/components/SetupView.tsx";
import { createMockRpc, FIXTURE_PAPERS } from "../src/rpc/mock.ts";
import type { PaperItem, ZoteroErrorCode } from "../src/rpc/types.ts";
import {
  formatZoteroError,
  getConnectionStatusText,
  setupReducer,
  type SetupState,
} from "../src/setupMachine.ts";
import { t } from "../src/strings.ts";

test("setup machine: 3 tested transitions (R-05, Slice 5)", () => {
  // Initial state: unconfigured
  const s0: SetupState = { kind: "unconfigured" };
  assert.equal(getConnectionStatusText(s0), "Not connected yet.");

  // Transition 1: Unconfigured -> Folder chosen
  const s1 = setupReducer(s0, {
    type: "CHOOSE_FOLDER",
    workspacePath: "/home/research/quantum",
  });
  assert.equal(s1.kind, "folder_chosen");
  if (s1.kind === "folder_chosen") {
    assert.equal(s1.workspacePath, "/home/research/quantum");
  }
  assert.equal(getConnectionStatusText(s1), "Not connected yet.");

  // Transition 2: Folder chosen -> Connecting -> Connected
  const s2Connecting = setupReducer(s1, { type: "CONNECT_START" });
  assert.equal(s2Connecting.kind, "connecting");
  assert.equal(getConnectionStatusText(s2Connecting), "Contacting Zotero on this computer.");

  const s2Connected = setupReducer(s2Connecting, {
    type: "CONNECT_SUCCESS",
    papers: FIXTURE_PAPERS,
  });
  assert.equal(s2Connected.kind, "connected");
  if (s2Connected.kind === "connected") {
    assert.equal(s2Connected.papers.length, 3);
    assert.equal(s2Connected.workspacePath, "/home/research/quantum");
  }
  assert.equal(getConnectionStatusText(s2Connected), "Connected, 3 papers.");

  // Connected with 1 paper displays singular string
  const singlePaperState: SetupState = {
    kind: "connected",
    workspacePath: "/home/research/quantum",
    papers: [FIXTURE_PAPERS[0] as PaperItem],
  };
  assert.equal(getConnectionStatusText(singlePaperState), "Connected, 1 paper.");

  // Transition 3: Folder chosen -> Connecting -> Error
  const s3Connecting = setupReducer(s1, { type: "CONNECT_START" });
  const s3Error = setupReducer(s3Connecting, {
    type: "CONNECT_ERROR",
    errorCode: "forbidden",
  });
  assert.equal(s3Error.kind, "error");
  if (s3Error.kind === "error") {
    assert.equal(s3Error.errorCode, "forbidden");
  }
  assert.equal(getConnectionStatusText(s3Error), "Zotero said no");

  // Recovery: Back to setup returns to folder_chosen
  const s3Recovered = setupReducer(s3Error, { type: "BACK_TO_SETUP" });
  assert.equal(s3Recovered.kind, "folder_chosen");

  // Reset returns to unconfigured
  const sReset = setupReducer(s3Recovered, { type: "RESET" });
  assert.equal(sReset.kind, "unconfigured");
});

test("Zotero error mapping asserts verbatim copy and setting name (R-04)", () => {
  const settingName = "Allow other applications on this computer to communicate with Zotero";

  // 1. Refused (Zotero not running)
  const errRefused = formatZoteroError("refused");
  assert.equal(errRefused.title, "Zotero cannot be reached");
  assert.equal(
    errRefused.body,
    "ScribeTrace could not reach Zotero on this computer. Start Zotero and keep it open. Then press Check again.",
  );
  assert.equal(errRefused.notice, "ScribeTrace does not retry on its own.");
  assert.equal(errRefused.retryAction, "Check again");
  assert.equal(errRefused.backAction, "Back to setup");

  // 2. Forbidden (HTTP 403, local access setting is off)
  const errForbidden = formatZoteroError("forbidden");
  assert.equal(errForbidden.title, "Zotero said no");
  assert.equal(
    errForbidden.body,
    `Zotero is running but its access setting is off. In Zotero, open Settings, then Advanced, and tick "${settingName}". Then press Check again.`,
  );
  assert.ok(errForbidden.body.includes(settingName));
  assert.equal(errForbidden.notice, "ScribeTrace does not retry on its own.");

  // 3. Timeout (> 5s)
  const errTimeout = formatZoteroError("timeout", { seconds: 5 });
  assert.equal(errTimeout.title, "Zotero did not answer");
  assert.equal(
    errTimeout.body,
    "Zotero did not answer within 5 seconds. Close any open Zotero dialog, then press Check again.",
  );

  // 4. Version (unsupported API version)
  const errVersion = formatZoteroError("version");
  assert.equal(errVersion.title, "This Zotero version is not supported yet");
  assert.equal(
    errVersion.body,
    "Zotero answered in a format ScribeTrace does not know. Nothing was imported and nothing was changed. Please report your Zotero version at github.com/sedam-or/scribetrace/issues.",
  );

  // 5. Too large (payload > 15 MB)
  const errTooLarge = formatZoteroError("too_large");
  assert.equal(errTooLarge.title, "That answer was too large");
  assert.equal(
    errTooLarge.body,
    "Zotero sent more data than ScribeTrace accepts in one go. Nothing was imported.",
  );

  // 6. Bad answer (malformed JSON or unexpected non-200)
  const errBadAnswer = formatZoteroError("bad_answer");
  assert.equal(errBadAnswer.title, "Unexpected answer from Zotero");
  assert.equal(
    errBadAnswer.body,
    "Zotero answered in a way ScribeTrace did not expect. Nothing was imported. Make sure Zotero is running normally, then press Check again.",
  );

  // 7. Internal (storage / IO error)
  const errInternal = formatZoteroError("internal", { detail: "Disk full on /home" });
  assert.equal(errInternal.title, "ScribeTrace could not use its history files");
  assert.equal(
    errInternal.body,
    "ScribeTrace could not read or write the .scribetrace folder. Check that the folder still exists and that you can write to it, then try again. Details: Disk full on /home",
  );

  // 8. Unknown error code cleanly falls back to error.generic without throwing
  const errUnknown = formatZoteroError("unknown_custom_code_999", {
    detail: "Something odd happened",
  });
  assert.equal(errUnknown.title, "Something went wrong. Nothing was changed. Details: unknown_custom_code_999");
  assert.equal(errUnknown.body, "Something odd happened");
  assert.equal(errUnknown.notice, "ScribeTrace does not retry on its own.");
});

test("SetupView renders 3 numbered step cards and WebDriver test-ids", () => {
  const html = renderToStaticMarkup(
    <SetupView
      state={{ kind: "folder_chosen", workspacePath: "/path/to/my-project" }}
      onChooseFolder={() => {}}
      onConnect={() => {}}
      onBackToSetup={() => {}}
    />,
  );

  assert.match(html, /data-testid="setup-view"/);
  assert.match(html, /data-testid="setup-step-1"/);
  assert.match(html, /data-testid="setup-step-2"/);
  assert.match(html, /data-testid="setup-step-3"/);
  assert.match(html, /data-testid="choose-folder-btn"/);
  assert.match(html, /data-testid="connect-btn"/);
  assert.match(html, /data-testid="connection-status"/);
  assert.match(html, /\/path\/to\/my-project/);
  assert.match(html, /Set up ScribeTrace in three steps/);
  assert.match(html, /Allow other applications on this computer to communicate with Zotero/);
});

test("SetupView error state renders alert role and resolution buttons", () => {
  const html = renderToStaticMarkup(
    <SetupView
      state={{
        kind: "error",
        workspacePath: "/path/to/my-project",
        errorCode: "forbidden",
      }}
      onChooseFolder={() => {}}
      onConnect={() => {}}
      onBackToSetup={() => {}}
    />,
  );

  assert.match(html, /role="alert"/);
  assert.match(html, /data-testid="error-banner"/);
  assert.match(html, /Zotero said no/);
  assert.match(html, /ScribeTrace does not retry on its own\./);
  assert.match(html, /Check again/);
  assert.match(html, /data-testid="back-to-setup-btn"/);
});

test("App renders connected papers and marks zero-mark paper with papers.no_marks", () => {
  const rpc = createMockRpc([], {
    initialWorkspace: "/home/research",
    papers: FIXTURE_PAPERS,
  });

  const html = renderToStaticMarkup(
    <App
      rpc={rpc}
      initialSetupState={{
        kind: "connected",
        workspacePath: "/home/research",
        papers: FIXTURE_PAPERS,
      }}
    />,
  );

  // Assert 'Connected, N papers' is rendered
  assert.match(html, /data-testid="connected-status"/);
  assert.match(html, /Connected, 3 papers\./);

  // Assert papers list rendered
  assert.match(html, /data-testid="paper-list"/);
  assert.match(html, /Attention is all you need/);
  assert.match(html, /data-testid="paper-page"/);
  assert.match(html, /10\.48550\/arXiv\.1706\.03762/);
  assert.match(html, /3 marks in history/);
  assert.match(html, /Deep residual learning for image recognition/);
  assert.match(html, /Language models are few-shot learners/);

  // Assert zero-marks paper has verbatim 'No marks in Zotero yet'
  assert.match(html, /No marks in Zotero yet/);
  assert.match(html, /title="This paper has no highlights or notes in Zotero, so there is nothing to import\."/);
});

test("App empty library renders papers.empty copy", () => {
  const rpc = createMockRpc([], {
    initialWorkspace: "/home/research",
    papers: [],
  });

  const html = renderToStaticMarkup(
    <App
      rpc={rpc}
      initialSetupState={{
        kind: "connected",
        workspacePath: "/home/research",
        papers: [],
      }}
    />,
  );

  assert.match(html, /No papers imported yet/);
  assert.match(html, /Pick a paper from your Zotero library and import its marks\./);
  assert.match(html, /Show my Zotero papers/);
});
