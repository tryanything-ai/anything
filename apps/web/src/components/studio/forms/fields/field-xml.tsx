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
import { Fullscreen, Variable } from "lucide-react";
import { Button } from "@repo/ui/components/ui/button";
import { Dialog, DialogContent } from "@repo/ui/components/ui/dialog";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@repo/ui/components/ui/tooltip";
import { useAnything } from "@/context/AnythingContext";
import { ExplorersPanel } from "@/components/studio/variable-explorers/explorer-panel";

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
  showInputsExplorer,
  showResultsExplorer,
  toggleStrictMode,
}: any) {
  const {
    workflow: {
      setShowExplorer,
      showExplorer,
      setExplorerTab,
      selected_node_data,
    },
  } = useAnything();

  // Directly access the strict value
  const schemaName = showResultsExplorer
    ? "inputs_schema"
    : "plugin_config_schema";
  const strict =
    selected_node_data?.[schemaName]?.properties?.[name]?.["x-any-validation"]
      ?.strict ?? true;

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
      <Label htmlFor={name}>
        {label}{" "}
        <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
          xml
        </span>
      </Label>

      {/* Updated container with overflow handling */}
      <div className="relative min-w-[200px]">
        {/* Moved controls outside of scroll area and increased z-index */}
        <div className="absolute -top-7 right-0 z-50 flex gap-1">
          <TooltipProvider>
            {showResultsExplorer && (
              <Tooltip>
                <TooltipTrigger asChild>
                  <Button
                    variant="ghost"
                    size="sm"
                    className={cn(
                      "h-6 px-2 font-medium",
                      strict
                        ? "text-[8px] tracking-wider"
                        : "text-[8px] tracking-wider",
                    )}
                    onClick={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      toggleStrictMode(name, !strict);
                    }}
                  >
                    {strict ? "STRICT" : "relaxed"}
                  </Button>
                </TooltipTrigger>
                <TooltipContent side="top" className="z-[60]">
                  <p>
                    {strict
                      ? "Switch to relaxed mode. Missing variables will return defaults."
                      : "Switch to strict mode. Missing variables will make action fail."}
                  </p>
                </TooltipContent>
              </Tooltip>
            )}

            {(showInputsExplorer || showResultsExplorer) && (
              <Tooltip>
                <TooltipTrigger asChild>
                  <Button
                    variant="ghost"
                    size="sm"
                    className="h-6 w-6 p-0"
                    onClick={() => {
                      if (setShowExplorer && setExplorerTab) {
                        setExplorerTab(
                          showInputsExplorer ? "inputs" : "results",
                        );
                        setShowExplorer(true);
                      }
                    }}
                  >
                    <Variable size={14} />
                  </Button>
                </TooltipTrigger>
                <TooltipContent side="top" className="z-[60]">
                  <p>Toggle Variables Explorer</p>
                </TooltipContent>
              </Tooltip>
            )}

            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="sm"
                  className="h-6 w-6 p-0"
                  onClick={() => setIsExpanded((prev) => !prev)}
                >
                  <Fullscreen size={14} />
                </Button>
              </TooltipTrigger>
              <TooltipContent side="top" className="z-[60]">
                <p>Toggle Expanded Editor</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </div>

        {/* Updated editor container with proper overflow handling */}
        <div className="w-full overflow-x-auto [&_.cm-editor.cm-focused]:outline-none">
          <CodeMirror
            {...codeEditorProps}
            className={cn(
              "w-full rounded-md border border-input bg-background text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&_.cm-content]:px-1 [&_.cm-content]:py-2 [&_.cm-gutters]:h-[100%] [&_.cm-gutters]:bottom-0 [&_.cm-gutters]:absolute",
              className,
            )}
            style={{
              minHeight: "2.25rem",
              height: "auto",
              minWidth: "100%", // Ensures content doesn't shrink below container width
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
      </div>

      {/* Expanded modal editor */}
      <Dialog open={isExpanded} onOpenChange={setIsExpanded}>
        <DialogContent className="max-w-[90vw] h-[90vh] flex flex-col overflow-hidden">
          <div className="flex flex-col h-full w-full overflow-hidden">
            <div className="flex-shrink-0 flex items-center justify-between mb-3">
              <Label htmlFor={name}>
                {label}{" "}
                <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
                  xml
                </span>
              </Label>
            </div>
            <div className="flex gap-4 flex-1 min-h-0 overflow-hidden">
              <ExplorersPanel
                showInputsExplorer={showInputsExplorer}
                showResultsExplorer={showResultsExplorer}
              />
              <div className="flex-1 flex flex-col min-h-0 overflow-hidden">
                <div className="flex-1 overflow-hidden border rounded-md">
                  <CodeMirror
                    {...codeEditorProps}
                    className={cn(
                      "w-full h-full bg-background text-sm overflow-auto",
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
          </div>
        </DialogContent>
      </Dialog>

      {error && <div className="text-red-500">{error}</div>}
    </div>
  );
}
