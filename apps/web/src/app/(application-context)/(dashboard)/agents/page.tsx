"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import WelcomeToAgents from "@/components/agents/welcome-to-agents";
import api from "@repo/anything-api";
import { createClient } from "@/lib/supabase/client";
import { useAnything } from "@/context/AnythingContext";
import { Separator } from "@repo/ui/components/ui/separator";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import Link from "next/link";
import DashboardTitleWithAction from "@/components/workflows/dashboard-title-with-action";
import NewAgentDialog from "@/components/agents/new-agent-dialog";
import { Phone } from "lucide-react";

interface Agent {
  agent_id: string;
  agent_name: string;
  greeting: string;
  system_prompt: string;
  created_at: string;
  updated_at: string;
  agent_communication_channels?: any[];
}

export default function AgentsPage() {
  const [showCreator, setShowCreator] = useState(false);
  const router = useRouter();
  const [agents, setAgents] = useState<Agent[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const {
    accounts: { selectedAccount },
  } = useAnything();

  useEffect(() => {
    const fetchAgents = async () => {
      if (!selectedAccount) return;

      try {
        setIsLoading(true);
        const fetchedAgents = await api.agents.getAgents(
          await createClient(),
          selectedAccount.account_id,
        );
        setAgents(fetchedAgents);
      } catch (error) {
        console.error("Error fetching agents:", error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchAgents();
  }, [selectedAccount]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-[200px]">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      </div>
    );
  }

  return (
    <div className="space-y-6 w-full">
      <DashboardTitleWithAction
        title="Agents"
        description="Manage agents."
        actions={[
          {
            label: "Create New Agent",
            onClick: () => setShowCreator(true),
          },
          //   { label: "Explore Templates", onClick: exploreTemplates },
        ]}
      />
      <Separator />
      {agents.length === 0 ? (
        <WelcomeToAgents setShowCreator={setShowCreator} />
      ) : (
        <div>
          {agents.map((agent) => (
            <Card
              key={agent.agent_id}
              className="mt-2 flex flex-row hover:border-green-500"
            >
              <Link href={`/agents/${agent.agent_id}`} className="flex-1 flex">
                <CardHeader className="w-1/4">
                  <CardTitle className="truncate leading-tight">
                    {agent.agent_name}
                  </CardTitle>
                  <CardDescription className="truncate">
                    <div className="text-sm text-gray-500">
                      Last updated:{" "}
                      {new Date(agent.updated_at).toLocaleDateString()}
                    </div>
                  </CardDescription>
                </CardHeader>
                <CardContent className="flex-1">
                  <div className="flex flex-col h-full justify-center gap-4">
                    <div className="flex gap-2">
                      {agent.agent_communication_channels?.map(
                        (channel) =>
                          channel.channel_type === "phone" && (
                            <div
                              key={channel.channel_id}
                              className="flex items-center"
                            >
                              <div>
                                <div className=" px-2 py-1  font-medium flex items-center gap-2 rounded-full bg-green-100 text-green-800">
                                  <Phone className="w-4 h-4" />{" "}
                                  {channel.phone_numbers.phone_number.replace(/(\d{1})(\d{3})(\d{3})(\d{4})/, '$1 ($2) $3-$4')}
                             
                                </div>
                              </div>
                            </div>
                          ),
                      )}
                    </div>
                  </div>
                </CardContent>
              </Link>
            </Card>
          ))}
        </div>
      )}
      <NewAgentDialog
        open={showCreator}
        onOpenChange={setShowCreator}
        // onCreateAgent={handleCreateAgent}
      />
    </div>
  );
}
