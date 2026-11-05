import { Button } from "./Button.tsx";
import { ErrorBanner } from "./ErrorBanner.tsx";
import {
  formatZoteroError,
  getConnectionStatusText,
  type SetupState,
} from "../setupMachine.ts";
import { t } from "../strings.ts";

export interface SetupViewProps {
  state: SetupState;
  onChooseFolder: () => void;
  onConnect: () => void;
  onBackToSetup?: () => void;
}

export function SetupView({
  state,
  onChooseFolder,
  onConnect,
  onBackToSetup,
}: SetupViewProps) {
  const isConnecting = state.kind === "connecting";
  const hasFolder = state.kind !== "unconfigured";
  const folderPath =
    state.kind !== "unconfigured" ? state.workspacePath : "";

  const statusText = getConnectionStatusText(state);
  const errorInfo =
    state.kind === "error"
      ? formatZoteroError(state.errorCode, {
          detail: state.detail,
          seconds: state.seconds,
        })
      : null;

  return (
    <div className="st-setup-view" data-testid="setup-view">
      <div className="st-setup-hero">
        <h1 className="st-setup-title">{t("setup.title")}</h1>
        <p className="st-setup-intro">{t("setup.intro")}</p>
      </div>

      {errorInfo && (
        <div data-testid="error-banner" className="st-setup-error-wrapper">
          <ErrorBanner
            title={errorInfo.title}
            body={errorInfo.body}
            notice={errorInfo.notice}
            actionLabel={errorInfo.retryAction}
            onAction={onConnect}
          />
          {onBackToSetup && (
            <div className="st-setup-error-actions">
              <Button
                variant="secondary"
                size="small"
                onClick={onBackToSetup}
                disabled={isConnecting}
                data-testid="back-to-setup-btn"
              >
                {errorInfo.backAction}
              </Button>
            </div>
          )}
        </div>
      )}

      <ol className="st-setup-steps" aria-label={t("setup.title")}>
        {/* Step 1: Zotero setting */}
        <li
          className="st-setup-step-card"
          data-testid="setup-step-1"
        >
          <div className="st-step-num" aria-hidden="true">
            1
          </div>
          <div className="st-step-body">
            <h2 className="st-step-title">{t("setup.step1.title")}</h2>
            <p className="st-step-desc">{t("setup.step1.body")}</p>
            <div className="st-step-instruction">
              {t("setup.step1.action", {
                zotero_setting: t("zotero.setting_name"),
              })}
            </div>
          </div>
        </li>

        {/* Step 2: Project folder */}
        <li
          className={`st-setup-step-card ${hasFolder ? "completed" : "active"}`}
          data-testid="setup-step-2"
        >
          <div className="st-step-num" aria-hidden="true">
            2
          </div>
          <div className="st-step-body">
            <h2 className="st-step-title">{t("setup.step2.title")}</h2>
            <p className="st-step-desc">{t("setup.step2.body")}</p>
            <div className="st-step-action-row">
              <Button
                variant={hasFolder ? "secondary" : "primary"}
                onClick={onChooseFolder}
                disabled={isConnecting}
                data-testid="choose-folder-btn"
              >
                {t("setup.step2.action")}
              </Button>
              {folderPath && (
                <span className="st-step-path-preview" title={folderPath}>
                  {folderPath}
                </span>
              )}
            </div>
          </div>
        </li>

        {/* Step 3: Connect */}
        <li
          className={`st-setup-step-card ${state.kind === "connected" ? "completed" : hasFolder ? "active" : ""}`}
          data-testid="setup-step-3"
        >
          <div className="st-step-num" aria-hidden="true">
            3
          </div>
          <div className="st-step-body">
            <h2 className="st-step-title">{t("setup.step3.title")}</h2>
            <p className="st-step-desc">{t("setup.step3.body")}</p>
            <div className="st-step-action-row">
              <Button
                variant="primary"
                onClick={onConnect}
                disabled={!hasFolder || isConnecting}
                data-testid="connect-btn"
              >
                {t("setup.step3.action")}
              </Button>
              <span
                className="st-connection-status"
                data-testid="connection-status"
                role="status"
              >
                {statusText}
              </span>
            </div>
          </div>
        </li>
      </ol>
    </div>
  );
}
