"use client";

import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@repo/ui/components/ui/dialog";
import { Button } from "@repo/ui/components/ui/button";
import { Label } from "@repo/ui/components/ui/label";
import { Input } from "@repo/ui/components/ui/input";
import api from "@repo/anything-api";
import { useRouter } from "next/navigation";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";

interface NewAgentDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  // onCreateAgent: (name: string) => Promise<void>;
}

export default function NewAgentDialog({
  open,
  onOpenChange,
  // onCreateAgent,
}: NewAgentDialogProps) {
  const router = useRouter();
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const [name, setName] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const handleCreateAgent = async () => {
    if (!selectedAccount) {
      console.error("No account selected");
      return;
    }

    if (!name.trim()) {
      console.error("Agent name cannot be empty");
      return;
    }

    setIsLoading(true);
    try {
      const client = await createClient();
      const newAgent = await api.agents.createAgent(
        client,
        selectedAccount.account_id,
        name,
      );

      onOpenChange(false);
      router.push(`/agents/${newAgent.agent_id}`);
    } catch (error) {
      console.error("Error creating agent:", error);
      alert("Failed to create agent");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="w-1/2 max-w-none">
        <DialogHeader>
          <DialogTitle>Create New Agent</DialogTitle>
          <DialogDescription>
            Create a new AI agent to help with your tasks
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col space-y-4 p-4">
          <div className="space-y-2">
            <Label htmlFor="name">Agent Name</Label>
            <Input
              id="name"
              placeholder="Enter agent name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              disabled={isLoading}
            />
          </div>

          <div className="flex justify-center">
            <Button
              size="lg"
              onClick={handleCreateAgent}
              disabled={!name.trim() || isLoading}
            >
              {isLoading ? (
                <>
                  <span className="animate-spin mr-2">⏳</span>
                  Creating...
                </>
              ) : (
                "Create Agent"
              )}
            </Button>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
