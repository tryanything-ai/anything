import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { javascript } from "@codemirror/lang-javascript";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";
import { Fullscreen, Variable } from "lucide-react";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@repo/ui/components/ui/tooltip";

import { autocompletion, CompletionSource } from "@codemirror/autocomplete";
import { linter, Diagnostic } from "@codemirror/lint";
import {
  localCompletionSource,
  scopeCompletionSource,
} from "@codemirror/lang-javascript";
import { useAnything } from "@/context/AnythingContext";
import { Button } from "@repo/ui/components/ui/button";
import { Dialog, DialogContent } from "@repo/ui/components/ui/dialog";
import { ExplorersPanel } from "@/components/studio/variable-explorers/explorer-panel";

function ensureStringValue(value: any): string {
  if (value === null || value === undefined) {
    return "";
  }
  if (typeof value === "string") {
    return value;
  }
  return String(value);
}

interface CodemirrorFieldJsProps {
  name: string;
  label: string;
  value: any;
  isVisible: boolean;
  error?: string;
  disabled?: boolean;
  onChange: (name: string, value: any, isValid: boolean) => void;
  onSelect?: (event: any) => void;
  onClick?: () => void;
  onKeyUp?: () => void;
  onFocus?: () => void;
  className?: string;
  actionId?: string;
  showInputsExplorer?: boolean;
  showResultsExplorer?: boolean;
}

export default function CodemirrorFieldJs({
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
}: CodemirrorFieldJsProps) {
  const {
    workflow: {
      selected_node_inputs,
      setShowExplorer,
      showExplorer,
      setExplorerTab,
    },
  } = useAnything();
  const editorRef = React.useRef<any>(null);
  const [editorValue, setEditorValue] = React.useState(
    ensureStringValue(value),
  );
  const [isValidInput, setIsValidInput] = React.useState(true);
  const [isExpanded, setIsExpanded] = React.useState(false);

  React.useEffect(() => {
    const newValue = ensureStringValue(value);
    if (newValue !== editorValue) {
      try {
        // Check for variable pattern first
        if (/^{{.*}}$/.test(newValue.trim())) {
          setEditorValue(newValue);
          setIsValidInput(true);
        } else {
          // For JS we just set the value directly since we can't easily validate syntax
          setEditorValue(newValue);
          setIsValidInput(true);
        }
      } catch {
        setEditorValue(newValue);
        const isPartialVariable = /{{.*/.test(newValue.trim());
        setIsValidInput(isPartialVariable);
      }
    }
  }, [value]);

  const handleChange = React.useCallback(
    (val: string) => {
      // For JS we accept any input but still check for variables
      const isCompleteVariable = /^{{.*}}$/.test(val.trim());
      const isPartialVariable = /{{.*/.test(val.trim());

      if (isCompleteVariable || isPartialVariable) {
        console.log("[FIELD JS] [HANDLE CHANGE] Valid variable pattern");
        setEditorValue(val);
        setIsValidInput(true);
        onChange(name, val, true);
        return;
      }

      setEditorValue(val);
      setIsValidInput(true);
      onChange(name, val, true);
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

  /**
   * Custom completion source integrating the dynamic 'variables' object.
   */
  const variablesCompletionSource: CompletionSource = React.useMemo(() => {
    // Create a scoped completion source for the 'variables' object
    //Prefix with variables key so we access via variables.test for example
    return scopeCompletionSource({ inputs: selected_node_inputs });
  }, [selected_node_inputs]);

  const completionExtension = React.useMemo(() => {
    return [
      autocompletion({
        override: [
          // Prioritize the scoped completion for 'variables'
          variablesCompletionSource,
          // Fallback to local completions
          localCompletionSource,
        ],
        defaultKeymap: true,
        closeOnBlur: true,
      }),
      javascript({
        jsx: false, // Set to true if you need JSX support
        typescript: false,
      }),
      // Add basic JavaScript linting
      linter((view) => {
        const diagnostics: Diagnostic[] = [];
        try {
          // Basic syntax validation
          new Function(view.state.doc.toString());
        } catch (e) {
          if (e instanceof SyntaxError) {
            diagnostics.push({
              from: 0,
              to: view.state.doc.length,
              severity: "error",
              message: e.message,
            });
          }
        }
        return diagnostics;
      }),
    ];
  }, [variablesCompletionSource]);

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
    extensions: completionExtension,
    basicSetup: {
      autocompletion: false,
      lineNumbers: true,
      foldGutter: true,
      highlightActiveLine: false,
      closeBrackets: true,
      bracketMatching: true,
      indentOnInput: true,
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
          javascript
        </span>
      </Label>

      {/* Updated container with overflow handling */}
      <div className="relative min-w-[200px]">
        {/* Moved controls outside of scroll area and increased z-index */}
        <div className="absolute -top-7 right-0 z-50 flex gap-1">
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
                  onClick={() => setIsExpanded((prev) => !prev)}
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
                  javascript
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
