import { EmptyState } from "./components/EmptyState.tsx";
import { ErrorBanner } from "./components/ErrorBanner.tsx";
import type { ViewState } from "./model.ts";
import { t } from "./strings.ts";

/** Pure view: everything it shows comes from `state`. Text is rendered as React text, never HTML. */
export function HistoryPanel({ state }: { state: ViewState }) {
  switch (state.kind) {
    case "idle":
      return (
        <EmptyState
          title={t("history.empty.title")}
          body={t("history.empty.body")}
        />
      );
    case "busy":
      return <p role="status">{t("history.events.loading")}</p>;
    case "error":
      return (
        <ErrorBanner
          title={t("paper.error.title")}
          body={t("error.generic", { detail: state.message })}
        />
      );
    case "done":
      return (
        <section aria-labelledby="history-events-title" data-testid="history-events">
          <h2 id="history-events-title">{t("history.events.title")}</h2>
          <ol className="st-raw-event-list">
            {state.events.map((event) => (
              <li key={event.id}>
                <pre>{JSON.stringify(event)}</pre>
              </li>
            ))}
          </ol>
        </section>
      );
  }
}
