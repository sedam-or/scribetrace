export type DiffType = "added" | "edited" | "removed";

export interface DiffBadgeProps {
  type: DiffType;
  count: number;
  className?: string;
}

const GLYPHS: Record<DiffType, string> = {
  added: "+",
  edited: "~",
  removed: "-",
};

const LABELS: Record<DiffType, string> = {
  added: "Added",
  edited: "Edited",
  removed: "Removed",
};

export function DiffBadge({ type, count, className = "" }: DiffBadgeProps) {
  const glyph = GLYPHS[type];
  const label = LABELS[type];
  const ariaLabel = `${count} ${label.toLowerCase()}`;

  return (
    <span
      className={`st-diff-badge st-diff-${type} ${className}`.trim()}
      aria-label={ariaLabel}
    >
      <span aria-hidden="true">{glyph}{count}</span>
      <span>{label}</span>
    </span>
  );
}
