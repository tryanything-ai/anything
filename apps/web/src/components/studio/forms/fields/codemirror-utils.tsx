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
import { Label } from "@repo/ui/components/ui/label";
import { cn } from "@repo/ui/lib/utils";

export class PropWidget extends WidgetType {
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

export const propMatcher = new MatchDecorator({
  regexp:
    /\{\{([\w.-]+(?:\.[\w.-]+)*(?:\[\d+\])*(?:\.[\w.-]+)*(?:\[\d+\])*(?:\.[\w.-]+)*)\}\}/g,
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

export const propsPlugin = ViewPlugin.fromClass(
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
