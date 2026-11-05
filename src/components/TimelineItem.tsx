import { Button } from "./Button.tsx";
import { DiffBadge } from "./DiffBadge.tsx";
import { t } from "../strings.ts";

export interface TimelineDiffCounts {
  added?: number;
  edited?: number;
  removed?: number;
}

export interface TimelineItemProps {
  id: string;
  title: string;
  timestamp: string;
  author?: string;
  device?: string;
  diff?: TimelineDiffCounts;
  note?: string;
  isCurrent?: boolean;
  onRestore?: (id: string) => void;
  restoreLabel?: string;
  className?: string;
}

export function TimelineItem({
  id,
  title,
  timestamp,
  author,
  device,
  diff,
  note,
  isCurrent = false,
  onRestore,
  restoreLabel,
  className = "",
}: TimelineItemProps) {
  const origin = [author, device].filter(Boolean).join(" · ");
  const metaText = origin ? `${timestamp} · ${origin}` : timestamp;
  const actionText = restoreLabel ?? t("history.restore.action");

  return (
    <li className={`st-timeline-item ${className}`.trim()}>
      <div className="st-timeline-track" aria-hidden="true">
        <div className="st-timeline-node" />
        <div className="st-timeline-rule" />
      </div>

      <div className="st-timeline-content">
        <div className="st-timeline-header">
          <span className="st-timeline-title">{title}</span>
          {!isCurrent && onRestore && (
            <Button
              size="small"
              variant="secondary"
              onClick={() => onRestore(id)}
              aria-label={`${actionText} ${title}`}
            >
              {actionText}
            </Button>
          )}
        </div>

        <div className="st-timeline-meta">{metaText}</div>

        {note && <p className="st-timeline-note">{note}</p>}

        {diff && (
          <div className="st-timeline-diffs">
            {typeof diff.added === "number" && diff.added > 0 && (
              <DiffBadge type="added" count={diff.added} />
            )}
            {typeof diff.edited === "number" && diff.edited > 0 && (
              <DiffBadge type="edited" count={diff.edited} />
            )}
            {typeof diff.removed === "number" && diff.removed > 0 && (
              <DiffBadge type="removed" count={diff.removed} />
            )}
          </div>
        )}
      </div>
    </li>
  );
}
