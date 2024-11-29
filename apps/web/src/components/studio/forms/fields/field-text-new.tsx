import React from "react";
import CodeMirror, {
  Decoration,
  DecorationSet,
  EditorView,
  ViewPlugin,
  ViewUpdate,
  WidgetType,
  MatchDecorator,
} from "@uiw/react-codemirror";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";
import { propsPlugin } from "./codemirror-utils";

export default function FieldTextNew({
  type,
  name,
  label,
  description,
  className,
  value,
  isVisible,
  error,
  submited,
  onFocus,
  disabled,
  onChange,
  onSelect,
  onClick,
  onKeyUp,
  required,
}: any) {
  const [editorValue, setEditorValue] = React.useState(value || "");

  if (!isVisible) {
    return null;
  }

  const handleChange = React.useCallback(
    (val: string) => {
      setEditorValue(val);
      onChange(name, val);
    },
    [name, onChange],
  );

  React.useEffect(() => {
    if (value !== editorValue) {
      setEditorValue(value || "");
    }
  }, [value]);

  const handleCursorActivity = React.useCallback(
    (view: any) => {
      const selection = view.state.selection.main;

      // Create synthetic event matching the expected format
      const target = {
        selectionStart: selection.from,
        selectionEnd: selection.to,
      };
      const syntheticEvent = {
        target,
        type: "select",
      };

      // Pass to parent form handler
      onSelect?.(syntheticEvent);
    },
    [onSelect],
  );

  return (
    <div className="grid gap-3 my-2 w-full">
      <Label htmlFor={name}>{label}</Label>
      <div className="relative w-full overflow-hidden [&_.cm-editor.cm-focused]:outline-none">
        <CodeMirror
          value={editorValue}
          onChange={handleChange}
          onFocus={onFocus}
          onClick={onClick}
          onKeyUp={onKeyUp}
          onSelect={onSelect}
          readOnly={disabled}
          onUpdate={handleCursorActivity}
          extensions={[propsPlugin]}
          basicSetup={{
            lineNumbers: false,
            foldGutter: false,
            highlightActiveLine: false,
          }}
          style={{
            minHeight: "2.5rem",
            height: "auto",
            width: "100%",
            maxWidth: "100%",
            overflow: "auto",
            wordWrap: "break-word",
            overflowWrap: "break-word",
            whiteSpace: "pre-wrap",
            boxSizing: "border-box",
            fontFamily: "monospace",
            outline: "none",
          }}
          className={cn(
            "w-full overflow-hidden rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
            className,
          )}
        />
      </div>
      {error && submited && <div className="text-red-500">{error}</div>}
    </div>
  );
}
