import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { javascript } from "@codemirror/lang-javascript";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";

import {
  autocompletion,
  CompletionContext,
  completionKeymap,
  CompletionResult,
} from "@codemirror/autocomplete";
import { javascriptLanguage } from "@codemirror/lang-javascript";

function ensureStringValue(value: any): string {
  if (value === null || value === undefined) {
    return "";
  }
  if (typeof value === "string") {
    return value;
  }
  return String(value);
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
}: any) {
  const editorRef = React.useRef<any>(null);
  const [editorValue, setEditorValue] = React.useState(
    ensureStringValue(value),
  );
  const [isValidInput, setIsValidInput] = React.useState(true);

  const variables = {
    test: "test",
    test2: "test2",
    things: {
      test3: "test3",
    },
    things2: {
      test4: 4,
    },
  };

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

  const createCompletions = React.useCallback(
    (context: CompletionContext): CompletionResult | null => {
      // Match 'var' or 'variables' partially
      const word = context.matchBefore(/\w+\.?/);
      if (!word) return null;

      const text = word.text.toLowerCase();

      // Return early if text doesn't start with 'v' or 'var'
      if (!text.startsWith("v")) return null;

      if (text.includes(".")) {
        // Handle property completions after 'variables.'
        const word = context.matchBefore(/variables\./);
        if (!word) return null;

        // Convert variables object into completion items (same as before)
        const completions = Object.entries(variables || {}).flatMap(
          ([name, value]) => {
            if (typeof value === "object") {
              const nestedCompletions = Object.entries(value).map(
                ([subName, subValue]) => ({
                  label: `${name}.${subName}`,
                  type: typeof subValue,
                  detail: String(subValue),
                  info: `Type: ${typeof subValue}`,
                  apply: `variables.${name}.${subName}`,
                }),
              );
              return [
                {
                  label: name,
                  type: "variable",
                  detail: JSON.stringify(value),
                  info: "Object",
                  apply: `variables.${name}`,
                },
                ...nestedCompletions,
              ];
            }
            return [
              {
                label: name,
                type: typeof value,
                detail: String(value),
                info: `Type: ${typeof value}`,
                apply: `variables.${name}`,
              },
            ];
          },
        );

        return {
          from: word.from,
          options: completions,
          validFor: /^variables\..*$/,
        };
      } else {
        // Suggest 'variables' when typing 'v' or 'var'
        return {
          from: word.from,
          options: [
            {
              label: "variables",
              type: "variable",
              detail: "Access variables object",
              apply: "variables.",
            },
          ],
          validFor: /^v\w*$/,
        };
      }
    },
    [variables],
  );

  const completionExtension = React.useMemo(() => {
    return autocompletion({
      override: [createCompletions],
    });
  }, [createCompletions]);

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
          extensions={[javascript(), completionExtension]}
          basicSetup={{
            autocompletion: false,
            lineNumbers: false,
            foldGutter: false,
            completionKeymap: true,
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
      {!isValidInput && <div className="text-red-500">Invalid Input</div>}
      {error && <div className="text-red-500">{error}</div>}
    </div>
  );
}
