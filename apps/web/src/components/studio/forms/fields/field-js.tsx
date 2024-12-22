import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { javascript } from "@codemirror/lang-javascript";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";

import {
  autocompletion,
  CompletionContext,
  CompletionResult,
  CompletionSource,
} from "@codemirror/autocomplete";
import { javascriptLanguage } from "@codemirror/lang-javascript";
import { linter, Diagnostic, Severity } from "@codemirror/lint";
import {
  localCompletionSource,
  scopeCompletionSource,
} from "@codemirror/lang-javascript";

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
}: CodemirrorFieldJsProps) {
  const editorRef = React.useRef<any>(null);
  const [editorValue, setEditorValue] = React.useState(
    ensureStringValue(value),
  );
  const [isValidInput, setIsValidInput] = React.useState(true);

  // Dynamic variables object
  const [variables, setVariables] = React.useState({
    variables: {
      test: "test",
      test2: "test2",
      things: {
        test3: "test3",
      },
      things2: {
        test4: 4,
      },
    },
  });

  // Example: Update variables dynamically (You can replace this with your actual logic)
  React.useEffect(() => {
    // Fetch or compute variables here and update using setVariables
    // For demonstration, we're keeping it static
  }, []);

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
    return scopeCompletionSource(variables);
  }, [variables]);

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
              severity: "error" as Severity,
              message: e.message,
            });
          }
        }
        return diagnostics;
      }),
    ];
  }, [variablesCompletionSource]);

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
          extensions={completionExtension}
          basicSetup={{
            autocompletion: false,
            lineNumbers: false,
            foldGutter: false,
            highlightActiveLine: false,
            closeBrackets: true,
            bracketMatching: true,
            indentOnInput: true,
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
      {!isValidInput && <div className="text-red-500">Invalid Input</div>}
      {error && <div className="text-red-500">{error}</div>}
    </div>
  );
}
