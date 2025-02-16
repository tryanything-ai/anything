"use client";

import { useEffect, useState } from "react";
import { useParams } from "next/navigation";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import { createClient } from "@/lib/supabase/client";
import api from "@repo/anything-api";
import { useAnything } from "@/context/AnythingContext";
import { Button } from "@repo/ui/components/ui/button";
import { Textarea } from "@repo/ui/components/ui/textarea";
import { Input } from "@repo/ui/components/ui/input";
import DeleteAgentDialog from "@/components/agents/delete-agent-dialog";
import Vapi from "@vapi-ai/web";
import { Phone } from "lucide-react";

interface Agent {
  agent_id: string;
  agent_name: string;
  icon: string | null;
  active: boolean;
  archived: boolean;
  config: {
    greeting: string;
    system_prompt: string;
  };
  created_at: string;
  updated_at: string;
}

const vapi = new Vapi(
    process.env.NEXT_PUBLIC_VAPI_PUBLIC_KEY!,
  );

export default function AgentPage() {
  const params = useParams();
  const [agent, setAgent] = useState<Agent | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [greeting, setGreeting] = useState("");
  const [systemPrompt, setSystemPrompt] = useState("");
  const [isDirty, setIsDirty] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [isCallActive, setIsCallActive] = useState(false);
  const {
    accounts: { selectedAccount },
  } = useAnything();

  useEffect(() => {
    const fetchAgent = async () => {
      if (!selectedAccount || !params.agent_id) return;

      try {
        const data = await api.agents.getAgent(
          await createClient(),
          selectedAccount.account_id,
          params.agent_id as string,
        );
        setAgent(data);
        setGreeting(data.config.greeting);
        setSystemPrompt(data.config.system_prompt);
      } catch (error) {
        console.error("Error fetching agent:", error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchAgent();
  }, [selectedAccount, params.agent_id]);

  const handleSave = async () => {
    if (!selectedAccount || !agent) {
      console.error("No account or agent selected");
      return;
    }

    setIsSaving(true);
    try {
      let res = await api.agents.updateAgent(
        await createClient(),
        selectedAccount.account_id,
        agent.agent_id,
        {
          name: agent.agent_name,
          greeting,
          system_prompt: systemPrompt,
        },
      );
      console.log("updated agent", res);
      setIsDirty(false);
    } catch (error) {
      console.error("error updating agent", error);
    } finally {
      setIsSaving(false);
    }
  };

  const toggleCall = () => {
    if (!params.agent_id) return;
    
    if (isCallActive) {
      vapi.stop();
      setIsCallActive(false);
    } else {
      vapi.start(params.agent_id as string);
      setIsCallActive(true);
    }
  };

  const handleGreetingChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setGreeting(e.target.value);
    setIsDirty(true);
  };

  const handleSystemPromptChange = (
    e: React.ChangeEvent<HTMLTextAreaElement>,
  ) => {
    setSystemPrompt(e.target.value);
    setIsDirty(true);
  };

  if (isLoading) {
    return <div>Loading...</div>;
  }

  if (!agent) {
    return <div>Agent not found</div>;
  }

  return (
    <div className="py-6 px-6">
      <div className="mb-8 flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">
            {agent.agent_name}
          </h1>
          <p className="text-muted-foreground mt-2">
            Manage and configure your voice agent
          </p>
        </div>
        <div className="flex gap-2">
          {isDirty && (
            <Button onClick={handleSave} disabled={isSaving}>
              {isSaving ? "Saving..." : "Save Changes"}
            </Button>
          )}
          <Button 
            variant="outline" 
            onClick={toggleCall}
            className={isCallActive ? "bg-red-500 hover:bg-red-600 text-white" : "bg-green-500 hover:bg-green-600 text-white"}
          >
            <Phone className="w-4 h-4 mr-2" />
            {isCallActive ? "Stop call" : "Start call"}
          </Button>
        </div>
      </div>

      <div className="grid gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Agent Configuration</CardTitle>
            <CardDescription>
              Current settings and configuration for your voice agent
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div>
                <h3 className="font-medium">Status</h3>
                <p className="text-sm text-muted-foreground">
                  {agent.active ? "Active" : "Inactive"}
                  {agent.archived ? " (Archived)" : ""}
                </p>
              </div>

              <div>
                <h3 className="font-medium">Greeting Message</h3>
                <Input
                  value={greeting}
                  onChange={handleGreetingChange}
                  className="mt-2"
                />
              </div>

              <div>
                <h3 className="font-medium">System Prompt</h3>
                <Textarea
                  value={systemPrompt}
                  onChange={handleSystemPromptChange}
                  className="mt-2"
                  rows={8}
                />
              </div>

              <div>
                <h3 className="font-medium">Created</h3>
                <p className="text-sm text-muted-foreground">
                  {new Date(agent.created_at).toLocaleDateString()}
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
      <div className="flex justify-end mt-6">
        <DeleteAgentDialog agentId={agent.agent_id} />
      </div>
    </div>
  );
}
