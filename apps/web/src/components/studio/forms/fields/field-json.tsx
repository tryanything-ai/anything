import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { json } from "@codemirror/lang-json";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";
import { linter, lintGutter } from "@codemirror/lint";

export default function FieldJson({
  name,
  label,
  value,
  isVisible,
  error,
  disabled,
  onChange,
  onSelect,
  onClick,
  onKeyUp,
  onFocus,
  className,
}: any) {
  const [editorValue, setEditorValue] = React.useState(value || "{}");

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
      setEditorValue(value || "{}");
    }
  }, [value]);

  const jsonLinter = linter((view) => {
    const doc = view.state.doc.toString();
    try {
      JSON.parse(doc);
      return [];
    } catch (e) {
      const error = e as SyntaxError;
      // Extract line and character position from the error message
      const match = error.message.match(/at position (\d+)/);
      const pos = match ? parseInt(match[1]!) : 0;

      return [
        {
          from: pos,
          to: pos + 1,
          severity: "error",
          message: error.message,
        },
      ];
    }
  });

  const handleCursorActivity = React.useCallback(
    (view: any) => {
      const selection = view.state.selection.main;
      console.log("[JSON CURSOR]", selection.from);

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
      <div
        className={cn(
          "w-full rounded-md border border-input bg-background ring-offset-background focus-within:ring-2 focus-within:ring-ring focus-within:ring-offset-2",
          disabled && "opacity-50 cursor-not-allowed",
          className,
        )}
      >
        <CodeMirror
          value={editorValue}
          onChange={handleChange}
          onFocus={onFocus}
          onClick={onClick}
          onKeyUp={onKeyUp}
          onSelect={onSelect}
          readOnly={disabled}
          onUpdate={handleCursorActivity}
          extensions={[json(), lintGutter(), jsonLinter]}
          basicSetup={{
            lineNumbers: false,
            foldGutter: false,
          }}
          style={{
            minHeight: "2.5rem",
            height: "auto",
            width: "100%",
            maxWidth: "100%",
            overflow: "auto",
            wordWrap: "break-word",
            overflowWrap: "break-word",
            boxSizing: "border-box",
            fontFamily: "monospace",
            whiteSpace: "pre",
          }}
          className={cn(
            "w-full overflow-hidden rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
            className,
          )}
        />
      </div>
      {error && <div className="text-red-500">{error}</div>}
    </div>
  );
}
