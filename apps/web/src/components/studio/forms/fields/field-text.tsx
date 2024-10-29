import { useState, useEffect, useRef } from "react";

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
import { useAnything } from "@/context/AnythingContext";

export default function FieldTex({
  type,
  name,
  label,
  const: constantValue,
  default: defaultValue,
  description,
  value,
  isVisible,
  error,
  submited,
  onChange,
  required,
  ...props
}: any) {
  console.log("FieldTex render with props:", {
    type,
    name,
    label,
    value,
    isVisible,
    error,
    submited,
    required,
  });

  // const editorRef = useRef<typeof Editor>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  const {
    explorer: { registerEditor, unregisterEditor, setActiveEditor },
  } = useAnything();

  if (!isVisible) {
    console.log("fieldtext not visible", name);
    return null;
  }

  useEffect(() => {
    console.log("[FieldText] Current textarea ref:", textareaRef.current); // Add this debug line
    const editorInfo = {
      ref: textareaRef,
      getValue: () => value,
      setValue: (newValue: string) => {
        onChange(newValue);
      },
    };
    registerEditor(name, editorInfo);
    return () => {
      unregisterEditor(name);
    };
  }, [name, registerEditor, unregisterEditor, value, onChange]);
  return (
    <div className="grid gap-3 my-4">
      <Label htmlFor={name}>{label}</Label>
      <div className="relative">
        <Editor
          id={name}
          ref={textareaRef}
          defaultValue={value}
          aria-invalid={!!error}
          aria-describedby={`${name}-error ${name}-description`}
          aria-required={required}
          value={value}
          // textareaRef={textareaRef}
          // textareaClassName="editable-input"
          onValueChange={onChange}
          // preClassName="language-handlebars"
          highlight={(code) => {
            if (!code || code.length === 0) {
              return "";
            }
            try {
              // You can verify if the grammar is loaded
              console.log("Available languages:", Object.keys(languages));
              return highlight(code, languages.handlebars, "handlebars");
            } catch (e) {
              console.error("Highlighting error:", e);
              return code; // Fallback to plain text if highlighting fails
            }
          }}
          className={cn(
            "w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
            props.className,
          )}
          padding={10}
          // style={{
          //   fontFamily: '"Fira code", "Fira Mono", monospace',
          //   fontSize: 12,
          // }}
          {...props} //order matters here which is kinda wild!
          onFocus={(e) => {
            // Stop event propagation to prevent parent from capturing
            e.stopPropagation();
            console.log("FieldText onFocus: ", name);
            setActiveEditor(name);
            // Call parent's onFocus if it exists
            props.onFocus?.(e);
          }}
        />
      </div>
      {/* <style jsx global>{`
        .language-handlebars {
          background-color: #e2f1ff;
          border-radius: 3px;
          border: 1px solid #b3d4ff;
          padding: 2px;
          color: #00008b;
        }
      `}</style> */}
    </div>
  );
}
