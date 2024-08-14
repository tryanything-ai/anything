"use client";

import { useEffect, useState } from "react";
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@repo/ui/components/ui//sheet";
import { useAnything } from "@/context/AnythingContext";
import api from "@/lib/anything-api";
import { Action } from "@/types/workflows";
import { ScrollArea } from "@repo/ui/components/ui//scroll-area";
import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";

export function StudioActionsSheet() {
  const { workflow } = useAnything();
  const [actions, setActions] = useState<any>([]);

  const fetchActions = async () => {
    try {
      const res = await api.action_templates.getActionTemplates();
      setActions(res);
    } catch (error) {
      console.error("Error fetching actions:", error);
    }
  };

  useEffect(() => {
    fetchActions();
  }, []);

  return (
    <Sheet
      open={workflow.showingActionSheet}
      onOpenChange={(open) => workflow.setShowingActionSheet(open)}
    >
      <SheetContent side={"bottom"} className="h-4/5">
        <SheetHeader>
          <SheetTitle>Actions Library</SheetTitle>
          <SheetDescription>
            Add a new step to your workflow to automate your tasks.
          </SheetDescription>
        </SheetHeader>
        <div className="py-4 flex flex-row">
          {/* Left Hand Panel */}
          {/* <ActionPanelLeftPanelNavigation /> */}
          <div className="flex-1 w-full h-full">
            <ScrollArea>
              {actions.map((db_action: any) => {
                let action: Action = db_action.action_template_definition;
                return (
                  <div
                    key={db_action.action_template_id}
                    onClick={() => {
                      workflow.addNode(action, { x: 100, y: 300 });
                      workflow.setShowingActionSheet(false);
                    }}
                    className="flex flex-row justify-between items-center p-4 m-1 border rounded-md border-black cursor-pointer hover:bg-gray-50"
                  >
                    <div className="flex flex-row gap-4 items-center">
                      {/* <div className="flex items-center justify-center w-10 h-10 bg-background rounded-lg"> */}
                      <BaseNodeIcon icon={action.icon} />
                      {/* <Package className="size-6 fill-foreground" />   */}
                      {/* </div> */}
                      <div>
                        <div className="text-lg font-semibold">
                          {action.label}
                        </div>
                        <div className="text-sm font-normal">
                          {action.description}
                        </div>
                      </div>
                    </div>
                    {/* <div>
                                        <Button variant="primary" size="sm">Add</Button>
                                    </div> */}
                  </div>
                );
              })}
            </ScrollArea>
          </div>
        </div>
      </SheetContent>
    </Sheet>
  );
}
