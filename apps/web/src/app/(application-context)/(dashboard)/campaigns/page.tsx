"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
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
import { Phone, Users } from "lucide-react";
import NewCampaignDialog from "@/components/campaigns/new-campaign-dialog";

interface Campaign {
  campaign_id: string;
  campaign_name: string;
  description: string;
  agent_id: string;
  customer_list_id: string;
  status: string;
  created_at: string;
  updated_at: string;
  agent?: {
    agent_name: string;
    agent_communication_channels?: any[];
  };
  customer_list?: {
    list_name: string;
    contact_count: number;
  };
}

export default function CampaignsPage() {
  const [showCreator, setShowCreator] = useState(false);
  const router = useRouter();
  const [campaigns, setCampaigns] = useState<Campaign[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const {
    accounts: { selectedAccount },
  } = useAnything();

  useEffect(() => {
    const fetchCampaigns = async () => {
      if (!selectedAccount) return;

      try {
        setIsLoading(true);
        const fetchedCampaigns = await api.campaigns.getCampaigns(
          await createClient(),
          selectedAccount.account_id,
        );
        setCampaigns(fetchedCampaigns);
      } catch (error) {
        console.error("Error fetching campaigns:", error);
      } finally {
        setIsLoading(false);
      }
    };

    fetchCampaigns();
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
        title="Campaigns"
        description="Manage outbound voice campaigns for your customer lists."
        actions={[
          {
            label: "Create New Campaign",
            onClick: () => setShowCreator(true),
          },
        ]}
      />
      <Separator />
      {campaigns.length === 0 ? (
        <WelcomeToCampaigns setShowCreator={setShowCreator} />
      ) : (
        <div>
          {campaigns.map((campaign) => (
            <Card
              key={campaign.campaign_id}
              className="mt-2 flex flex-row hover:border-green-500"
            >
              <Link
                href={`/campaigns/${campaign.campaign_id}`}
                className="flex-1 flex"
              >
                <CardHeader className="w-1/4">
                  <CardTitle className="truncate leading-tight">
                    {campaign.campaign_name}
                  </CardTitle>
                  <CardDescription className="truncate">
                    <div className="text-sm text-gray-500">
                      Status:{" "}
                      <span
                        className={`font-medium ${campaign.status === "active" ? "text-green-600" : campaign.status === "paused" ? "text-amber-600" : "text-gray-600"}`}
                      >
                        {campaign.status}
                      </span>
                    </div>
                    <div className="text-sm text-gray-500">
                      Last updated:{" "}
                      {new Date(campaign.updated_at).toLocaleDateString()}
                    </div>
                  </CardDescription>
                </CardHeader>
                <CardContent className="flex-1">
                  <div className="flex flex-col h-full justify-center gap-4">
                    <div className="flex gap-4">
                      {campaign.agent && (
                        <div className="flex items-center">
                          <div className="px-2 py-1 font-medium flex items-center gap-2 rounded-full bg-blue-100 text-blue-800">
                            <Phone className="w-4 h-4" />{" "}
                            {campaign.agent.agent_name}
                          </div>
                        </div>
                      )}
                      {campaign.customer_list && (
                        <div className="flex items-center">
                          <div className="px-2 py-1 font-medium flex items-center gap-2 rounded-full bg-purple-100 text-purple-800">
                            <Users className="w-4 h-4" />{" "}
                            {campaign.customer_list.list_name} (
                            {campaign.customer_list.contact_count} contacts)
                          </div>
                        </div>
                      )}
                    </div>
                  </div>
                </CardContent>
              </Link>
            </Card>
          ))}
        </div>
      )}
      <NewCampaignDialog open={showCreator} onOpenChange={setShowCreator} />
    </div>
  );
}

// Welcome component for first-time users
function WelcomeToCampaigns({
  setShowCreator,
}: {
  setShowCreator: (show: boolean) => void;
}) {
  return (
    <div className="flex flex-col items-center justify-center py-12 px-4">
      <div className="text-center max-w-md">
        <h2 className="text-2xl font-bold mb-4">Welcome to Campaigns</h2>
        <p className="text-muted-foreground mb-8">
          Create outbound voice campaigns to automatically call your customer
          lists using your AI agents.
        </p>
        <button
          onClick={() => setShowCreator(true)}
          className="inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 bg-primary text-primary-foreground hover:bg-primary/90 h-10 px-4 py-2"
        >
          Create Your First Campaign
        </button>
      </div>
    </div>
  );
}
