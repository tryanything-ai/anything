import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { json } from "@codemirror/lang-json";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";
import { linter, lintGutter } from "@codemirror/lint";
import { propsPlugin } from "./codemirror-utils";

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
  const [isValidJson, setIsValidJson] = React.useState(true);
  const editorRef = React.useRef<any>(null);

  const handleChange = React.useCallback(
    (val: string) => {
      try {
        // Try to parse and format the JSON
        const parsed = JSON.parse(val);
        const formatted = JSON.stringify(parsed, null, 2);
        setEditorValue(formatted);
        setIsValidJson(true);
        onChange(name, parsed, true);
      } catch (e) {
        // If it's not valid JSON, update with raw value but mark as invalid
        setEditorValue(val);
        setIsValidJson(false);
        onChange(name, val, false);
      }
    },
    [name, onChange],
  );

  React.useEffect(() => {
    if (value !== editorValue) {
      setEditorValue(value || "{}");
    }
  }, [value]);

  const handleCursorActivity = React.useCallback(
    (viewUpdate: any) => {
      if (viewUpdate.view) {
        const pos = viewUpdate.view.state.selection.main.head;
        if (onSelect) {
          onSelect({ target: { selectionStart: pos, selectionEnd: pos } });
        }
      }
    },
    [onSelect],
  );

  const jsonLinter = linter((view) => {
    const doc = view.state.doc.toString();
    try {
      JSON.parse(doc);
      return [];
    } catch (e) {
      const error = e as SyntaxError;
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

  if (!isVisible) {
    return null;
  }

  return (
    <div className="grid gap-3 my-2 w-full">
      <Label htmlFor={name}>{label}</Label>
      <div className="relative w-full overflow-hidden [&_.cm-editor.cm-focused]:outline-none">
        <CodeMirror
          ref={editorRef}
          value={editorValue}
          onChange={handleChange}
          onFocus={onFocus}
          onClick={onClick}
          onKeyUp={onKeyUp}
          onUpdate={handleCursorActivity}
          readOnly={disabled}
          extensions={[json(), lintGutter(), jsonLinter, propsPlugin]}
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
            whiteSpace: "pre-wrap",
            boxSizing: "border-box",
            fontFamily: "monospace",
          }}
          className={cn(
            "w-full overflow-hidden rounded-md border border-input bg-background text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
            className,
          )}
        />
      </div>
      {!isValidJson && <div className="text-red-500">Invalid JSON</div>}
      {error && <div className="text-red-500">{error}</div>}
    </div>
  );
}
