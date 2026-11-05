import type { ChangeEvent, InputHTMLAttributes } from "react";

export interface InputProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "onChange"> {
  id?: string;
  label?: string;
  value: string;
  onChange?: (value: string, e: ChangeEvent<HTMLInputElement>) => void;
  className?: string;
}

export function Input({
  id,
  label,
  value,
  onChange,
  className = "",
  type = "text",
  ...rest
}: InputProps) {
  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (onChange) {
      onChange(e.target.value, e);
    }
  };

  const inputElement = (
    <input
      id={id}
      type={type}
      value={value}
      onChange={handleChange}
      className={`st-input ${className}`.trim()}
      {...rest}
    />
  );

  if (label && id) {
    return (
      <div className="st-form-group">
        <label htmlFor={id} className="st-label">
          {label}
        </label>
        {inputElement}
      </div>
    );
  }

  return inputElement;
}
