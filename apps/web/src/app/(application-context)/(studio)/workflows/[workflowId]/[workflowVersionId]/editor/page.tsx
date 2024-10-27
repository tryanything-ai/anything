"use client";

import StudioHeader from "@/components/studio/studio-header";
import StudioWorkflowEditor from "@/components/studio/studio-workflow-editor";
import RightPanelFormEditor from "@/components/studio/forms/right-panel-form-editor";
import { useState } from "react";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@repo/ui/components/ui/resizable";
import { ResultsExplorer } from "@/components/studio/variable-explorers/results-explorer";
import { VariablesExplorer } from "@/components/studio/variable-explorers/variables-explorer";
import { useAnything } from "@/context/AnythingContext";

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
        <ResizableHandle withHandle />
        <ResizablePanel defaultSize={40}>
          <RightPanelFormEditor />
        </ResizablePanel>
        {showExplorer && <ResizableHandle withHandle />}
        {showExplorer && (
          <ResizablePanel defaultSize={20}>
            {explorerTab === "results" && <ResultsExplorer />}
            {explorerTab === "variables" && <VariablesExplorer />}
          </ResizablePanel>
        )}
      </ResizablePanelGroup>
    </div>
  );
}
