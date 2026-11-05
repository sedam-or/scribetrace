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
      return <p role="status">Importing…</p>;
    case "error":
      return (
        <ErrorBanner
          title="Error"
          body={state.message}
        />
      );
    case "done":
      return (
        <section aria-labelledby="history-title">
          <h2 id="history-title">History</h2>
          <p role="status">Imported {state.imported} mark(s).</p>
          <ol className="st-timeline">
            {state.events.map((e) => (
              <li key={e.id} className="st-timeline-item">
                <div className="st-timeline-track" aria-hidden="true">
                  <div className="st-timeline-node" />
                  <div className="st-timeline-rule" />
                </div>
                <div className="st-timeline-content">
                  <div className="meta">
                    {e.ts} · {e.author} · {e.type}
                  </div>
                  <blockquote>{e.payload.text ?? "(no text)"}</blockquote>
                  {e.payload.comment !== null && <p>{e.payload.comment}</p>}
                </div>
              </li>
            ))}
          </ol>
        </section>
      );
  }
}
