import type { ReactNode } from "react";

import { Button } from "./Button.tsx";

export interface EmptyStateProps {
  title: string;
  body: string;
  actionLabel?: string;
  onAction?: () => void;
  diagnostic?: string;
  icon?: ReactNode;
  className?: string;
}

export function EmptyState({
  title,
  body,
  actionLabel,
  onAction,
  diagnostic,
  icon,
  className = "",
}: EmptyStateProps) {
  return (
    <div className={`st-empty-state ${className}`.trim()}>
      <div className="st-empty-icon" aria-hidden="true">
        {icon ?? (
          <svg
            viewBox="0 0 24 24"
            width="48"
            height="48"
            fill="none"
            stroke="currentColor"
            strokeWidth="1.5"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
            <polyline points="14 2 14 8 20 8" />
            <line x1="16" y1="13" x2="8" y2="13" />
            <line x1="16" y1="17" x2="8" y2="17" />
            <polyline points="10 9 9 9 8 9" />
          </svg>
        )}
      </div>

      <h3 className="st-empty-title">{title}</h3>
      <p className="st-empty-body">{body}</p>

      {diagnostic && <div className="st-empty-diagnostic">{diagnostic}</div>}

      {actionLabel && onAction && (
        <Button variant="primary" onClick={onAction}>
          {actionLabel}
        </Button>
      )}
    </div>
  );
}
