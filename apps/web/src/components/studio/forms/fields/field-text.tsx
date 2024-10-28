import { useState, useEffect, useRef } from "react";

import { Label } from "@repo/ui/components/ui/label";

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
  const [touched, setTouched] = useState(false);
  const contentRef = useRef<HTMLDivElement>(null); // Ref to the contenteditable div

  if (!isVisible) {
    console.log("fieldtext not visible", name);
    return null;
  }

  console.log("[RENDERING TEXT FIELD: ", name, " = ", value, "]");

  // Function to highlight syntax inside contenteditable
  function highlightSyntax(text: string) {
    return text.replace(/(\{\{.*?\}\})/g, '<span class="highlight">$1</span>');
  }

  // Sync the content back to the onChange handler
  function syncContent() {
    const rawValue = contentRef.current?.innerText || "";
    onChange(name, rawValue); // Call onChange with the raw text
  }

  // Handle input changes and apply syntax highlighting
  function handleInput() {
    if (!touched) setTouched(true);
    syncContent(); // Sync raw content
    applyHighlighting(); // Reapply highlighting
  }

  // Apply syntax highlighting with cursor preservation
  function applyHighlighting() {
    const rawText = contentRef.current?.innerText || "";
    const highlighted = highlightSyntax(rawText);
    contentRef.current!.innerHTML = highlighted;
    placeCursorAtEnd(contentRef.current!); // Keep cursor in place
  }

  // Keep cursor at the end after updating the innerHTML
  function placeCursorAtEnd(el: HTMLElement) {
    const range = document.createRange();
    const sel = window.getSelection();
    range.selectNodeContents(el);
    range.collapse(false);
    sel?.removeAllRanges();
    sel?.addRange(range);
  }

  // Initialize with syntax highlighting on mount
  useEffect(() => {
    if (contentRef.current) {
      contentRef.current.innerText = value || "";
      applyHighlighting(); // Initial highlighting
    }
  }, [value]);

  return (
    <div className="grid gap-3 my-4">
      <Label htmlFor={name}>{label}</Label>
      <div
        ref={contentRef}
        contentEditable
        className="editable-input border border-gray-300 p-2 rounded"
        onInput={handleInput}
        aria-invalid={!!error}
        aria-describedby={`${name}-error ${name}-description`}
        aria-required={required}
        {...props}
      />
      {(touched || submited) && error && (
        <div className="text-red-500" id={`${name}-error`}>
          {error}
        </div>
      )}
      <style jsx global>{`
        .editable-input {
          white-space: pre-wrap;
          font-family: monospace;
          min-height: 100px;
          background-color: #fafafa;
          line-height: 1.5;
        }

        .highlight {
          background-color: #e2f1ff;
          border-radius: 3px;
          padding: 0 2px;
          border: 1px solid #b3d4ff;
          margin: 0 1px;
          display: inline-block;
        }

        /* Ensure text is visible */
        .editable-input,
        .editable-input * {
          color: #000;
        }
      `}</style>
    </div>
  );
}
