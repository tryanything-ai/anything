import React from "react";
import CodeMirror, {
  Decoration,
  DecorationSet,
  EditorView,
  ViewPlugin,
  ViewUpdate,
  WidgetType,
  MatchDecorator,
} from "@uiw/react-codemirror";
import { xml } from "@codemirror/lang-xml";
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";

class PropWidget extends WidgetType {
  private static activeWidgets = new Set<HTMLElement>();

  constructor(
    private fullMatch: string,
    private propName: string,
  ) {
    super();
  }

  toDOM() {
    const wrap = document.createElement("span");
    wrap.className =
      "inline-flex items-center bg-blue-100 px-1 rounded cursor-pointer border border-blue-200 ";
    wrap.textContent = this.propName;
    wrap.title = this.fullMatch;

    const toggleView = () => {
      if (wrap.textContent === this.propName) {
        // Clear other expanded widgets first
        PropWidget.activeWidgets.forEach((widget) => {
          if (widget !== wrap) {
            widget.textContent = widget.getAttribute("data-prop-name");
          }
        });
        PropWidget.activeWidgets.clear();

        // Expand this widget
        wrap.textContent = this.fullMatch;
        PropWidget.activeWidgets.add(wrap);
      } else {
        wrap.textContent = this.propName;
        PropWidget.activeWidgets.delete(wrap);
      }
    };

    wrap.addEventListener("click", toggleView);
    wrap.setAttribute("data-prop-name", this.propName);

    return wrap;
  }
}

const propMatcher = new MatchDecorator({
  regexp: /\{\{([\w.-]+(?:\.[\w.-]+)*(?:\[\d+\])*(?:\.[\w.-]+)*)\}\}/g,
  decoration: (match) => {
    const fullMatch = match[0];
    const path = match[1] || "";
    // Split on dots but preserve array indices
    const parts = path.split(/\.(?![^\[]*\])/);
    // Get last meaningful part (property or array access)
    const lastPart = parts[parts.length - 1] || "";
    // Clean up array notation if present
    const propName = lastPart.replace(/\[\d+\]/g, "");
    return Decoration.replace({
      widget: new PropWidget(fullMatch, propName),
    });
  },
});

const propsPlugin = ViewPlugin.fromClass(
  class {
    props: DecorationSet;
    constructor(view: EditorView) {
      this.props = propMatcher.createDeco(view);
    }
    update(update: ViewUpdate) {
      this.props = propMatcher.updateDeco(update, this.props);
    }
  },
  {
    decorations: (instance) => instance.props,
    provide: (plugin) =>
      EditorView.atomicRanges.of((view) => {
        return view.plugin(plugin)?.props || Decoration.none;
      }),
  },
);

export default function FieldXml({
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
  const [editorValue, setEditorValue] = React.useState(value || "");

  if (!isVisible) {
    return null;
  }

  const handleChange = React.useCallback(
    (val: string) => {
      setEditorValue(val);
      onChange(name, val);
    },
    [name, onChange],
  );

  React.useEffect(() => {
    if (value !== editorValue) {
      setEditorValue(value || "");
    }
  }, [value]);

  const handleCursorActivity = React.useCallback(
    (view: any) => {
      const selection = view.state.selection.main;

      // Create synthetic event matching the expected format
      const target = {
        selectionStart: selection.from,
        selectionEnd: selection.to,
      };
      const syntheticEvent = {
        target,
        type: "select",
      };

      // Pass to parent form handler
      onSelect?.(syntheticEvent);
    },
    [onSelect],
  );

  return (
    <div className="grid gap-3 my-2 w-full">
      <Label htmlFor={name}>{label}</Label>
      <div className="relative w-full overflow-hidden [&_.cm-editor.cm-focused]:outline-none">
        <CodeMirror
          value={editorValue}
          onChange={handleChange}
          onFocus={onFocus}
          onClick={onClick}
          onKeyUp={onKeyUp}
          onSelect={onSelect}
          readOnly={disabled}
          onUpdate={handleCursorActivity}
          extensions={[xml(), propsPlugin]}
          basicSetup={{
            lineNumbers: false,
            foldGutter: false,
            highlightActiveLine: false,
          }}
          style={{
            minHeight: "2.5rem",
            height: "auto",
            width: "100%",
            maxWidth: "100%",
            overflow: "auto",
            wordWrap: "break-word",
            overflowWrap: "break-word",
            whiteSpace: "pre-wrap",
            boxSizing: "border-box",
            fontFamily: "monospace",
          }}
          className={cn(
            "w-full overflow-hidden rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
            className,
          )}
        />
      </div>
      {error && <div className="text-red-500">{error}</div>}
    </div>
  );
}
