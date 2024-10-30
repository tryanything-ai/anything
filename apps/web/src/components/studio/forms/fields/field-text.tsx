import Editor from "react-simple-code-editor";
import { highlight,languages } from "prismjs/components/prism-core";
import "prismjs/components/prism-core";
import "prismjs/components/prism-clike";
import "prismjs/components/prism-markup";
import "prismjs/components/prism-markup-templating";
import "prismjs/components/prism-handlebars";
import "prismjs/themes/prism.css";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";

export default function FieldText({
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
  if (!isVisible) {
    console.log("fieldtext not visible", name);
    return null;
  }

  return (
    <div className="grid gap-3 my-4">
      <Label htmlFor={name}>{label}</Label>
      <div className="relative">
        <Editor
          id={name}
          defaultValue={value}
          aria-invalid={!!error}
          aria-describedby={`${name}-error ${name}-description`}
          aria-required={required}
          value={value}
          onValueChange={onChange}
          highlight={(code) => {
            if (!code || code.length === 0) {
              return "";
            }
            try {
              return highlight(code, languages.handlebars, "handlebars");
            } catch (e) {
              console.error("Highlighting error:", e);
              return code; // Fallback to plain text if highlighting fails
            }
          }}
          padding={10}
          className={cn(
            "w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
            props.className,
          )}
          {...props} //order matters here which is kinda wild!
        />
      </div>
    </div>
  );
}
