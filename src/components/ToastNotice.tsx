import { useEffect } from "react";

export interface ToastNoticeProps {
  message: string;
  visible: boolean;
  onDismiss?: () => void;
  className?: string;
}

export function ToastNotice({
  message,
  visible,
  onDismiss,
  className = "",
}: ToastNoticeProps) {
  useEffect(() => {
    if (!visible || !onDismiss) return;

    const handleKeyDown = (e: globalThis.KeyboardEvent) => {
      if (e.key === "Escape") {
        onDismiss();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [visible, onDismiss]);

  if (!visible) return null;

  return (
    <div
      role="status"
      aria-live="polite"
      className={`st-toast ${className}`.trim()}
    >
      <span>{message}</span>
      {onDismiss && (
        <button
          type="button"
          onClick={onDismiss}
          className="st-toast-close"
          aria-label="Dismiss notice"
        >
          <svg
            viewBox="0 0 24 24"
            width="16"
            height="16"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      )}
    </div>
  );
}
