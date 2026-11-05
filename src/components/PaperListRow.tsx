import type { KeyboardEvent, MouseEvent } from "react";

import { t } from "../strings.ts";
import { Button } from "./Button.tsx";

export interface PaperListRowProps {
  id: string;
  title: string;
  authors?: string;
  year?: string | number;
  marksCount: number;
  statusText?: string;
  active?: boolean;
  onSelect?: (id: string) => void;
  onRefresh?: (id: string, e: MouseEvent<HTMLButtonElement>) => void;
  className?: string;
}

export function PaperListRow({
  id,
  title,
  authors,
  year,
  marksCount,
  statusText,
  active = false,
  onSelect,
  onRefresh,
  className = "",
}: PaperListRowProps) {
  const handleKeyDown = (e: KeyboardEvent<HTMLDivElement>) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      onSelect?.(id);
    }
  };

  const handleClick = () => {
    onSelect?.(id);
  };

  const handleRefreshClick = (e: MouseEvent<HTMLButtonElement>) => {
    e.stopPropagation();
    onRefresh?.(id, e);
  };

  const metaString = [authors, year].filter(Boolean).join(" · ");

  return (
    <div
      role="button"
      tabIndex={0}
      aria-selected={active}
      onClick={handleClick}
      onKeyDown={handleKeyDown}
      className={`st-list-row ${active ? "active" : ""} ${className}`.trim()}
      aria-label={`${title}${metaString ? `, ${metaString}` : ""}, ${marksCount} marks`}
    >
      <div className="st-list-info">
        <span className="st-list-title">{title}</span>
        <span className="st-list-meta">
          {metaString && <span>{metaString}</span>}
          {statusText && (
            <span title={t("papers.no_marks.help")}>· {statusText}</span>
          )}
        </span>
      </div>

      <div className="st-list-actions">
        <span className="st-list-badge">
          {marksCount} {marksCount === 1 ? "mark" : "marks"}
        </span>
        {onRefresh && (
          <Button
            size="small"
            variant="secondary"
            onClick={handleRefreshClick}
            aria-label={`Refresh marks for ${title}`}
          >
            Refresh
          </Button>
        )}
      </div>
    </div>
  );
}
