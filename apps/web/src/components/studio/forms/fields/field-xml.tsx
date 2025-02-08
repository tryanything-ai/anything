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
import { Fullscreen } from "lucide-react";
import { Button } from "@repo/ui/components/ui/button";
import { Dialog, DialogContent } from "@repo/ui/components/ui/dialog";

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

export default function CodemirrorFieldXml({
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
  const editorRef = React.useRef<any>(null);
  const [isExpanded, setIsExpanded] = React.useState(false);

  const handleChange = React.useCallback(
    (val: string) => {
      setEditorValue(val);
      onChange(name, val);
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

  React.useEffect(() => {
    if (value !== editorValue) {
      setEditorValue(value || "");
    }
  }, [value]);

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
    extensions: [xml(), propsPlugin],
    basicSetup: {
      lineNumbers: false,
      foldGutter: false,
      highlightActiveLine: false,
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
            xml
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
                  xml
                </span>
              </Label>
            </div>
            <CodeMirror
              {...codeEditorProps}
              className={cn(
                "w-full h-full overflow-hidden rounded-md border border-input bg-background text-sm",
                className,
              )}
              style={{
                height: "95%",
                width: "100%",
                fontFamily: "monospace",
              }}
            />
          </div>
        </DialogContent>
      </Dialog>

      {error && <div className="text-red-500">{error}</div>}
    </div>
  );
}
