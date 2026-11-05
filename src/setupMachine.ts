import type { PaperItem, ZoteroErrorCode } from "./rpc/types.ts";
import { t } from "./strings.ts";

export type SetupState =
  | { kind: "unconfigured" }
  | { kind: "folder_chosen"; workspacePath: string }
  | { kind: "connecting"; workspacePath: string }
  | { kind: "connected"; workspacePath: string; papers: PaperItem[] }
  | {
      kind: "error";
      workspacePath: string;
      errorCode: ZoteroErrorCode | string;
      detail?: string;
      seconds?: number;
    };

export type SetupAction =
  | { type: "CHOOSE_FOLDER"; workspacePath: string }
  | { type: "CONNECT_START" }
  | { type: "CONNECT_SUCCESS"; papers: PaperItem[] }
  | {
      type: "CONNECT_ERROR";
      errorCode: ZoteroErrorCode | string;
      detail?: string;
      seconds?: number;
    }
  | { type: "BACK_TO_SETUP" }
  | { type: "RESET" };

/** Pure state transition reducer for the 3-step setup flow. */
export function setupReducer(state: SetupState, action: SetupAction): SetupState {
  switch (action.type) {
    case "CHOOSE_FOLDER":
      return { kind: "folder_chosen", workspacePath: action.workspacePath };

    case "CONNECT_START":
      if (state.kind === "folder_chosen" || state.kind === "error" || state.kind === "connected") {
        return { kind: "connecting", workspacePath: state.workspacePath };
      }
      return state;

    case "CONNECT_SUCCESS":
      if (state.kind === "connecting") {
        return {
          kind: "connected",
          workspacePath: state.workspacePath,
          papers: action.papers,
        };
      }
      return state;

    case "CONNECT_ERROR":
      if (state.kind === "connecting") {
        return {
          kind: "error",
          workspacePath: state.workspacePath,
          errorCode: action.errorCode,
          detail: action.detail,
          seconds: action.seconds,
        };
      }
      return state;

    case "BACK_TO_SETUP":
      if (state.kind === "error" || state.kind === "connected") {
        return { kind: "folder_chosen", workspacePath: state.workspacePath };
      }
      return state;

    case "RESET":
      return { kind: "unconfigured" };
  }
}

export interface FormattedZoteroError {
  title: string;
  body: string;
  notice: string;
  retryAction: string;
  backAction: string;
}

/** Maps a text-free error code to localized copy in en.json. Never throws. */
export function formatZoteroError(
  code: ZoteroErrorCode | string,
  options?: { detail?: string; seconds?: number },
): FormattedZoteroError {
  const seconds = options?.seconds ?? 5;
  const detail = options?.detail ?? code;

  let title = "";
  let body = "";

  switch (code) {
    case "refused":
      title = t("zotero.error.refused.title");
      body = t("zotero.error.refused.body");
      break;

    case "forbidden":
      title = t("zotero.error.forbidden.title");
      body = t("zotero.error.forbidden.body", {
        zotero_setting: t("zotero.setting_name"),
      });
      break;

    case "timeout":
      title = t("zotero.error.timeout.title");
      body = t("zotero.error.timeout.body", { seconds });
      break;

    case "version":
      title = t("zotero.error.version.title");
      body = t("zotero.error.version.body");
      break;

    case "too_large":
      title = t("zotero.error.too_large.title");
      body = t("zotero.error.too_large.body");
      break;

    case "bad_answer":
      title = t("zotero.error.bad_answer.title");
      body = t("zotero.error.bad_answer.body");
      break;

    case "internal":
      title = t("zotero.error.internal.title");
      body = t("zotero.error.internal.body", { detail });
      break;

    default:
      title = t("error.generic", { detail: code });
      body = options?.detail ?? "";
      break;
  }

  return {
    title,
    body,
    notice: t("zotero.error.no_retry"),
    retryAction: t("zotero.action.check_again"),
    backAction: t("zotero.action.back_to_setup"),
  };
}

/** Formats the live connection status string according to current setup state. */
export function getConnectionStatusText(state: SetupState): string {
  switch (state.kind) {
    case "unconfigured":
    case "folder_chosen":
      return t("setup.waiting");

    case "connecting":
      return t("setup.contact_notice");

    case "connected":
      if (state.papers.length === 1) {
        return t("setup.connected.one");
      }
      return t("setup.connected", { count: state.papers.length });

    case "error":
      return formatZoteroError(state.errorCode, {
        detail: state.detail,
        seconds: state.seconds,
      }).title;
  }
}
