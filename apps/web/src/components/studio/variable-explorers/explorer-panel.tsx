import React from "react";

import { BaseInputsExplorer } from "@/components/studio/variable-explorers/variables-explorer";
import { BaseVariableEditingExplorer } from "../variable-explorers/variable-editing-explorer-layout";

interface ExplorersPanelProps {
  showInputsExplorer?: boolean;
  showResultsExplorer?: boolean;
}

export function ExplorersPanel({
  showInputsExplorer,
  showResultsExplorer,
}: ExplorersPanelProps) {
  if (!showInputsExplorer && !showResultsExplorer) return null;

  return (
    <div className="flex-shrink-0 flex flex-col gap-4 w-[400px]">
      {showInputsExplorer && (
        <div className="flex-1 min-h-0">
          <div className="h-full overflow-auto border rounded-md bg-background">
            <BaseInputsExplorer />
          </div>
        </div>
      )}
      {showResultsExplorer && (
        <div className="flex-1 min-h-0">
          <div className="h-full overflow-auto border rounded-md bg-background p-2">
            <BaseVariableEditingExplorer />
          </div>
        </div>
      )}
    </div>
  );
}
