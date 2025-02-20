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
import { Edit, Loader2, Phone, Plus } from "lucide-react";
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@repo/ui/components/ui/tabs";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";
import { AddToolDialog } from "@/components/agents/add-tool-dialog";
import RemoveToolDialog from "@/components/agents/remove-tool-dialog";
import { AddPhoneNumberDialog } from "@/components/agents/add-phone-number-dialog";
import RemovePhoneNumberDialog from "@/components/agents/remove-phone-number-dialog";

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
  const [addPhoneNumberOpen, setAddPhoneNumberOpen] = useState(false);
  const [phoneNumbers, setPhoneNumbers] = useState<any[]>([]);
  const [connectingPhoneNumber, setConnectingPhoneNumber] = useState<
    string | null
  >(null);
  const [removingPhoneNumber, setRemovingPhoneNumber] = useState<string | null>(
    null,
  );
  const [connectedPhoneNumber, setConnectedPhoneNumber] = useState<
    string | null
  >(null);

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

      const phoneNumbers = await api.agents.getAccountPhoneNumbers(
        await createClient(),
        selectedAccount.account_id,
      );
      console.log("phoneNumbers", phoneNumbers);
      setPhoneNumbers(phoneNumbers);

      setConnectedPhoneNumber(
        phoneNumbers.find((number: any) =>
          number.agent_communication_channels.some(
            (channel: any) => channel.agent_id === params.agent_id,
          ),
        )?.phone_number_id || null,
      );
      console.log("connectedPhoneNumber", connectedPhoneNumber);
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

  const handleConnectPhoneToAgent = async (phoneNumberId: string) => {
    console.log("Connecting phone number to agent:", phoneNumberId);

    if (!selectedAccount || !agent) {
      console.error("No account or agent selected");
      return;
    }

    setConnectingPhoneNumber(phoneNumberId);
    try {
      let res = await api.agents.addPhoneNumberToAgent(
        await createClient(),
        selectedAccount.account_id,
        agent.agent_id,
        phoneNumberId,
      );
      fetchAgent();
    } catch (error) {
      console.error("Error connecting phone number to agent:", error);
    } finally {
      setConnectingPhoneNumber(null);
    }
  };



  const handleAddPhoneNumber = async (phoneNumber: string) => {
    console.log("Adding phone number:", phoneNumber);

    if (!selectedAccount || !agent) {
      console.error("No account or agent selected");
      return;
    }

    try {
      let res = await api.agents.buyPhoneNumber(
        await createClient(),
        selectedAccount.account_id,
        phoneNumber,
      );
      console.log("Added phone number:", res);
      fetchAgent();
    } catch (error) {
      console.error("Error adding phone number:", error);
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
          <TabsTrigger value="phone_number">Phone Number</TabsTrigger>
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
                      <Card
                        key={tool.flow_id}
                        className="mt-2 flex flex-row hover:border-green-500"
                      >
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
                            <div className="flex flex-row">
                              <div className="flex-1 flex mt-8">
                                {tool.tool_parameters?.parameters
                                  ?.properties && (
                                  <div className="text-sm w-full">
                                    <div className="mb-2">Inputs:</div>

                                    <div className="flex flex-row gap-4 justify-start">
                                      {Object.entries(
                                        tool.tool_parameters.parameters
                                          .properties,
                                      )
                                        .slice(0, 4)
                                        .map(([key, value]: [string, any]) => (
                                          <div
                                            key={key}
                                            className="flex items-center"
                                          >
                                            <span className="mr-2">{key}</span>
                                            <span className="ml-1 rounded bg-muted px-1.5 py-0.5 text-[0.6rem] font-medium uppercase text-muted-foreground">
                                              {value.type}
                                            </span>
                                          </div>
                                        ))}
                                    </div>
                                  </div>
                                )}
                              </div>
                              <div className="flex justify-end gap-2 mt-8">
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

        <TabsContent value="phone_number" className="h-full">
          <ScrollArea>
            <Card>
              <CardHeader className="flex flex-row items-center justify-between">
                <div>
                  <CardTitle>Phone Number</CardTitle>
                  <CardDescription>
                    Manage the phone number your agent can communicate through
                  </CardDescription>
                </div>
                {phoneNumbers.length === 0 && (
                  <Button onClick={() => setAddPhoneNumberOpen(true)}>
                    <Plus className="w-4 h-4 mr-2" />
                    Get New Phone Number
                  </Button>
                )}
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <div>
                    <h3 className="font-medium">Your Phone Numbers</h3>
                    <p className="text-sm text-muted-foreground mt-1 mb-4">
                      Give your agent a phone number your customers can call
                    </p>

                    {phoneNumbers && phoneNumbers.length > 0 ? (
                      <div className="mb-4">
                        {/* Show connected number first */}
                        {phoneNumbers
                          .filter(
                            (number) =>
                              number.phone_number_id === connectedPhoneNumber,
                          )
                          .map((number: any) => (
                            <div
                              key={number.phone_number_id}
                              className="p-4 border rounded-lg mb-2 flex items-center justify-between bg-muted/50"
                            >
                              <div className="flex items-center gap-2">
                                <div>
                                  <div className="font-medium flex items-center gap-2">
                                    {number.phone_number}
                                    <span className="px-2 py-1 rounded-full text-xs bg-green-100 text-green-800">
                                      Connected
                                    </span>
                                  </div>
                                  <div className="text-sm text-muted-foreground">
                                    {number.locality}
                                  </div>
                                  <div className="text-sm text-muted-foreground">
                                    Created{" "}
                                    {new Date(
                                      number.created_at,
                                    ).toLocaleDateString()}
                                  </div>
                                </div>
                              </div>
                              <RemovePhoneNumberDialog
                                agentId={agent?.agent_id || ""}
                                phoneNumberId={number.phone_number_id}
                                onRemove={fetchAgent}
                              />
                            </div>
                          ))}

                        {/* Show other numbers */}
                        {phoneNumbers
                          .filter(
                            (number) =>
                              number.phone_number_id !== connectedPhoneNumber,
                          )
                          .map((number: any) => {
                            const connectedAgent = number.agent_communication_channels.find(
                              (channel: any) => channel.agent_id !== params.agent_id
                            );
                            
                            return (
                              <div
                                key={number.phone_number_id}
                                className="p-4 border rounded-lg mb-2 flex items-center justify-between"
                              >
                                <div>
                                  <div className="font-medium">
                                    {number.phone_number}
                                  </div>
                                  <div className="text-sm text-muted-foreground">
                                    {number.locality}
                                  </div>
                                  <div className="text-sm text-muted-foreground">
                                    Created{" "}
                                    {new Date(
                                      number.created_at,
                                    ).toLocaleDateString()}
                                  </div>
                                  {connectedAgent && (
                                    <div className="text-sm text-yellow-600 mt-1">
                                      Connected to another agent
                                    </div>
                                  )}
                                </div>
                                <Button
                                  size="sm"
                                  onClick={() =>
                                    handleConnectPhoneToAgent(
                                      number.phone_number_id,
                                    )
                                  }
                                  disabled={
                                    connectingPhoneNumber === number.phone_number_id || connectedAgent
                                  }
                                >
                                  {connectingPhoneNumber === number.phone_number_id ? (
                                    <Loader2 className="w-4 h-4 animate-spin" />
                                  ) : connectedAgent ? (
                                    "In use"
                                  ) : (
                                    "Use this number"
                                  )}
                                </Button>
                              </div>
                            );
                          })}
                      </div>
                    ) : (
                      <div className="text-sm text-muted-foreground mb-4">
                        No phone numbers
                      </div>
                    )}
                  </div>
                </div>
              </CardContent>
            </Card>
          </ScrollArea>
          <AddPhoneNumberDialog
            open={addPhoneNumberOpen}
            onOpenChange={setAddPhoneNumberOpen}
            onPhoneNumberAdd={handleAddPhoneNumber}
          />
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
