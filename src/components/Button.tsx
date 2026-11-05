import type { ButtonHTMLAttributes, ReactNode } from "react";

export type ButtonVariant = "primary" | "secondary" | "ghost" | "destructive";
export type ButtonSize = "standard" | "small";

export interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
  children: ReactNode;
}

export function Button({
  variant = "secondary",
  size = "standard",
  type = "button",
  disabled = false,
  className = "",
  children,
  ...rest
}: ButtonProps) {
  const variantClass = `st-btn-${variant}`;
  const sizeClass = size === "small" ? "st-btn-sm" : "";
  const combinedClass = `st-btn ${variantClass} ${sizeClass} ${className}`.trim();

  return (
    <button
      type={type}
      className={combinedClass}
      disabled={disabled}
      aria-disabled={disabled ? "true" : undefined}
      {...rest}
    >
      {children}
    </button>
  );
}
