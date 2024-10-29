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

  const [touched, setTouched] = useState(false);
  const editorRef = useRef<HTMLDivElement>(null);
  const {
    explorer: { registerEditorRef, unregisterEditorRef, setActiveEditor },
  } = useAnything();

  if (!isVisible) {
    console.log("fieldtext not visible", name);
    return null;
  }

  console.log("[RENDERING TEXT FIELD: ", name, " = ", value, "]");

  // Function to highlight syntax inside contenteditable
  function highlightSyntax(text: string) {
    console.log("highlightSyntax input:", text);
    const result = text.replace(
      /(\{\{.*?\}\})/g,
      '<span class="highlight">$1</span>',
    );
    console.log("highlightSyntax output:", result);
    return result;
  }

  // Sync the content back to the onChange handler
  function syncContent() {
    console.log("syncContent called");
    const rawValue = editorRef.current?.innerText || "";
    console.log("syncContent rawValue:", rawValue);
    onChange(name, rawValue); // Call onChange with the raw text
  }

  // Handle input changes and apply syntax highlighting
  function handleInput() {
    console.log("handleInput called");
    if (!touched) {
      console.log("Setting touched to true");
      setTouched(true);
    }
    syncContent(); // Sync raw content
    applyHighlighting(); // Reapply highlighting
  }

  // Apply syntax highlighting with cursor preservation
  function applyHighlighting() {
    console.log("applyHighlighting called");
    const rawText = editorRef.current?.innerText || "";
    console.log("Current raw text:", rawText);
    const highlighted = highlightSyntax(rawText);
    console.log("Highlighted text:", highlighted);
    editorRef.current!.innerHTML = highlighted;
    placeCursorAtEnd(editorRef.current!); // Keep cursor in place
  }

  // Keep cursor at the end after updating the innerHTML
  function placeCursorAtEnd(el: HTMLElement) {
    console.log("placeCursorAtEnd called");
    const range = document.createRange();
    const sel = window.getSelection();
    range.selectNodeContents(el);
    range.collapse(false);
    sel?.removeAllRanges();
    sel?.addRange(range);
    console.log("Cursor position updated");
  }

  useEffect(() => {
    console.log("useEffect triggered with value:", value);
    if (editorRef.current) {
      console.log("Setting initial text:", value || "");
      editorRef.current.innerText = value || "";
      applyHighlighting(); // Initial highlighting
    } else {
      console.log("editorRef.current is null");
    }
  }, [value]);

  useEffect(() => {
    registerEditorRef(name, editorRef);
    return () => unregisterEditorRef(name);
  }, [name]);

  const [code, setCode] = useState(`{{ variable }}`);

  console.log("FieldText render with value:", value);

  return (
    <>
      <Label htmlFor={name}>{label}</Label>
      <Editor
        id={name}
        type="text"
        defaultValue={value}
        aria-invalid={!!error}
        aria-describedby={`${name}-error ${name}-description`}
        aria-required={required}
        value={value}
        onValueChange={onChange}
        // highlight={(code) => {
        //   if (!code || code.length === 0) {
        //     return "";
        //   }
        //   return highlight(code, languages.handlebars, "handlebars");
        // }}
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
        style={{
          fontFamily: '"Fira code", "Fira Mono", monospace',
          fontSize: 12,
        }}
        {...props}
      />
    </>
  );

  // return (
  //   <div className="grid gap-3 my-4">
  //     <Label htmlFor={name}>{label}</Label>
  //     <div className="relative">
  //       <div
  //         ref={editorRef}
  //         contentEditable
  //         className="editable-input w-full min-h-[40px] max-h-[300px] rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
  //         onInput={handleInput}
  //         // onFocus={() => setActiveEditor(name)}
  //         // onFocus={(e) => {
  //         //   setActiveEditor(name);
  //         //   // Call parent's onFocus if it exists
  //         //   props.onFocus?.(e);
  //         // }}
  //         aria-invalid={!!error}
  //         aria-describedby={`${name}-error ${name}-description`}
  //         aria-required={required}
  //         style={{
  //           resize: "vertical",
  //           overflowY: "auto",
  //         }}
  //         {...props}
  //       />
  //     </div>
  //     {(touched || submited) && error && (
  //       <div className="text-red-500" id={`${name}-error`}>
  //         {error}
  //       </div>
  //     )}
  //     <style jsx global>{`
  //       .editable-input {
  //         white-space: pre-wrap;
  //         font-family: monospace;
  //         background-color: #fafafa;
  //         line-height: 1.5;
  //       }

  //       .highlight {
  //         background-color: #e2f1ff;
  //         border-radius: 3px;
  //         padding: 0 2px;
  //         border: 1px solid #b3d4ff;
  //         margin: 0 1px;
  //         display: inline-block;
  //       }

  //       /* Ensure text is visible */
  //       .editable-input,
  //       .editable-input * {
  //         color: #000;
  //       }

  //       /* Add resize handle styling */
  //       .editable-input::-webkit-resizer {
  //         border-width: 6px;
  //         border-style: solid;
  //         border-color: transparent #ccc #ccc transparent;
  //         background-color: transparent;
  //       }
  //     `}</style>
  //   </div>
  // );
}
