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
} from "@repo/ui/components/ui/sheet";
import { useAnything } from "@/context/AnythingContext";
import api from "@/lib/anything-api";
import { Action } from "@/types/workflows";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";
import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";

export function StudioActionsSheet(): JSX.Element {
  const {
    workflow: { showingActionSheet, setShowingActionSheet, addNode },
    accounts: { selectedAccount },
  } = useAnything();
  const [actions, setActions] = useState<any>([]);

  const fetchActions = async () => {
    try {
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      const res = await api.action_templates.getActionTemplates(
        selectedAccount.account_id,
      );
      console.log("action sheet templates res:", res);
      setActions(res);
    } catch (error) {
      console.error("Error fetching actions:", error);
    }
  };

  useEffect(() => {
    fetchActions();
  }, [showingActionSheet]);

  useEffect(() => {
    if (showingActionSheet) {
      console.log("should be showing action sheet");
    } else {
      console.log("should not be showing action sheet");
    }
  }, []);

  return (
    <Sheet
      open={showingActionSheet}
      onOpenChange={(open) => setShowingActionSheet(open)}
    >
      <SheetContent side={"bottom"} className="h-4/5 flex flex-col">
        <SheetHeader>
          <SheetTitle>Actions Library</SheetTitle>
          <SheetDescription>
            Add a new step to your workflow to automate your tasks.
          </SheetDescription>
        </SheetHeader>
        <div className="py-4 flex-grow overflow-hidden">
          {/* Left Hand Panel */}
          {/* <ActionPanelLeftPanelNavigation /> */}
          <div className="h-full">
            <ScrollArea className="h-full pr-4">
              {actions.map((db_action: any) => {
                let action: Action = db_action.action_template_definition;
                return (
                  <div
                    key={db_action.action_template_id}
                    onClick={() => {
                      addNode(action, { x: 100, y: 300 });
                      setShowingActionSheet(false);
                    }}
                    className="flex flex-row justify-between items-center p-4 mb-2 border rounded-md border-black cursor-pointer hover:bg-gray-50"
                  >
                    <div className="flex flex-row gap-4 items-center">
                      <BaseNodeIcon icon={action.icon} />
                      <div>
                        <div className="text-lg font-semibold">
                          {action.label}
                        </div>
                        <div className="text-sm font-normal">
                          {action.description}
                        </div>
                      </div>
                    </div>
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
