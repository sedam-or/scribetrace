import type { ViewState } from "./model.ts";

/** Pure view: everything it shows comes from `state`. Text is rendered as React text, never HTML. */
export function HistoryPanel({ state }: { state: ViewState }) {
  switch (state.kind) {
    case "idle":
      return <p>Choose a project folder, then press Import.</p>;
    case "busy":
      return <p role="status">Importing…</p>;
    case "error":
      return (
        <p role="alert" className="error">
          {state.message}
        </p>
      );
    case "done":
      return (
        <section aria-labelledby="history-title">
          <h2 id="history-title">History</h2>
          <p role="status">Imported {state.imported} mark(s).</p>
          <ol>
            {state.events.map((e) => (
              <li key={e.id}>
                <span className="meta">
                  {e.ts} · {e.author} · {e.type}
                </span>
                <blockquote>{e.payload.text ?? "(no text)"}</blockquote>
                {e.payload.comment !== null && <p>{e.payload.comment}</p>}
              </li>
            ))}
          </ol>
        </section>
      );
  }
}
