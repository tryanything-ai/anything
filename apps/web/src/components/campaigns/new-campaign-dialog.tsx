import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@repo/ui/components/ui/dialog";
import { Button } from "@repo/ui/components/ui/button";
import { Input } from "@repo/ui/components/ui/input";
import { Label } from "@repo/ui/components/ui/label";
import { Textarea } from "@repo/ui/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@repo/ui/components/ui/select";
import { createClient } from "@/lib/supabase/client";
import api from "@repo/anything-api";
import { useAnything } from "@/context/AnythingContext";

interface NewCampaignDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export default function NewCampaignDialog({
  open,
  onOpenChange,
}: NewCampaignDialogProps) {
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [selectedAgentId, setSelectedAgentId] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [agents, setAgents] = useState<any[]>([]);
  const [isLoadingAgents, setIsLoadingAgents] = useState(false);
  const router = useRouter();
  const {
    accounts: { selectedAccount },
  } = useAnything();

  // Load agents when dialog opens
  useEffect(() => {
    if (open && selectedAccount) {
      loadAgents();
    }
  }, [open, selectedAccount]);

  const loadAgents = async () => {
    if (!selectedAccount) return;

    try {
      setIsLoadingAgents(true);
      const fetchedAgents = await api.agents.getAgents(
        await createClient(),
        selectedAccount.account_id,
      );
      setAgents(fetchedAgents);
    } catch (error) {
      console.error("Error loading agents:", error);
      alert("Failed to load agents. Please try again.");
    } finally {
      setIsLoadingAgents(false);
    }
  };

  const handleCreateCampaign = async () => {
    if (!selectedAccount) return;

    if (!name.trim()) {
      alert("Please enter a campaign name");
      return;
    }

    if (!selectedAgentId) {
      alert("Please select an agent for this campaign");
      return;
    }

    try {
      setIsLoading(true);
      const campaign = await api.campaigns.createCampaign(
        await createClient(),
        selectedAccount.account_id,
        {
          name,
          description,
          agent_id: selectedAgentId,
        },
      );

      alert("Campaign created successfully");
      onOpenChange(false);
      router.push(`/campaigns/${campaign.campaign_id}`);
    } catch (error) {
      console.error("Error creating campaign:", error);
      alert("Failed to create campaign. Please try again.");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Create New Campaign</DialogTitle>
          <DialogDescription>
            Create a new outbound voice campaign to call your customer list.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <div className="grid gap-2">
            <Label htmlFor="name">Campaign Name</Label>
            <Input
              id="name"
              placeholder="Enter campaign name"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>
          <div className="grid gap-2">
            <Label htmlFor="description">Description (optional)</Label>
            <Textarea
              id="description"
              placeholder="Enter campaign description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              rows={3}
            />
          </div>
          <div className="grid gap-2">
            <Label htmlFor="agent">Select Agent</Label>
            <Select value={selectedAgentId} onValueChange={setSelectedAgentId}>
              <SelectTrigger id="agent">
                <SelectValue placeholder="Select an agent" />
              </SelectTrigger>
              <SelectContent>
                {isLoadingAgents ? (
                  <SelectItem value="loading" disabled>
                    Loading agents...
                  </SelectItem>
                ) : agents.length === 0 ? (
                  <SelectItem value="none" disabled>
                    No agents available
                  </SelectItem>
                ) : (
                  agents.map((agent) => (
                    <SelectItem key={agent.agent_id} value={agent.agent_id}>
                      {agent.agent_name}
                    </SelectItem>
                  ))
                )}
              </SelectContent>
            </Select>
          </div>
        </div>
        <DialogFooter>
          <Button
            type="submit"
            onClick={handleCreateCampaign}
            disabled={isLoading}
          >
            {isLoading ? "Creating..." : "Create Campaign"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
