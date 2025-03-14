"use client";

import StudioHeader from "@/components/studio/studio-header";
import StudioWorkflowEditor from "@/components/studio/studio-workflow-editor";
import RightPanelFormEditor from "@/components/studio/forms/right-panel-form-editor";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@repo/ui/components/ui/resizable";
import { ResultsExplorer } from "@/components/studio/variable-explorers/results-explorer";
import { VariablesExplorer } from "@/components/studio/variable-explorers/variables-explorer";
import { useAnything } from "@/context/AnythingContext";
import VariableEditingExplorer from "@/components/studio/variable-explorers/variable-editing-explorer-layout";

export default function StudioLayout(): JSX.Element {
  const {
    workflow: { showExplorer, explorerTab },
  } = useAnything();

  return (
    <div className="flex flex-col h-screen">
      <StudioHeader />
      <ResizablePanelGroup direction="horizontal" className="">
        <ResizablePanel defaultSize={60}>
          <div className="relative flex h-full min-h-[50vh] flex-col bg-muted/50 p-2 lg:col-span-2">
            <StudioWorkflowEditor />
          </div>
        </ResizablePanel>
        {showExplorer && (
          <ResizablePanel defaultSize={40} className="flex flex-col min-h-0">
            <div className="flex-1 overflow-hidden">
              {explorerTab === "results" && <VariableEditingExplorer />}
              {explorerTab === "inputs" && <VariablesExplorer />}
            </div>
          </ResizablePanel>
        )}
        <ResizableHandle withHandle />
        <ResizablePanel defaultSize={40}>
          <RightPanelFormEditor />
        </ResizablePanel>
        {/* {showExplorer && <ResizableHandle withHandle />} */}
      </ResizablePanelGroup>
    </div>
  );
}
