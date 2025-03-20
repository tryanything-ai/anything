import React, { useState } from "react";
import CodeMirror from "@uiw/react-codemirror";
import { Label } from "@repo/ui/components/ui/label";
import { Switch } from "@repo/ui/components/ui/switch";
import { Button } from "@repo/ui/components/ui/button";
import { propsPlugin } from "./codemirror-utils";
import { cn } from "@repo/ui/lib/utils";
import { Fullscreen, Variable } from "lucide-react";
import { Dialog, DialogContent } from "@repo/ui/components/ui/dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@repo/ui/components/ui/tooltip";
import { useAnything } from "@/context/AnythingContext";
import { ExplorersPanel } from "@/components/studio/variable-explorers/explorer-panel";

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
  showInputsExplorer,
  showResultsExplorer,
}: any) {
  const {
    workflow: { setShowExplorer, showExplorer, setExplorerTab },
  } = useAnything();

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
  const [isExpanded, setIsExpanded] = React.useState(false);

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

  // Add shared CodeMirror component config
  const codeEditorProps = {
    ref: editorRef,
    value: editorValue,
    onChange: handleEditorChange,
    onFocus: onFocus,
    onClick: onClick,
    onKeyUp: onKeyUp,
    onUpdate: handleCursorActivity,
    extensions: [propsPlugin],
    basicSetup: {
      lineNumbers: false,
      foldGutter: false,
      highlightActiveLine: false,
    },
  };

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
          <div className="flex items-center gap-2">
            <Button
              variant="link"
              className="h-auto p-0"
              onClick={
                isVariable ? handleSwitchToToggle : handleSwitchToVariable
              }
            >
              {isVariable ? "Use Toggle" : "Use Variable"}
            </Button>
            {isVariable && (
              <div className="flex gap-1">
                <TooltipProvider>
                  {(showInputsExplorer || showResultsExplorer) && (
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <Button
                          variant="ghost"
                          size="sm"
                          className="h-6 w-6 p-0"
                          onClick={() => {
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
                      <TooltipContent>
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
                        onClick={() => setIsExpanded((prev) => !prev)}
                      >
                        <Fullscreen size={14} />
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <p>Toggle Expanded Editor</p>
                    </TooltipContent>
                  </Tooltip>
                </TooltipProvider>
              </div>
            )}
          </div>
        </div>

        {isVariable ? (
          <div className="relative w-full overflow-hidden [&_.cm-editor.cm-focused]:outline-none">
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

      {/* Expanded modal editor */}
      {isVariable && (
        <Dialog open={isExpanded} onOpenChange={setIsExpanded}>
          <DialogContent className="max-w-[90vw] h-[90vh] flex flex-col overflow-hidden">
            <div className="flex flex-col h-full w-full overflow-hidden">
              <div className="flex-shrink-0 flex items-center justify-between mb-3">
                <Label htmlFor={name}>
                  {label}{" "}
                  <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
                    boolean
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
      )}

      {error && submited && <div className="text-red-500">{error}</div>}
    </div>
  );
}
