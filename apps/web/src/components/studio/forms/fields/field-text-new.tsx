import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";
import { propsPlugin } from "./codemirror-utils";
import { Fullscreen } from "lucide-react";
import { Button } from "@repo/ui/components/ui/button";
import { Dialog, DialogContent } from "@repo/ui/components/ui/dialog";

export default function CodeMirrorFieldText({
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
  // Convert value to string if it isn't already
  const initialValue = typeof value === "string" ? value : String(value || "");
  const [editorValue, setEditorValue] = React.useState(initialValue);
  const editorRef = React.useRef<any>(null);
  const [isExpanded, setIsExpanded] = React.useState(false);

  const handleChange = React.useCallback(
    (val: string) => {
      setEditorValue(val);
      onChange(name, val);
    },
    [name, onChange],
  );

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

  React.useEffect(() => {
    // Convert new value to string if it isn't already
    const newValue = typeof value === "string" ? value : String(value || "");
    if (newValue !== editorValue) {
      setEditorValue(newValue);
    }
  }, [value]);

  // Add shared CodeMirror component config
  const codeEditorProps = {
    ref: editorRef,
    value: editorValue,
    onChange: handleChange,
    onFocus: onFocus,
    onClick: onClick,
    onKeyUp: onKeyUp,
    onUpdate: handleCursorActivity,
    readOnly: disabled,
    extensions: [propsPlugin],
    basicSetup: {
      lineNumbers: false,
      foldGutter: false,
      highlightActiveLine: false,
    },
  };

  if (!isVisible) {
    return null;
  }

  return (
    <div className="grid gap-3 my-2 w-full">
      <Label htmlFor={name}>
        {label}{" "}
        <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
          text
        </span>
      </Label>

      {/* Regular inline editor */}
      <div className="relative w-full overflow-hidden [&_.cm-editor.cm-focused]:outline-none">
        <Button
          variant="ghost"
          size="sm"
          onClick={() => setIsExpanded(true)}
          className="absolute right-0 top-0 z-10"
        >
          <Fullscreen size={16} />
        </Button>
        <CodeMirror
          {...codeEditorProps}
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

      {/* Expanded modal editor */}
      <Dialog open={isExpanded} onOpenChange={setIsExpanded}>
        <DialogContent className="max-w-[90vw] h-[90vh]">
          <div className="h-full w-full">
            <div className="flex items-center justify-between mb-3">
              <Label htmlFor={name}>
                {label}{" "}
                <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
                  text
                </span>
              </Label>
            </div>
            <CodeMirror
              {...codeEditorProps}
              className={cn(
                "w-full h-full overflow-hidden rounded-md border border-input bg-background text-sm",
                className,
              )}
              style={{
                height: "95%",
                width: "100%",
                fontFamily: "monospace",
              }}
            />
          </div>
        </DialogContent>
      </Dialog>

      {error && submited && <div className="text-red-500">{error}</div>}
    </div>
  );
}
