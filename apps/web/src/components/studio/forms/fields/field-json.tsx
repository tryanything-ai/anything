import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { json } from "@codemirror/lang-json";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";
import { linter, lintGutter } from "@codemirror/lint";
import { propsPlugin } from "./codemirror-utils";
import { Fullscreen, Sparkles, Variable } from "lucide-react";
import { Button } from "@repo/ui/components/ui/button";
import { Dialog, DialogContent } from "@repo/ui/components/ui/dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@repo/ui/components/ui/tooltip";
import { useAnything } from "@/context/AnythingContext";
import { ExplorersPanel } from "@/components/studio/variable-explorers/explorer-panel";

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
  showInputsExplorer,
  showResultsExplorer,
}: any) {
  const {
    workflow: { setShowExplorer, setExplorerTab },
  } = useAnything();

  const editorRef = React.useRef<any>(null);
  const [editorValue, setEditorValue] = React.useState(
    ensureStringValue(value),
  );
  const [isValidInput, setIsValidInput] = React.useState(true);
  const [isExpanded, setIsExpanded] = React.useState(false);
  const [showAIHelper, setShowAIHelper] = React.useState(false);

  React.useEffect(() => {
    const newValue = ensureStringValue(value);
    if (newValue !== editorValue) {
      try {
        // Check for variable pattern first
        if (/^{{.*}}$/.test(newValue.trim())) {
          setEditorValue(newValue);
          setIsValidInput(true);
        } else {
          const parsed = JSON.parse(newValue);
          const formatted = JSON.stringify(parsed, null, 2);
          setEditorValue(formatted);
          setIsValidInput(true);
        }
      } catch {
        setEditorValue(newValue);
        // Only mark as invalid if it's not a partial variable pattern
        const isPartialVariable = /{{.*/.test(newValue.trim());
        setIsValidInput(isPartialVariable);
      }
    }
  }, [value]);

  const handleChange = React.useCallback(
    (val: string) => {
      try {
        // Check for complete or partial variable pattern
        const isCompleteVariable = /^{{.*}}$/.test(val.trim());
        const isPartialVariable = /{{.*/.test(val.trim());

        if (isCompleteVariable || isPartialVariable) {
          console.log("[FIELD JSON] [HANDLE CHANGE] Valid variable pattern");
          setEditorValue(val);
          setIsValidInput(true);
          onChange(name, val, true);
          return;
        }

        // Try to parse as regular JSON and format it
        console.log("[FIELD JSON] [HANDLE CHANGE] Parsing JSON:", val);
        const parsed = JSON.parse(val);
        const formatted = JSON.stringify(parsed, null, 2);
        setEditorValue(formatted);
        setIsValidInput(true);
        onChange(name, formatted, true);
      } catch (e) {
        console.log("[FIELD JSON] [HANDLE CHANGE] Invalid JSON:", e);
        setEditorValue(val);
        setIsValidInput(false);
        onChange(name, val, false);
      }
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

  const jsonLinter = linter((view) => {
    const doc = view.state.doc.toString();

    // Check for variable pattern first
    const isVariablePattern = /^{{.*}}$/.test(doc.trim());
    if (isVariablePattern) {
      return [];
    }

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
    extensions: [json(), lintGutter(), jsonLinter, propsPlugin],
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
          json
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
        <DialogContent className="max-w-[90vw] h-[90vh] flex flex-col overflow-hidden">
          <div className="flex flex-col h-full w-full overflow-hidden">
            <div className="flex-shrink-0 flex items-center justify-between mb-3">
              <Label htmlFor={name}>
                {label}{" "}
                <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
                  json
                </span>
              </Label>
            </div>
            <div className="flex gap-4 flex-1 min-h-0 overflow-hidden">
              <ExplorersPanel
                showInputsExplorer={showInputsExplorer}
                showResultsExplorer={showResultsExplorer}
              />
              <div className="flex-1 flex flex-col min-h-0 overflow-hidden">
                <div className="flex-1 overflow-hidden border rounded-md">
                  <CodeMirror
                    {...codeEditorProps}
                    className={cn(
                      "w-full h-full bg-background text-sm overflow-auto",
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

      {!isValidInput && <div className="text-red-500">Invalid Input</div>}
      {error && <div className="text-red-500">{error}</div>}
    </div>
  );
}
