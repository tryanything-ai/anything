"use client";

import { useEffect, useState } from "react";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from "@repo/ui/components/ui/sheet";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";
import { Badge } from "@repo/ui/components/ui/badge";
import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";
import api from "@repo/anything-api";
import { useParams, useRouter } from "next/navigation";
import { Button } from "@repo/ui/components/ui/button";
import { Loader2 } from "lucide-react";
import NewToolDialog from "@/components/agents/new-tool-dialog";

interface AddToolDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onToolAdd: (toolId: string) => Promise<void>;
}

export function AddToolDialog({ open, onOpenChange, onToolAdd }: AddToolDialogProps): JSX.Element {
  const {
    accounts: { selectedAccount },
  } = useAnything();
  const router = useRouter();

  const [tools, setTools] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isAddingTool, setIsAddingTool] = useState(false);
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [isCreatingTool, setIsCreatingTool] = useState(false);
  const [loadingToolId, setLoadingToolId] = useState<string | null>(null);
  const params = useParams<{ agentId: string }>();

  const createTool = async (name: string, description: string) => {
    if (!selectedAccount) {
      console.error("No account selected");
      return;
    }

    if (!name || name.trim() === "") {
      console.error("Workflow name cannot be empty");
      return;
    }

    setIsCreatingTool(true);
    try {
      let res = await api.flows.createFlow(
        await createClient(),
        selectedAccount.account_id,
        name.trim(),
        description.trim(),
        "tool"
      );
      console.log("created workflow", res);
      setShowCreateDialog(false);
      router.push(
        `/workflows/${res.workflow_id}/${res.workflow_version_id}/editor`,
      );
    } catch (error) {
      console.error("error creating workflow", error);
    } finally {
      setIsCreatingTool(false);
    }
  };

  const fetchTools = async () => {
    setIsLoading(true);
    try {
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      const res = await api.flows.getToolFlows(
        await createClient(),
        selectedAccount.account_id
      );
      console.log("tools res:", res);
      if (res && Array.isArray(res)) {
        setTools(res);
      }
    } catch (error) {
      console.error("Error fetching tools:", error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    if (open) {
      fetchTools();
    }
  }, [open, selectedAccount]);

  const handleToolClick = async (toolId: string) => {
    setIsAddingTool(true);
    setLoadingToolId(toolId);
    try {
      await onToolAdd(toolId);
      onOpenChange(false);
    } catch (error) {
      console.error("Error adding tool:", error);
    } finally {
      setIsAddingTool(false);
      setLoadingToolId(null);
    }
  };

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent side={"bottom"} className="h-4/5 flex flex-col">
        <SheetHeader className="flex flex-row justify-between pr-20">
          <div className="flex flex-col">
            <SheetTitle>Add Tool</SheetTitle>
            <SheetDescription>
              Browse and add tools to your agent
            </SheetDescription>
          </div>
        </SheetHeader>

        <div className="py-4 flex-grow overflow-hidden">
          <ScrollArea className="h-full pr-4 pb-4">
            {isLoading ? (
              <div className="flex items-center justify-center h-full">
                <Loader2 className="w-6 h-6 animate-spin" />
              </div>
            ) : tools.length === 0 ? (
              <div className="flex flex-col items-center justify-center h-full space-y-4">
                <p className="text-muted-foreground text-center">
                  No tools available. Create your first tool to get started.
                </p>
                <Button 
                  onClick={() => setShowCreateDialog(true)}
                  disabled={isCreatingTool}
                >
                  {isCreatingTool ? (
                    <>
                      <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                      Creating Tool...
                    </>
                  ) : (
                    "Create Your First Tool"
                  )}
                </Button>
              </div>
            ) : (
              <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                {tools.map((tool: any) => {
                  let marketplace: boolean = "featured" in tool;
                  const isLoading = loadingToolId === tool.flow_id;
                  return (
                    <div
                      key={`${tool.flow_id}`}
                      onClick={() => !isAddingTool && handleToolClick(tool.flow_id)}
                      className={`flex flex-col justify-between p-4 border rounded-md border-black 
                        ${isAddingTool ? 'cursor-not-allowed opacity-50' : 'cursor-pointer hover:bg-gray-50'}`}
                    >
                      <div
                        className="flex flex-row gap-4 items-center"
                        key={`content-${tool.flow_id}`}
                      >
                        {isLoading ? (
                          <Loader2 className="w-6 h-6 animate-spin" />
                        ) : (
                          <BaseNodeIcon icon={tool.icon || "tool"} />
                        )}
                        <div className="min-w-0">
                          <div className="text-lg font-semibold truncate">
                            {tool.name}
                            {!marketplace && (
                              <Badge className="ml-2" variant="outline">
                                Team
                              </Badge>
                            )}
                          </div>
                          <div className="text-sm font-normal truncate">
                            {tool.description}
                          </div>
                        </div>
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
            <div className="h-12" />
          </ScrollArea>
        </div>
      </SheetContent>
      <NewToolDialog
        open={showCreateDialog}
        onOpenChange={setShowCreateDialog}
        onCreateTool={createTool}
      />
    </Sheet>
  );
}
