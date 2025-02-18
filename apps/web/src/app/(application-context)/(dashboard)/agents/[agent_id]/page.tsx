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
import Link from "next/link";
import { Edit, Phone } from "lucide-react";
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@repo/ui/components/ui/tabs";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";
import { AddToolDialog } from "@/components/agents/add-tool-dialog";
import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";
import RemoveToolDialog from "@/components/agents/remove-tool-dialog";

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

const vapi = new Vapi(process.env.NEXT_PUBLIC_VAPI_PUBLIC_KEY!);

export default function AgentPage() {
  const params = useParams();
  const [agent, setAgent] = useState<Agent | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [greeting, setGreeting] = useState("");
  const [systemPrompt, setSystemPrompt] = useState("");
  const [isDirty, setIsDirty] = useState(false);
  const [isSaving, setIsSaving] = useState(false);
  const [isCallActive, setIsCallActive] = useState(false);
  const [agentTools, setAgentTools] = useState<any[]>([]);
  const [addToolOpen, setAddToolOpen] = useState(false);

  const {
    accounts: { selectedAccount },
  } = useAnything();
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

      // TODO: get tools taht are connected to this.
      const tools = await api.agents.getAgentTools(
        await createClient(),
        selectedAccount.account_id,
        params.agent_id as string,
      );
      setAgentTools(tools);
    } catch (error) {
      console.error("Error fetching agent:", error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchAgent();
  }, [selectedAccount, params.agent_id]);

  const handleAddTool = async (toolId: string) => {
    console.log("Adding tool:", toolId);

    if (!selectedAccount || !agent) {
      console.error("No account or agent selected");
      return;
    }

    try {
      let res = await api.agents.addToolToAgent(
        await createClient(),
        selectedAccount.account_id,
        agent.agent_id,
        toolId,
      );
      fetchAgent();
    } catch (error) {
      console.error("Error adding tool:", error);
    }
  };

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
      <div className="mb-4 flex justify-between items-center">
        <div>
          <div className="flex items-center gap-3">
            <h1 className="text-3xl font-bold tracking-tight">
              {agent.agent_name}
            </h1>
            <span
              className={`px-2 py-1 text-xs font-medium rounded-full ${
                agent.active
                  ? "bg-green-100 text-green-800"
                  : "bg-gray-100 text-gray-800"
              }`}
            >
              {agent.active ? "Active" : "Inactive"}
              {agent.archived ? " (Archived)" : ""}
            </span>
          </div>
          <div className="flex items-center gap-2 mt-2">
            <p className="text-muted-foreground">
              Manage and configure your voice agent
            </p>
            <span className="text-muted-foreground">•</span>
            <p className="text-sm text-muted-foreground">
              Created {new Date(agent.created_at).toLocaleDateString()}
            </p>
          </div>
        </div>
        <div className="flex gap-1">
          {isDirty && (
            <Button onClick={handleSave} disabled={isSaving}>
              {isSaving ? "Saving..." : "Save Changes"}
            </Button>
          )}
          <Button
            variant="outline"
            onClick={toggleCall}
            className={
              isCallActive
                ? "bg-red-500 hover:bg-red-600 text-white"
                : "bg-green-500 hover:bg-green-600 text-white"
            }
          >
            <Phone className="w-4 h-4 mr-2" />
            {isCallActive ? "Stop call" : "Start call"}
          </Button>
        </div>
      </div>

      <Tabs defaultValue="prompts" className="flex flex-col h-full">
        <TabsList className="mb-2 w-fit">
          <TabsTrigger value="prompts">Prompts</TabsTrigger>
          <TabsTrigger value="tools">Tools</TabsTrigger>
          <TabsTrigger value="channels">Channels</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
        </TabsList>

        <TabsContent value="prompts" className="h-full">
          <ScrollArea>
            <Card>
              <CardHeader>
                <CardTitle>Agent Prompts</CardTitle>
                <CardDescription>
                  Configure your agent's greeting and system prompt
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
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
                      rows={15}
                    />
                  </div>
                </div>
              </CardContent>
            </Card>
          </ScrollArea>
        </TabsContent>

        <TabsContent value="tools" className="h-full">
          <ScrollArea>
            <Card>
              <CardHeader className="flex flex-row items-center justify-between">
                <div>
                  <CardTitle>Agent Tools</CardTitle>
                  <CardDescription>
                    Configure the tools available to your agent
                  </CardDescription>
                </div>
                <Button
                  variant="outline"
                  onClick={() => setAddToolOpen(true)}
                  className="ml-auto"
                >
                  Add Tool
                </Button>
              </CardHeader>
              <CardContent>
                {agentTools.length === 0 ? (
                  <div className="flex flex-col items-center justify-center py-8">
                    <p className="text-muted-foreground text-center mb-4">
                      No tools configured yet. Add tools to enhance your agent's
                      capabilities.
                    </p>
                    <Button
                      variant="outline"
                      onClick={() => setAddToolOpen(true)}
                    >
                      Add a tool to your agent
                    </Button>
                  </div>
                ) : (
                  <div className="grid gap-4">
                    {agentTools.map((tool) => (
                      <Card key={tool.flow_id} className="mt-2 flex flex-row hover:border-green-500">
                        <div className="flex-1 flex">
                          <CardHeader className="w-1/4">
                            <CardTitle className="truncate leading-tight">
                              {tool.flow.flow_name}
                            </CardTitle>
                            <CardDescription className="truncate">
                              {tool.flow.description}
                            </CardDescription>
                          </CardHeader>
                          <CardContent className="flex-1">
                            <div className="flex flex-row h-full items-end">
                              <div className="flex-1" />
                              <div className="flex gap-2">
                                <Link href={`/workflows/${tool.flow.flow_id}`}>
                                  <Button>
                                    <Edit size={16} />
                                  </Button>
                                </Link>
                                <RemoveToolDialog
                                  agentId={params.agent_id as string}
                                  toolId={tool.flow_id}
                                  onRemove={() => {
                                    // Refresh the tools list
                                    fetchAgent();
                                  }}
                                />
                              </div>
                            </div>
                          </CardContent>
                        </div>
                      </Card>
                    ))}
                  </div>
                )}
              </CardContent>
            </Card>
          </ScrollArea>
          <AddToolDialog
            open={addToolOpen}
            onOpenChange={setAddToolOpen}
            onToolAdd={handleAddTool}
          />
        </TabsContent>

        <TabsContent value="channels" className="h-full">
          <ScrollArea>
            <Card>
              <CardHeader>
                <CardTitle>Agent Channels</CardTitle>
                <CardDescription>
                  Manage the channels your agent can communicate through
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="text-muted-foreground">
                  Channel configuration coming soon
                </div>
              </CardContent>
            </Card>
          </ScrollArea>
        </TabsContent>

        <TabsContent value="settings" className="h-full">
          <ScrollArea>
            <Card>
              <CardHeader>
                <CardTitle>Agent Settings</CardTitle>
                <CardDescription>
                  Manage your agent's settings and configuration
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <div>
                    <h3 className="font-medium text-red-600">Danger Zone</h3>
                    <p className="text-sm text-muted-foreground mt-1 mb-4">
                      Actions here cannot be undone
                    </p>
                    <DeleteAgentDialog agentId={agent.agent_id} />
                  </div>
                </div>
              </CardContent>
            </Card>
          </ScrollArea>
        </TabsContent>
      </Tabs>
    </div>
  );
}
