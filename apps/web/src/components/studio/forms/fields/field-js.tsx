import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { javascript } from "@codemirror/lang-javascript";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";
import { Fullscreen } from "lucide-react";

import { autocompletion, CompletionSource } from "@codemirror/autocomplete";
import { linter, Diagnostic } from "@codemirror/lint";
import {
  localCompletionSource,
  scopeCompletionSource,
} from "@codemirror/lang-javascript";
import { useAnything } from "@/context/AnythingContext";
import { Button } from "@repo/ui/components/ui/button";
import { Dialog, DialogContent } from "@repo/ui/components/ui/dialog";
import { BaseInputsExplorer } from "@/components/studio/variable-explorers/variables-explorer";

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
    workflow: { selected_node_inputs },
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
      <div className="flex items-center justify-between">
        <Label htmlFor={name}>
          {label}{" "}
          <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
            javascript
          </span>
        </Label>
      </div>

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

      {/* Expanded modal editor */}
      <Dialog open={isExpanded} onOpenChange={setIsExpanded}>
        <DialogContent className="max-w-[90vw] h-[90vh]">
          <div className="h-full w-full">
            <div className="flex items-center justify-between mb-3">
              <Label htmlFor={name}>
                {label}{" "}
                <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
                  javascript
                </span>
              </Label>
            </div>
            {/* Add flex container for side-by-side layout in expanded mode */}
            <div className="flex gap-4 h-[95%]">
              {/* Variables Explorer Panel */}
              {showInputsExplorer && (
                <div className="w-1/4 border px-2 rounded-md bg-background">
                  <BaseInputsExplorer />
                </div>
              )}
              {/* Editor Container */}
              <div className="flex-1">
                <CodeMirror
                  {...codeEditorProps}
                  className={cn(
                    "w-full h-full overflow-hidden rounded-md border border-input bg-background text-sm",
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
        </DialogContent>
      </Dialog>

      {!isValidInput && <div className="text-red-500">Invalid Input</div>}
      {error && <div className="text-red-500">{error}</div>}
    </div>
  );
}
