import React, { useState } from "react";
import CodeMirror from "@uiw/react-codemirror";
import { Label } from "@repo/ui/components/ui/label";
import { Switch } from "@repo/ui/components/ui/switch";
import { Button } from "@repo/ui/components/ui/button";
import { propsPlugin } from "./codemirror-utils";
import { cn } from "@repo/ui/lib/utils";

export default function CodeMirrorFieldBoolean({
  name,
  label,
  description,
  value,
  isVisible,
  error,
  className,
  submited,
  onChange,
  onFocus,
  onSelect,
  onClick,
  onKeyUp,
}: any) {
  // Check if value is a valid boolean string
  const isValidBoolean = (val: any) => {
    return val === "true" || val === "false";
  };

  // Initialize isVariable based on the opposite of isValidBoolean
  const [isVariable, setIsVariable] = useState(() => {
    // Show toggle first if it's a valid boolean
    return !isValidBoolean(value);
  });

  const [editorValue, setEditorValue] = React.useState(value || "");
  const editorRef = React.useRef<any>(null);

  const handleCursorActivity = React.useCallback(
    (viewUpdate: any) => {
      if (viewUpdate.view && onSelect) {
        const pos = viewUpdate.view.state.selection.main.head;
        onSelect({ target: { selectionStart: pos, selectionEnd: pos } });
      }
    },
    [onSelect],
  );

  const handleSwitchChange = (checked: boolean) => {
    const stringValue = String(checked);
    onChange(name, stringValue);
    setEditorValue(stringValue);
  };

  const handleEditorChange = React.useCallback(
    (val: string) => {
      setEditorValue(val);
      onChange(name, val);
    },
    [name, onChange],
  );

  const handleSwitchToToggle = () => {
    setIsVariable(false);
    const newValue = "true";
    setEditorValue(newValue);
    onChange(name, newValue);
  };

  const handleSwitchToVariable = () => {
    setIsVariable(true);
    setEditorValue("");
    onChange(name, "");
  };

  React.useEffect(() => {
    // Convert new value to string if it isn't already
    const newValue = typeof value === "string" ? value : String(value || "");
    if (newValue !== editorValue) {
      setEditorValue(newValue);
    }
    // Update isVariable if the new value isn't a valid boolean
    setIsVariable(!isValidBoolean(newValue));
  }, [value]);

  if (!isVisible) return null;

  return (
    <div className="grid gap-3 my-2 w-full">
      <div className="flex flex-col gap-1 [&_.cm-editor.cm-focused]:outline-none">
        <div className="flex items-center justify-between">
          <Label htmlFor={name}>
            {label}{" "}
            <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
              boolean
            </span>
          </Label>
          <Button
            variant="link"
            className="h-auto p-0"
            onClick={isVariable ? handleSwitchToToggle : handleSwitchToVariable}
          >
            {isVariable ? "Use Toggle" : "Use Variable"}
          </Button>
        </div>

        {isVariable ? (
          <div className="relative w-full overflow-hidden [&_.cm-editor.cm-focused]:outline-none">
            <CodeMirror
              ref={editorRef}
              value={editorValue}
              extensions={[propsPlugin]}
              onChange={handleEditorChange}
              onFocus={onFocus}
              onClick={onClick}
              onKeyUp={onKeyUp}
              onUpdate={handleCursorActivity}
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
        ) : (
          <div className="flex items-center gap-2 pt-2">
            <Switch
              id={name}
              className="data-[state=checked]:bg-green-400 data-[state=unchecked]:bg-input"
              checked={value === "true"}
              onCheckedChange={handleSwitchChange}
            />
            {description && (
              <div className="text-sm text-muted-foreground">{description}</div>
            )}
          </div>
        )}
      </div>

      {error && submited && <div className="text-red-500">{error}</div>}
    </div>
  );
}
