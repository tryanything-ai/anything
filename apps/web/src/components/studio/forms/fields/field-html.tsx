import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { html } from "@codemirror/lang-html";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";
import { propsPlugin } from "./codemirror-utils";
import { Fullscreen, Variable } from "lucide-react";
import { Button } from "@repo/ui/components/ui/button";
import { Dialog, DialogContent } from "@repo/ui/components/ui/dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@repo/ui/components/ui/tooltip";
import { useAnything } from "@/context/AnythingContext";
import { BaseInputsExplorer } from "@/components/studio/variable-explorers/variables-explorer";
import { BaseVariableEditingExplorer } from "@/components/studio/variable-explorers/variable-editing-explorer-layout";

export default function CodeMirrorFieldHtml({
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
  showInputsExplorer,
  showResultsExplorer,
}: any) {
  const {
    workflow: { setShowExplorer, setExplorerTab },
  } = useAnything();

  const [editorValue, setEditorValue] = React.useState(value || "");
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
    if (value !== editorValue) {
      setEditorValue(value || "");
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
    extensions: [html(), propsPlugin],
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
          html
        </span>
      </Label>

      {/* Container with relative positioning for controls overlay */}
      <div className="relative">
        {/* Controls positioned absolutely in top-right */}
        <div className="absolute -top-7 right-0 z-10 flex gap-1">
          <TooltipProvider>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="sm"
                  className="h-6 w-6 p-0"
                  onClick={() => {
                    setExplorerTab(showInputsExplorer ? "inputs" : "results");
                    setShowExplorer(true);
                  }}
                >
                  <Variable size={14} />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                <p>Explore Available Variables</p>
              </TooltipContent>
            </Tooltip>

            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="sm"
                  className="h-6 w-6 p-0"
                  onClick={() => setIsExpanded(true)}
                >
                  <Fullscreen size={14} />
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                <p>Expand Editor</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>

        {/* Editor */}
        <div className="w-full overflow-hidden [&_.cm-editor.cm-focused]:outline-none">
          <CodeMirror
            {...codeEditorProps}
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
      </div>

      {/* Expanded modal editor */}
      <Dialog open={isExpanded} onOpenChange={setIsExpanded}>
        <DialogContent className="max-w-[90vw] h-[90vh]">
          <div className="flex flex-col h-full w-full">
            <div className="flex items-center justify-between mb-3">
              <Label htmlFor={name}>
                {label}{" "}
                <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
                  html
                </span>
              </Label>
            </div>
            {/* Add flex container for side-by-side layout in expanded mode */}
            <div className="flex gap-4 flex-1 min-h-0">
              {/* Variables Explorer Panel */}
              {showInputsExplorer && (
                <div className="w-[400px] flex flex-col min-h-0">
                  <div className="flex-1 overflow-hidden border rounded-md bg-background">
                    <BaseInputsExplorer />
                  </div>
                </div>
              )}
              {showResultsExplorer && (
                <div className="w-[400px] flex flex-col min-h-0">
                  <div className="flex-1 overflow-hidden border rounded-md bg-background">
                    <BaseVariableEditingExplorer />
                  </div>
                </div>
              )}
              {/* Editor Container */}
              <div className="flex-1 flex flex-col min-h-0">
                <div className="flex-1 overflow-hidden border rounded-md">
                  <CodeMirror
                    {...codeEditorProps}
                    className={cn(
                      "w-full h-full bg-background text-sm",
                      className,
                    )}
                    style={{
                      height: "100%",
                      width: "100%",
                      fontFamily: "monospace",
                    }}
                  />
                </div>
              </div>
            </div>
          </div>
        </DialogContent>
      </Dialog>

      {error && <div className="text-red-500">{error}</div>}
    </div>
  );
}
