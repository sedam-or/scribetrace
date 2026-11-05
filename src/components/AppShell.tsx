import { useEffect, useState, type ReactNode } from "react";

import { t } from "../strings.ts";
import { Button } from "./Button.tsx";

export type ThemeMode = "system" | "light" | "dark";

export interface AppShellProps {
  navContent: ReactNode;
  children: ReactNode;
  headerActions?: ReactNode;
  defaultTheme?: ThemeMode;
}

export function AppShell({
  navContent,
  children,
  headerActions,
  defaultTheme = "system",
}: AppShellProps) {
  const [theme, setTheme] = useState<ThemeMode>(defaultTheme);

  useEffect(() => {
    if (typeof document === "undefined") return;

    if (theme === "system") {
      delete document.documentElement.dataset.theme;
    } else {
      document.documentElement.dataset.theme = theme;
    }
  }, [theme]);

  const cycleTheme = () => {
    setTheme((prev) => {
      if (prev === "system") return "light";
      if (prev === "light") return "dark";
      return "system";
    });
  };

  const themeLabel =
    theme === "system"
      ? "Theme: System"
      : theme === "light"
        ? "Theme: Light"
        : "Theme: Dark";

  return (
    <div className="st-app-shell">
      <a href="#main-content" className="st-skip-link">
        Skip to main content
      </a>

      <header className="st-header" role="banner">
        <div className="st-header-brand">
          <div className="st-header-logo" aria-hidden="true">
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
              <line x1="4" y1="6" x2="20" y2="6" />
              <line x1="4" y1="12" x2="16" y2="12" />
              <line x1="4" y1="18" x2="12" y2="18" />
              <circle cx="18" cy="15" r="3" />
            </svg>
          </div>
          <span className="st-header-title">{t("app.name")}</span>
          <span className="st-header-tagline">
            Remember how your reading changed
          </span>
        </div>

        <div className="st-header-controls">
          {headerActions}
          <Button
            size="small"
            variant="ghost"
            onClick={cycleTheme}
            aria-label={`Current theme is ${theme}. Click to switch theme.`}
            title={themeLabel}
          >
            {themeLabel}
          </Button>
        </div>
      </header>

      <div className="st-app-body">
        <aside className="st-nav-pane" aria-label="Papers">
          {navContent}
        </aside>

        <main id="main-content" className="st-detail-pane" tabIndex={-1}>
          {children}
        </main>
      </div>
    </div>
  );
}
