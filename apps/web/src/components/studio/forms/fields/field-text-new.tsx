import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";
import { propsPlugin } from "./codemirror-utils";
import { Fullscreen, Variable, AlertCircle } from "lucide-react";
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
// import { useFieldValidation } from "@/context/WorkflowVersionProvider";
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
  showInputsExplorer,
  showResultsExplorer,
  toggleStrictMode,
}: any) {
  const {
    workflow: {
      setShowExplorer,
      showExplorer,
      setExplorerTab,
      selected_node_data,
    },
  } = useAnything();

  // Directly access the strict value
  const schemaName = showResultsExplorer
    ? "inputs_schema"
    : "plugin_config_schema";
  const strict =
    selected_node_data?.[schemaName]?.properties?.[name]?.["x-any-validation"]
      ?.strict ?? true;

  // Convert value to string if it isn't already
  const initialValue = typeof value === "string" ? value : String(value || "");
  const [editorValue, setEditorValue] = React.useState(initialValue);
  const editorRef = React.useRef<any>(null);
  const [isExpanded, setIsExpanded] = React.useState(false);
  // const [isStrict, setIsStrict] = React.useState(true);

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

      {/* Updated container with overflow handling */}
      <div className="relative min-w-[200px]">
        {/* Moved controls outside of scroll area and ensured they stay on top */}
        <div className="absolute -top-7 right-0 z-50 flex gap-1">
          <TooltipProvider>
            {showResultsExplorer && (
              <Tooltip>
                <TooltipTrigger asChild>
                  <Button
                    variant="ghost"
                    size="sm"
                    className={cn(
                      "h-6 px-2 font-medium",
                      strict
                        ? "text-[8px] tracking-wider"
                        : "text-[8px] tracking-wider",
                    )}
                    onClick={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      toggleStrictMode(name, !strict);
                    }}
                  >
                    {strict ? "STRICT" : "relaxed"}
                  </Button>
                </TooltipTrigger>
                <TooltipContent side="top" className="z-[60]">
                  <p>
                    {strict
                      ? "Switch to relaxed mode. Missing variables will return defaults."
                      : "Switch to strict mode. Missing variables will make action fail."}
                  </p>
                </TooltipContent>
              </Tooltip>
            )}

            {(showInputsExplorer || showResultsExplorer) && (
              <Tooltip>
                <TooltipTrigger asChild>
                  <Button
                    variant="ghost"
                    size="sm"
                    className="h-6 w-6 p-0"
                    onClick={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      if (setShowExplorer && setExplorerTab) {
                        setExplorerTab(
                          showInputsExplorer ? "inputs" : "results",
                        );
                        setShowExplorer(true);
                      }
                    }}
                  >
                    <Variable size={14} />
                  </Button>
                </TooltipTrigger>
                <TooltipContent side="top" className="z-[60]">
                  <p>Toggle Variables Explorer</p>
                </TooltipContent>
              </Tooltip>
            )}

            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="sm"
                  className="h-6 w-6 p-0"
                  onClick={(e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    setIsExpanded((prev) => !prev);
                  }}
                >
                  <Fullscreen size={14} />
                </Button>
              </TooltipTrigger>
              <TooltipContent side="top" className="z-[60]">
                <p>Toggle Expanded Editor</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>

        {/* Updated editor container with proper overflow handling */}
        <div className="w-full overflow-x-auto [&_.cm-editor.cm-focused]:outline-none">
          <CodeMirror
            {...codeEditorProps}
            className={cn(
              "w-full rounded-md border border-input bg-background text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&_.cm-content]:px-1 [&_.cm-content]:py-2 [&_.cm-gutters]:h-[100%] [&_.cm-gutters]:bottom-0 [&_.cm-gutters]:absolute",
              className,
            )}
            style={{
              minHeight: "2.25rem",
              height: "auto",
              minWidth: "100%", // Ensures content doesn't shrink below container width
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
                  text
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

      {error && submited && <div className="text-red-500">{error}</div>}
    </div>
  );
}
