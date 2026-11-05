import { Button } from "./Button.tsx";

export interface ErrorBannerProps {
  title: string;
  body: string;
  actionLabel?: string;
  onAction?: () => void;
  notice?: string;
  className?: string;
}

export function ErrorBanner({
  title,
  body,
  actionLabel,
  onAction,
  notice,
  className = "",
}: ErrorBannerProps) {
  return (
    <div
      role="alert"
      className={`st-banner ${className}`.trim()}
    >
      <div className="st-banner-icon" aria-hidden="true">
        <svg
          viewBox="0 0 24 24"
          width="20"
          height="20"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <circle cx="12" cy="12" r="10" />
          <line x1="12" y1="8" x2="12" y2="12" />
          <line x1="12" y1="16" x2="12.01" y2="16" />
        </svg>
      </div>

      <div className="st-banner-content">
        <div className="st-banner-title">{title}</div>
        <div className="st-banner-body">{body}</div>
        {notice && <div className="st-banner-notice">{notice}</div>}
        {actionLabel && onAction && (
          <div className="st-banner-actions">
            <Button
              size="small"
              variant="secondary"
              onClick={onAction}
            >
              {actionLabel}
            </Button>
          </div>
        )}
      </div>
    </div>
  );
}
