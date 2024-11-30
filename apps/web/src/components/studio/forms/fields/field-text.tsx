import Editor from "react-simple-code-editor";
import { highlight, languages } from "prismjs/components/prism-core";
import "prismjs/components/prism-core";
import "prismjs/components/prism-clike";
import "prismjs/components/prism-markup";
import "prismjs/components/prism-markup-templating";
import "prismjs/components/prism-handlebars";
import "prismjs/themes/prism.css";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";
import { useState } from "react";

export default function ReactSimpleCodeEditorFieldText({
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
  ...props
}: any) {
  const [touched, setTouched] = useState(false);

  if (!isVisible) {
    console.log("fieldtext not visible", name);
    return null;
  }

  console.log("[FIELDTEXT]:", value);

  const displayError = submited || touched ? error : null;

  ///TODO: get rid of this. not the right direction
  function cleanAndParseValue(val: string) {
    try {
      // Remove escaped characters and newlines
      const cleaned = val
        .replace(/\\n/g, " ")
        .replace(/\\/g, "")
        .replace(/\n/g, " ")
        .trim();

      // Attempt to parse as JSON if it looks like JSON
      if (cleaned.startsWith("{") || cleaned.startsWith("[")) {
        try {
          const parsed = JSON.parse(cleaned);
          return JSON.stringify(parsed);
        } catch (e) {
          // If JSON parsing fails, return cleaned string
          return cleaned;
        }
      }
      return cleaned;
    } catch (e) {
      console.error("Error cleaning value:", e);
      return val;
    }
  }

  function handleChange(value: any) {
    if (!touched) setTouched(true);
    const cleanedValue = cleanAndParseValue(value);
    console.log(`[FIELD TEXT FIELD] [HANDLE CHANGE] ${name}:`, cleanedValue);
    onChange(name, cleanedValue);
  }

  return (
    <div className="grid gap-3 my-2 w-full overflow-hidden">
      <Label htmlFor={name}>{label}</Label>
      <div className="relative w-full overflow-hidden">
        <Editor
          id={name}
          onFocus={onFocus}
          aria-invalid={!!error}
          aria-describedby={`${name}-error ${name}-description`}
          aria-required={required}
          value={value}
          onSelect={onSelect}
          onClick={onClick}
          onKeyUp={onKeyUp}
          onValueChange={handleChange}
          highlight={(code) => {
            if (!code || code.length === 0) {
              console.log("No code to highlight");
              return "";
            }
            try {
              return highlight(code, languages.handlebars, "handlebars");
            } catch (e) {
              console.error("Highlighting error:", e);
              return code;
            }
          }}
          padding={10}
          disabled={disabled}
          style={{
            minHeight: "2.5rem",
            // maxHeight: "300px",
            height: "auto",
            width: "100%",
            maxWidth: "100%",
            overflow: "auto",
            wordWrap: "break-word",
            overflowWrap: "break-word",
            boxSizing: "border-box",
            fontFamily: "monospace",
            whiteSpace: "pre",
          }}
          className={cn(
            "w-full overflow-hidden rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
            className,
          )}
        />
      </div>
      {displayError && <div className="text-red-500">{displayError}</div>}
    </div>
  );
}
