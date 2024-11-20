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
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@repo/ui/components/ui/tabs";
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
    workflow: {
      showingActionSheet,
      setShowingActionSheet,
      addNode,
      actionSheetMode,
      setActionSheetMode,
      changeTrigger,
    },
    accounts: { selectedAccount },
  } = useAnything();

  const [actions, setActions] = useState<any>([]);
  const [triggers, setTriggers] = useState<any>([]);
  const [other, setOther] = useState<any>([]);

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

  const fetchTriggers = async () => {
    try {
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      const res = await api.action_templates.getTriggerTemplatesForAccount(
        selectedAccount.account_id,
      );
      console.log("action sheet trigger templates res:", res);
      setTriggers(res);
    } catch (error) {
      console.error("Error fetching triggers:", error);
    }
  };

  const fetchOther = async () => {
    try {
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      const res = await api.action_templates.getOtherActionTemplatesForAccount(
        selectedAccount.account_id,
      );
      console.log("action sheet trigger templates res:", res);
      setOther(res);
    } catch (error) {
      console.error("Error fetching triggers:", error);
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
    fetchTriggers();
    fetchOther();
  }, [showingActionSheet]);

  return (
    <Sheet
      open={showingActionSheet}
      onOpenChange={(open) => setShowingActionSheet(open)}
    >
      <SheetContent side={"bottom"} className="h-4/5 flex flex-col">
        <SheetHeader className="flex flex-row justify-between pr-20">
          <div className="flex flex-col">
            <SheetTitle>Library</SheetTitle>
            <SheetDescription>
              Browse and add components to your workflow
            </SheetDescription>
          </div>

          <div className="mt-4">
            <Button
              variant="outline"
              onClick={() => {
                setAddingJson(true);
              }}
            >
              Add from JSON
            </Button>
          </div>
        </SheetHeader>

        <div className="py-4 flex-grow overflow-hidden">
          <Tabs
            defaultValue="actions"
            className="h-full"
            value={actionSheetMode}
            onValueChange={setActionSheetMode}
          >
            <TabsList>
              <TabsTrigger value="triggers">Triggers</TabsTrigger>
              <TabsTrigger value="actions">Actions</TabsTrigger>
              <TabsTrigger value="other">Other</TabsTrigger>
            </TabsList>

            {addingJson ? (
              <div className="flex flex-col space-y-4 p-2 mt-4">
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
              <>
                <TabsContent value="actions" className="h-full">
                  <ScrollArea className="h-full pr-4 pb-4">
                    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                      {actions &&
                        actions.map((db_action: any) => {
                          let action: Action =
                            db_action.action_template_definition;
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
                              <div
                                className="flex flex-row gap-4 items-center"
                                key={`content-${db_action.action_template_id}`}
                              >
                                <BaseNodeIcon icon={action.icon} />
                                <div className="min-w-0">
                                  <div className="text-lg font-semibold truncate">
                                    {action.label}
                                    {!marketplace && (
                                      <Badge className="ml-2" variant="outline">
                                        Team
                                      </Badge>
                                    )}
                                  </div>
                                  <div className="text-sm font-normal truncate">
                                    {action.description}
                                  </div>
                                </div>
                              </div>
                            </div>
                          );
                        })}
                    </div>
                    <div className="h-12" />
                  </ScrollArea>
                </TabsContent>

                <TabsContent value="triggers" className="h-full">
                  <ScrollArea className="h-full pr-4 pb-4">
                    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                      {triggers &&
                        triggers.map((db_trigger: any) => {
                          let action: Action =
                            db_trigger.action_template_definition;
                          let marketplace: boolean = "featured" in db_trigger;
                          return (
                            <div
                              key={`${db_trigger.action_template_id}-${action.label}`}
                              onClick={() => {
                                changeTrigger(action);
                                setShowingActionSheet(false);
                              }}
                              className="flex flex-col justify-between p-4 border rounded-md border-black cursor-pointer hover:bg-gray-50"
                            >
                              <div
                                className="flex flex-row gap-4 items-center"
                                key={`content-${db_trigger.action_template_id}`}
                              >
                                <BaseNodeIcon icon={action.icon} />
                                <div className="min-w-0">
                                  <div className="text-lg font-semibold truncate">
                                    {action.label}
                                    {!marketplace && (
                                      <Badge className="ml-2" variant="outline">
                                        Team
                                      </Badge>
                                    )}
                                  </div>
                                  <div className="text-sm font-normal truncate">
                                    {action.description}
                                  </div>
                                </div>
                              </div>
                            </div>
                          );
                        })}
                    </div>
                    <div className="h-12" />
                  </ScrollArea>
                </TabsContent>

                <TabsContent value="other" className="h-full">
                  <ScrollArea className="h-full pr-4 pb-4">
                    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                      {other &&
                        other.map((db_other: any) => {
                          let action: Action =
                            db_other.action_template_definition;
                          let marketplace: boolean = "featured" in db_other;
                          return (
                            <div
                              key={`${db_other.action_template_id}-${action.label}`}
                              onClick={() => {
                                addNode(action, { x: 100, y: 300 });
                                setShowingActionSheet(false);
                              }}
                              className="flex flex-col justify-between p-4 border rounded-md border-black cursor-pointer hover:bg-gray-50"
                            >
                              <div
                                className="flex flex-row gap-4 items-center"
                                key={`content-${db_other.action_template_id}`}
                              >
                                <BaseNodeIcon icon={action.icon} />
                                <div className="min-w-0">
                                  <div className="text-lg font-semibold truncate">
                                    {action.label}
                                    {!marketplace && (
                                      <Badge className="ml-2" variant="outline">
                                        Team
                                      </Badge>
                                    )}
                                  </div>
                                  <div className="text-sm font-normal truncate">
                                    {action.description}
                                  </div>
                                </div>
                              </div>
                            </div>
                          );
                        })}
                    </div>
                    <div className="h-12" />
                  </ScrollArea>
                </TabsContent>
              </>
            )}
          </Tabs>
        </div>
      </SheetContent>
    </Sheet>
  );
}
