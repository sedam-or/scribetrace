import { useEffect, useState } from "react";

import type { Rpc } from "../rpc/rpc.ts";
import { ZoteroError, type DiffLine, type Mark, type Milestone, type MilestoneDiff, type MilestoneReason } from "../rpc/types.ts";
import { t, type StringKey } from "../strings.ts";
import { Button } from "./Button.tsx";
import { EmptyState } from "./EmptyState.tsx";
import { ErrorBanner } from "./ErrorBanner.tsx";

type TimelineState =
  | { kind: "loading" }
  | { kind: "error"; code: string }
  | { kind: "ready"; milestones: Milestone[]; loadMs: number };

type DiffState =
  | { kind: "idle" }
  | { kind: "loading" }
  | { kind: "error"; code: string }
  | { kind: "ready"; diff: MilestoneDiff; fromId: string; toId: string };

const REASON_KEYS: Record<MilestoneReason, StringKey> = {
  import: "history.reason.import",
  refresh: "history.reason.refresh",
  session: "history.reason.session",
  manual: "history.reason.manual",
  before_restore: "history.reason.before_restore",
};

function getMilestoneName(milestone: Milestone): string {
  return milestone.label ?? t("history.milestone.unnamed");
}

function formatMilestoneTime(timestamp: string): string {
  const date = new Date(timestamp);
  if (Number.isNaN(date.getTime())) return timestamp;
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: "medium",
    timeStyle: "short",
  }).format(date);
}

function MarkText({ mark }: { mark: Mark }) {
  return (
    <div className="st-mark-diff-text">
      <blockquote>{mark.text ?? mark.kind}</blockquote>
      {mark.comment && <p>{mark.comment}</p>}
    </div>
  );
}

export function DiffLines({ lines }: { lines: DiffLine[] }) {
  return (
    <ul className="st-note-diff">
      {lines.map((line, index) => (
        <li className={`st-note-diff-${line.op}`} key={`${line.op}-${index}`}>
          <span aria-hidden="true">{line.op === "add" ? "+" : line.op === "remove" ? "−" : " "}</span> {line.text}
        </li>
      ))}
    </ul>
  );
}

export interface MilestoneTimelineListProps {
  milestones: Milestone[];
  activeMilestoneId: string | null;
  fromId: string;
  toId: string;
  onOpenCompare: (index: number) => void;
  onFromChange: (id: string) => void;
  onToChange: (id: string) => void;
  onCompare: () => void;
}

export function MilestoneTimelineList({
  milestones,
  activeMilestoneId,
  fromId,
  toId,
  onOpenCompare,
  onFromChange,
  onToChange,
  onCompare,
}: MilestoneTimelineListProps) {
  return (
    <ol className="st-timeline">
      {milestones.map((milestone, index) => (
        <li className={`st-timeline-item ${activeMilestoneId === milestone.id ? "active" : ""}`.trim()} key={milestone.id}>
          <div className="st-timeline-track" aria-hidden="true">
            <div className="st-timeline-node" />
            <div className="st-timeline-rule" />
          </div>
          <div className="st-timeline-content">
            <div className="st-timeline-header">
              <span className="st-timeline-title">{getMilestoneName(milestone)}</span>
              <Button
                size="small"
                variant="ghost"
                aria-expanded={activeMilestoneId === milestone.id}
                onClick={() => onOpenCompare(index)}
              >
                {t("history.timeline.compare")}
              </Button>
            </div>
            <div className="st-timeline-meta">
              {t(REASON_KEYS[milestone.reason])} · {formatMilestoneTime(milestone.ts)}
            </div>

            {activeMilestoneId === milestone.id && (
              <div className="st-milestone-picker" role="group" aria-label={t("history.timeline.compare")}>
                <p>{t("history.changes.pick")}</p>
                <label>
                  <span>{t("history.timeline.from")}</span>
                  <select value={fromId} onChange={(event) => onFromChange(event.currentTarget.value)}>
                    {milestones.map((option) => (
                      <option value={option.id} key={option.id}>{getMilestoneName(option)} · {formatMilestoneTime(option.ts)}</option>
                    ))}
                  </select>
                </label>
                <label>
                  <span>{t("history.timeline.to")}</span>
                  <select value={toId} onChange={(event) => onToChange(event.currentTarget.value)}>
                    {milestones.map((option) => (
                      <option value={option.id} key={option.id}>{getMilestoneName(option)} · {formatMilestoneTime(option.ts)}</option>
                    ))}
                  </select>
                </label>
                <Button size="small" variant="primary" onClick={onCompare}>
                  {t("history.timeline.compare")}
                </Button>
                {fromId === toId && <p role="alert">{t("history.changes.same")}</p>}
              </div>
            )}
          </div>
        </li>
      ))}
    </ol>
  );
}

export interface MilestoneTimelineProps {
  rpc: Rpc;
  paperId: string;
}

/** Loads and compares milestones through the typed RPC. All mark text is rendered inert. */
export function MilestoneTimeline({ rpc, paperId }: MilestoneTimelineProps) {
  const [timelineState, setTimelineState] = useState<TimelineState>({ kind: "loading" });
  const [activeMilestoneId, setActiveMilestoneId] = useState<string | null>(null);
  const [fromId, setFromId] = useState("");
  const [toId, setToId] = useState("");
  const [diffState, setDiffState] = useState<DiffState>({ kind: "idle" });

  useEffect(() => {
    let cancelled = false;
    const startedAt = performance.now();
    setTimelineState({ kind: "loading" });
    if (!rpc.list_milestones) {
      setTimelineState({ kind: "error", code: "internal" });
      return () => { cancelled = true; };
    }

    void rpc.list_milestones(paperId).then(
      (milestones) => {
        if (!cancelled) setTimelineState({ kind: "ready", milestones, loadMs: performance.now() - startedAt });
      },
      (error: unknown) => {
        const code = error instanceof ZoteroError ? error.code : "internal";
        if (!cancelled) setTimelineState({ kind: "error", code });
      },
    );

    return () => { cancelled = true; };
  }, [paperId, rpc]);

  const openCompare = (milestones: Milestone[], index: number) => {
    const current = milestones[index];
    if (!current) return;
    const previous = index > 0 ? milestones[index - 1] : current;
    setActiveMilestoneId(current.id);
    setFromId(previous?.id ?? current.id);
    setToId(current.id);
    setDiffState({ kind: "idle" });
  };

  const compareMilestones = async () => {
    if (fromId === toId) {
      setDiffState({ kind: "error", code: "same" });
      return;
    }
    if (!rpc.milestone_diff) {
      setDiffState({ kind: "error", code: "internal" });
      return;
    }
    setDiffState({ kind: "loading" });
    try {
      const diff = await rpc.milestone_diff(paperId, fromId, toId);
      setDiffState({ kind: "ready", diff, fromId, toId });
    } catch (error) {
      const code = error instanceof ZoteroError ? error.code : "internal";
      setDiffState({ kind: "error", code });
    }
  };

  const updateFromMilestone = (id: string) => {
    setFromId(id);
    setDiffState({ kind: "idle" });
  };

  const updateToMilestone = (id: string) => {
    setToId(id);
    setDiffState({ kind: "idle" });
  };

  if (timelineState.kind === "loading") {
    return <p role="status">{t("history.timeline.loading")}</p>;
  }

  if (timelineState.kind === "error") {
    return (
      <ErrorBanner
        title={t("paper.error.title")}
        body={t("error.generic", { detail: timelineState.code })}
      />
    );
  }

  if (timelineState.milestones.length === 0) {
    return <EmptyState title={t("history.empty.title")} body={t("history.empty.body")} />;
  }

  const diffErrorBody = diffState.kind === "error" && diffState.code === "milestone_missing"
    ? t("history.changes.missing")
    : null;

  return (
    <section
      className="st-milestone-timeline"
      aria-labelledby="milestone-timeline-title"
      data-testid="milestone-timeline"
      data-load-ms={Math.round(timelineState.loadMs)}
    >
      <h2 id="milestone-timeline-title">{t("history.timeline.title")}</h2>
      <MilestoneTimelineList
        milestones={timelineState.milestones}
        activeMilestoneId={activeMilestoneId}
        fromId={fromId}
        toId={toId}
        onOpenCompare={(index) => openCompare(timelineState.milestones, index)}
        onFromChange={updateFromMilestone}
        onToChange={updateToMilestone}
        onCompare={() => void compareMilestones()}
      />

      {diffState.kind === "loading" && <p role="status">{t("history.timeline.loading")}</p>}
      {diffState.kind === "error" && diffState.code === "same" && (
        <p role="alert">{t("history.changes.same")}</p>
      )}
      {diffState.kind === "error" && diffState.code !== "same" && (
        <ErrorBanner
          title={t("paper.error.title")}
          body={diffErrorBody ?? t("error.generic", { detail: diffState.code })}
        />
      )}
      {diffState.kind === "ready" && (
        <section className="st-milestone-diff" aria-labelledby="milestone-diff-title" data-testid="milestone-diff">
          <h3 id="milestone-diff-title">
            {t("history.changes.title", {
              from: getMilestoneName(timelineState.milestones.find(({ id }) => id === diffState.fromId)!),
              to: getMilestoneName(timelineState.milestones.find(({ id }) => id === diffState.toId)!),
            })}
          </h3>
          {diffState.diff.added.length + diffState.diff.removed.length + diffState.diff.edited.length === 0
          && !diffState.diff.noteDiff.some((line) => line.op !== "equal") ? (
            <p>{t("history.changes.none")}</p>
          ) : (
            <div className="st-milestone-diff-groups">
              {diffState.diff.noteDiff.length > 0 && (
                <section>
                  <h4>{t("history.changes.paper_note")}</h4>
                  <DiffLines lines={diffState.diff.noteDiff} />
                </section>
              )}
              {diffState.diff.added.length > 0 && (
                <section>
                  <h4>{t("history.changes.added", { count: diffState.diff.added.length })}</h4>
                  <ul>{diffState.diff.added.map((mark) => <li key={mark.key}><MarkText mark={mark} /></li>)}</ul>
                </section>
              )}
              {diffState.diff.removed.length > 0 && (
                <section>
                  <h4>{t("history.changes.removed", { count: diffState.diff.removed.length })}</h4>
                  <ul>{diffState.diff.removed.map((mark) => <li key={mark.key}><MarkText mark={mark} /></li>)}</ul>
                </section>
              )}
              {diffState.diff.edited.length > 0 && (
                <section>
                  <h4>{t("history.changes.edited", { count: diffState.diff.edited.length })}</h4>
                  <ul>
                    {diffState.diff.edited.map((edit) => (
                      <li key={edit.key} className="st-edited-mark">
                        <div><strong>{t("history.diff.before")}</strong><MarkText mark={edit.before} /></div>
                        <div><strong>{t("history.diff.after")}</strong><MarkText mark={edit.after} /></div>
                        {edit.noteDiff.length > 0 && (
                          <DiffLines lines={edit.noteDiff} />
                        )}
                      </li>
                    ))}
                  </ul>
                </section>
              )}
            </div>
          )}
        </section>
      )}
    </section>
  );
}
