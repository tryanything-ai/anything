"use client";

import { useAnything } from "@/context/AnythingContext";
import StudioHeader from "@/components/studio/studio-header";
import StudioWorkflowEditor from "@/components/studio/studio-workflow-editor";
import RightPanelFormEditor from "@/components/studio/forms/right-panel-form-editor";
import { StudioActionsSheet } from "@/components/studio/action-sheet/studio-actions-sheet";

import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

export default function StudioLayout() {
  // const { workflow } = useAnything();
  return (
    <div className="flex flex-col h-screen">
      <StudioHeader
       
      />
      <ResizablePanelGroup direction="horizontal" className="">
        <ResizablePanel defaultSize={60}>
          <div className="relative flex h-full min-h-[50vh] flex-col rounded-xl bg-muted/50 p-2 lg:col-span-2">
            {/* Add Editor */}
            <StudioWorkflowEditor />
          </div>
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel defaultSize={40}>
          {/* <div className="relative hidden flex-col items-start gap-8 md:flex"> */}
          <RightPanelFormEditor />
          {/* </div> */}
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}

//    <main className="grid flex-1 gap-4 overflow-auto p-4 md:grid-cols-2 lg:grid-cols-3">
{
  /* Main Box */
}
//     <div className="relative flex h-full min-h-[50vh] flex-col rounded-xl bg-muted/50 p-2 lg:col-span-2">
//         {/* Add Editor */}
//         <StudioWorkflowEditor />
//     </div>
//     {/* Actions sheet */}
//     <StudioActionsSheet />
//     {/* Right Hand Form */}
//     <div className="relative hidden flex-col items-start gap-8 md:flex">
//         <RightPanelFormEditor />
//     </div>
// </main>
