import { Button } from "./Button.tsx";
import { Input } from "./Input.tsx";

export interface FolderPickerProps {
  id: string;
  label: string;
  value: string;
  onChange?: (value: string) => void;
  onBrowse?: () => void;
  browseLabel?: string;
  placeholder?: string;
  readOnly?: boolean;
  disabled?: boolean;
}

export function FolderPicker({
  id,
  label,
  value,
  onChange,
  onBrowse,
  browseLabel = "Choose folder",
  placeholder,
  readOnly = false,
  disabled = false,
}: FolderPickerProps) {
  return (
    <div className="st-form-group">
      <label htmlFor={id} className="st-label">
        {label}
      </label>
      <div className="st-picker-group">
        <Input
          id={id}
          value={value}
          onChange={onChange}
          readOnly={readOnly}
          placeholder={placeholder}
          disabled={disabled}
        />
        {onBrowse && (
          <Button
            variant="secondary"
            onClick={onBrowse}
            disabled={disabled}
            type="button"
          >
            {browseLabel}
          </Button>
        )}
      </div>
    </div>
  );
}
