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
import api from "@repo/anything-api";
import { Action } from "@/types/workflows";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";
import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";
import { Badge } from "@repo/ui/components/ui/badge";
import { Button } from "@repo/ui/components/ui/button";
import { Label } from "@repo/ui/components/ui/label";
import { ExpandableInput } from "@repo/ui/components/ui/expandable-input";

export function StudioActionsSheet(): JSX.Element {
  const {
    workflow: { showingActionSheet, setShowingActionSheet, addNode },
    accounts: { selectedAccount },
  } = useAnything();
  const [actions, setActions] = useState<any>([]);
  const [addingJson, setAddingJson] = useState(false);
  const [json, setJson] = useState("");

  const fetchActions = async () => {
    try {
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      const res = await api.action_templates.getActionTemplatesForAccount(
        selectedAccount.account_id,
      );
      console.log("action sheet templates res:", res);
      setActions(res);
    } catch (error) {
      console.error("Error fetching actions:", error);
    }
  };

  const addNodeFromJson = (json: string) => {
    addNode(JSON.parse(json), { x: 100, y: 300 });
    setAddingJson(false);
    setShowingActionSheet(false);
  };

  const handleChange = (e: any) => {
    setJson(e.target.value);
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
        <SheetHeader className="flex flex-row justify-between pr-20">
          <div className="flex flex-col">
            <SheetTitle>Actions Library</SheetTitle>
            <SheetDescription>
              Add a new step to your workflow to automate your tasks.
            </SheetDescription>
          </div>

          <div className="mt-4">
            <Button
              variant="outline"
              onClick={() => {
                // TODO: Implement JSON import functionality
                setAddingJson(true);
                console.log("Add from JSON clicked");
              }}
            >
              Add from JSON
            </Button>
          </div>
        </SheetHeader>
        <div className="py-4 flex-grow overflow-hidden">
          {/* Left Hand Panel */}
          {/* <ActionPanelLeftPanelNavigation /> */}
          <div className="h-full">
            {addingJson ? (
              <div className="flex flex-col space-y-4 p-2">
                <p className="text-yellow-600 font-semibold pb-4">
                  Experimental: Providing incorrect JSON will cause problems.
                  Use at your own risk. 🐉
                </p>
                <Label htmlFor="json-input">Paste your JSON here:</Label>
                <ExpandableInput
                  className="h-64 resize-none"
                  placeholder="Enter your JSON..."
                  onChange={handleChange}
                />
                <div className="flex justify-end space-x-2">
                  <Button
                    variant="outline"
                    onClick={() => setAddingJson(false)}
                  >
                    Cancel
                  </Button>
                  <Button
                    onClick={() => {
                      addNodeFromJson(json);
                    }}
                  >
                    Add Action
                  </Button>
                </div>
              </div>
            ) : (
              <ScrollArea className="h-full pr-4">
                <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                  {actions.map((db_action: any) => {
                    let action: Action = db_action.action_template_definition;
                    let marketplace: boolean = "featured" in db_action;
                    return (
                      <div
                        key={`${db_action.action_template_id}-${action.label}`}
                        onClick={() => {
                          addNode(action, { x: 100, y: 300 });
                          setShowingActionSheet(false);
                        }}
                        className="flex flex-col justify-between p-4 border rounded-md border-black cursor-pointer hover:bg-gray-50"
                      >
                        <div className="flex flex-row gap-4 items-center" key={`content-${db_action.action_template_id}`}>
                          <BaseNodeIcon icon={action.icon} />
                          <div>
                            <div className="text-lg font-semibold">
                              {action.label}
                              {!marketplace && (
                                <Badge className="ml-2" variant="outline">
                                  Team
                                </Badge>
                              )}
                            </div>
                            <div className="text-sm font-normal truncate overflow-ellipsis">
                              {action.description}
                            </div>
                          </div>
                        </div>
                      </div>
                    );
                  })}
                </div>
              </ScrollArea>
            )}
          </div>
        </div>
      </SheetContent>
    </Sheet>
  );
}
