import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { json } from "@codemirror/lang-json";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";
import { linter, lintGutter } from "@codemirror/lint";
import { propsPlugin } from "./codemirror-utils";

function ensureStringValue(value: any): string {
  if (value === null || value === undefined) {
    return "{}";
  }
  if (typeof value === "string") {
    return value;
  }
  if (typeof value === "object") {
    return JSON.stringify(value, null, 2);
  }
  return String(value);
}

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
  const [editorValue, setEditorValue] = React.useState(
    ensureStringValue(value),
  );
  const [isValidJson, setIsValidJson] = React.useState(true);
  const editorRef = React.useRef<any>(null);

  const handleChange = React.useCallback(
    (val: string) => {
      try {
        // First try to parse as regular JSON
        JSON.parse(val);
        setEditorValue(val);
        setIsValidJson(true);
        onChange(name, val, true);
      } catch (e) {
        // If JSON parsing fails, check if it's a valid variable pattern
        const isVariablePattern = /^{{.*}}$/.test(val.trim());
        if (isVariablePattern) {
          setEditorValue(val);
          setIsValidJson(true);
          onChange(name, val, true);
        } else {
          // If neither valid JSON nor variable pattern, mark as invalid
          setEditorValue(val);
          setIsValidJson(false);
          onChange(name, val, false);
        }
      }
    },
    [name, onChange],
  );

  React.useEffect(() => {
    if (value !== editorValue) {
      setEditorValue(ensureStringValue(value));
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
      // First try to parse as regular JSON
      JSON.parse(doc);
      return [];
    } catch (e) {
      // If JSON parsing fails, check if it's a valid variable pattern
      const isVariablePattern = /^{{.*}}$/.test(doc.trim());
      if (isVariablePattern) {
        return [];
      }

      // If neither valid JSON nor variable pattern, show error
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
      <Label htmlFor={name}>
        {label}{" "}
        <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
          json
        </span>
      </Label>
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
            highlightActiveLine: false,
          }}
          className={cn(
            "w-full overflow-hidden rounded-md border border-input bg-background text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&_.cm-content]:px-1 [&_.cm-content]:py-2 [&_.cm-gutters]:h-[100%] [&_.cm-gutters]:bottom-0 [&_.cm-gutters]:absolute",
            className,
          )}
          style={{
            minHeight: "2.25rem",
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
        />
      </div>
      {!isValidJson && <div className="text-red-500">Invalid JSON</div>}
      {error && <div className="text-red-500">{error}</div>}
    </div>
  );
}
