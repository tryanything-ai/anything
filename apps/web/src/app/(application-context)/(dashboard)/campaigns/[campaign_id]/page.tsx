"use client";

import { useEffect, useState } from "react";
import { useParams, useRouter } from "next/navigation";
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
import Link from "next/link";
import {
  AlertCircle,
  Edit,
  FileUp,
  Loader2,
  Pause,
  Phone,
  Play,
  Plus,
  Users,
  Clock,
} from "lucide-react";
import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@repo/ui/components/ui/tabs";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";
import {
  Alert,
  AlertDescription,
  AlertTitle,
} from "@repo/ui/components/ui/alert";
import DeleteCampaignDialog from "@/components/campaigns/delete-campaign-dialog";
// import { Progress } from "@repo/ui/components/ui/progress";
import { Label } from "@repo/ui/components/ui/label";
import { UploadContactsListDialog } from "@/components/campaigns/upload-contacts-list-dialog";

interface Campaign {
  campaign_id: string;
  campaign_name: string;
  description: string;
  status: string;
  agent_id: string;
  created_at: string;
  updated_at: string;
  schedule_days_of_week: string[];
  schedule_start_time: string;
  schedule_end_time: string;
  timezone: string;
  agents: {
    agent_id: string;
    agent_name: string;
  };
  campaign_contacts: CampaignContact[];
  campaign_stats?: {
    total_calls: number;
    completed_calls: number;
    in_progress_calls: number;
    failed_calls: number;
  };
}

interface CampaignContact {
  campaign_contact_id: string;
  campaign_id: string;
  contact_id: string;
  account_id: string;
  status: string;
  active: boolean;
  archived: boolean;
  created_at: string;
  updated_at: string;
  contacts: {
    contact_id: string;
    first_name: string;
    last_name: string;
    email: string;
    phone: string;
    company: string;
    title: string;
    address: string;
    city: string;
    state: string;
    postal_code: string;
    country: string;
    status: string;
    source: string;
    notes: string;
    tags: string[];
    custom_fields: any;
    archived: boolean;
    created_at: string;
    updated_at: string;
  };
}

export default function CampaignPage() {
  const params = useParams();
  const router = useRouter();
  const [campaign, setCampaign] = useState<Campaign | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [contacts, setContacts] = useState<CampaignContact[]>([]);
  const [contactsLoading, setContactsLoading] = useState(false);
  const [uploadDialogOpen, setUploadDialogOpen] = useState(false);
  const [isUpdatingStatus, setIsUpdatingStatus] = useState(false);

  const {
    accounts: { selectedAccount },
  } = useAnything();

  const fetchCampaign = async () => {
    if (!selectedAccount || !params.campaign_id) return;

    try {
      setIsLoading(true);
      const data = await api.campaigns.getCampaign(
        await createClient(),
        selectedAccount.account_id,
        params.campaign_id as string,
      );
      setCampaign(data);

      // Fetch contacts for this campaign
      await fetchContacts();
    } catch (error) {
      console.error("Error fetching campaign:", error);
    } finally {
      setIsLoading(false);
    }
  };

  const fetchContacts = async () => {
    if (!selectedAccount || !params.campaign_id) return;

    try {
      setContactsLoading(true);
      const data = await api.campaigns.getCampaignContacts(
        await createClient(),
        selectedAccount.account_id,
        params.campaign_id as string,
      );
      setContacts(data);
    } catch (error) {
      console.error("Error fetching contacts:", error);
    } finally {
      setContactsLoading(false);
    }
  };

  useEffect(() => {
    fetchCampaign();
  }, [selectedAccount, params.campaign_id]);

  const handleToggleCampaignStatus = async () => {
    if (!selectedAccount || !campaign) return;

    try {
      setIsUpdatingStatus(true);
      const newStatus = campaign.status === "active" ? "paused" : "active";

      await api.campaigns.updateCampaignStatus(
        await createClient(),
        selectedAccount.account_id,
        campaign.campaign_id,
        newStatus,
      );

      // Refresh campaign data
      fetchCampaign();
    } catch (error) {
      console.error("Error updating campaign status:", error);
    } finally {
      setIsUpdatingStatus(false);
    }
  };

  // Add computed properties for agent and contact count
  const agent = campaign?.agents; // Get the first agent from the agents array
  const contactCount = contacts.length;

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-[200px]">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
      </div>
    );
  }

  if (!campaign) {
    return <div>Campaign not found</div>;
  }

  const completionPercentage = campaign.campaign_stats
    ? Math.round(
        (campaign.campaign_stats.completed_calls /
          campaign.campaign_stats.total_calls) *
          100,
      )
    : 0;

  return (
    <div className="py-6 px-6">
      <div className="mb-4 flex justify-between items-center">
        <div>
          <div className="flex items-center gap-3">
            <h1 className="text-3xl font-bold tracking-tight">
              {campaign.campaign_name}
            </h1>
            <span
              className={`px-2 py-1 text-xs font-medium rounded-full ${
                campaign.status === "active"
                  ? "bg-green-100 text-green-800"
                  : campaign.status === "paused"
                    ? "bg-amber-100 text-amber-800"
                    : "bg-gray-100 text-gray-800"
              }`}
            >
              {campaign.status === "active"
                ? "Active"
                : campaign.status === "paused"
                  ? "Paused"
                  : campaign.status}
            </span>
          </div>
          <div className="flex items-center gap-2 mt-2">
            <p className="text-muted-foreground">
              {campaign.description || "Outbound voice campaign"}
            </p>
            <span className="text-muted-foreground">•</span>
            <p className="text-sm text-muted-foreground">
              Created {new Date(campaign.created_at).toLocaleDateString()}
            </p>
          </div>
        </div>
        <div className="flex gap-2">
          <Button
            variant="outline"
            onClick={handleToggleCampaignStatus}
            disabled={isUpdatingStatus}
            className={
              campaign.status === "active"
                ? "bg-amber-500 hover:bg-amber-600 text-white"
                : "bg-green-500 hover:bg-green-600 text-white"
            }
          >
            {isUpdatingStatus ? (
              <Loader2 className="w-4 h-4 mr-2 animate-spin" />
            ) : campaign.status === "active" ? (
              <Pause className="w-4 h-4 mr-2" />
            ) : (
              <Play className="w-4 h-4 mr-2" />
            )}
            {campaign.status === "active" ? "Pause Campaign" : "Start Campaign"}
          </Button>
        </div>
      </div>

      <Tabs defaultValue="overview" className="flex flex-col h-full">
        <TabsList className="mb-2 w-fit">
          <TabsTrigger value="overview">Overview</TabsTrigger>
          <TabsTrigger value="contacts">Contacts</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
        </TabsList>

        <TabsContent value="overview" className="h-full">
          <ScrollArea>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <Card>
                <CardHeader>
                  <CardTitle>Campaign Progress</CardTitle>
                  <CardDescription>
                    Track the progress of your outbound campaign
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  {contactCount > 0 ? (
                    <div className="space-y-4">
                      <div>
                        <div className="flex justify-between mb-2">
                          <Label>Campaign Completion</Label>
                          <span className="text-sm font-medium">
                            {completionPercentage}%
                          </span>
                        </div>
                        {/* <Progress
                          value={completionPercentage}
                          className="h-2"
                        /> */}
                      </div>

                      <div className="grid grid-cols-2 gap-4 mt-4">
                        <div className="bg-muted rounded-lg p-3">
                          <div className="text-sm text-muted-foreground">
                            Total Contacts
                          </div>
                          <div className="text-2xl font-bold">
                            {contactCount}
                          </div>
                        </div>
                        <div className="bg-muted rounded-lg p-3">
                          <div className="text-sm text-muted-foreground">
                            Completed Calls
                          </div>
                          <div className="text-2xl font-bold">
                            {campaign.campaign_stats?.completed_calls || 0}
                          </div>
                        </div>
                        <div className="bg-muted rounded-lg p-3">
                          <div className="text-sm text-muted-foreground">
                            In Progress
                          </div>
                          <div className="text-2xl font-bold">
                            {campaign.campaign_stats?.in_progress_calls || 0}
                          </div>
                        </div>
                        <div className="bg-muted rounded-lg p-3">
                          <div className="text-sm text-muted-foreground">
                            Failed Calls
                          </div>
                          <div className="text-2xl font-bold">
                            {campaign.campaign_stats?.failed_calls || 0}
                          </div>
                        </div>
                      </div>
                    </div>
                  ) : (
                    <div className="flex flex-col items-center justify-center py-6">
                      <AlertCircle className="h-12 w-12 text-amber-500 mb-4" />
                      <h3 className="text-lg font-medium mb-2">No Contacts</h3>
                      <p className="text-sm text-muted-foreground text-center mb-4">
                        You need to upload contacts to start this campaign.
                      </p>
                      <Button onClick={() => setUploadDialogOpen(true)}>
                        <FileUp className="w-4 h-4 mr-2" />
                        Upload Contacts
                      </Button>
                    </div>
                  )}
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle>Campaign Details</CardTitle>
                  <CardDescription>
                    Information about this campaign
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="space-y-4">
                    {agent && (
                      <div>
                        <Label className="text-muted-foreground mb-1 block">
                          Agent
                        </Label>
                        <div className="flex items-center">
                          <div className="px-3 py-2 rounded-md border flex items-center gap-2 w-full">
                            <Phone className="w-4 h-4 text-blue-500" />
                            <span>{agent.agent_name}</span>
                          </div>
                          <Link
                            href={`/agents/${agent[0]?.agent_id}`}
                            className="ml-2"
                          >
                            <Button variant="outline" size="icon">
                              <Edit className="h-4 w-4" />
                            </Button>
                          </Link>
                        </div>
                      </div>
                    )}

                    <div>
                      <Label className="text-muted-foreground mb-1 block">
                        Call Schedule
                      </Label>
                      <div className="px-3 py-2 rounded-md border">
                        <div className="flex flex-col gap-2">
                          <div className="flex items-center gap-2">
                            <Clock className="w-4 h-4 text-orange-500" />
                            <span>
                              {campaign.schedule_start_time?.substring(0, 5)} -{" "}
                              {campaign.schedule_end_time?.substring(0, 5)}
                            </span>
                            <span className="text-sm text-muted-foreground">
                              ({campaign.timezone})
                            </span>
                          </div>
                          <div className="flex flex-wrap gap-1 mt-1">
                            {campaign.schedule_days_of_week?.map((day) => (
                              <span
                                key={day}
                                className="px-2 py-1 text-xs font-medium rounded-full bg-blue-100 text-blue-800"
                              >
                                {day.substring(0, 3)}
                              </span>
                            ))}
                          </div>
                        </div>
                      </div>
                    </div>

                    {contactCount > 0 ? (
                      <div>
                        <Label className="text-muted-foreground mb-1 block">
                          Contacts
                        </Label>
                        <div className="flex items-center">
                          <div className="px-3 py-2 rounded-md border flex items-center gap-2 w-full">
                            <Users className="w-4 h-4 text-purple-500" />
                            <span>Campaign Contacts</span>
                            <span className="text-sm text-muted-foreground ml-auto">
                              {contactCount} contacts
                            </span>
                          </div>
                          <Button
                            variant="outline"
                            size="icon"
                            className="ml-2"
                            onClick={() => setUploadDialogOpen(true)}
                          >
                            <FileUp className="h-4 w-4" />
                          </Button>
                        </div>
                      </div>
                    ) : (
                      <Alert variant="destructive">
                        <AlertCircle className="h-4 w-4" />
                        <AlertTitle>No contacts</AlertTitle>
                        <AlertDescription>
                          Upload contacts to start making calls.
                        </AlertDescription>
                      </Alert>
                    )}
                  </div>
                </CardContent>
              </Card>
            </div>
          </ScrollArea>
        </TabsContent>

        <TabsContent value="contacts" className="h-full">
          <ScrollArea>
            <Card>
              <CardHeader className="flex flex-row items-center justify-between">
                <div>
                  <CardTitle>Contacts</CardTitle>
                  <CardDescription>
                    Manage the contacts in your campaign
                  </CardDescription>
                </div>
                <Button onClick={() => setUploadDialogOpen(true)}>
                  <FileUp className="w-4 h-4 mr-2" />
                  Upload Contacts
                </Button>
              </CardHeader>
              <CardContent>
                {contactsLoading ? (
                  <div className="flex justify-center py-8">
                    <Loader2 className="w-6 h-6 animate-spin text-muted-foreground" />
                  </div>
                ) : contacts.length === 0 ? (
                  <div className="text-center py-8">
                    <Users className="w-12 h-12 text-muted-foreground mx-auto mb-4" />
                    <h3 className="text-lg font-medium mb-2">
                      No Contacts Yet
                    </h3>
                    <p className="text-sm text-muted-foreground mb-4">
                      Upload a CSV file with your contacts to get started.
                    </p>
                    <Button onClick={() => setUploadDialogOpen(true)}>
                      <FileUp className="w-4 h-4 mr-2" />
                      Upload Contacts List
                    </Button>
                  </div>
                ) : (
                  <div className="rounded-md border">
                    <div className="grid grid-cols-6 gap-4 p-4 font-medium border-b">
                      <div className="col-span-2">Name</div>
                      <div>Phone</div>
                      <div>Status</div>
                      <div>Attempts</div>
                      <div>Last Call</div>
                    </div>
                    {contacts.map((campaignContact) => (
                      <div
                        key={campaignContact.contact_id}
                        className="grid grid-cols-6 gap-4 p-4 border-b last:border-0"
                      >
                        <div className="col-span-2">
                          {campaignContact.contacts.first_name}{" "}
                          {campaignContact.contacts.last_name}
                          <div className="text-sm text-muted-foreground">
                            {campaignContact.contacts.email}
                          </div>
                        </div>
                        <div>{campaignContact.contacts.phone}</div>
                        <div>
                          <span
                            className={`px-2 py-1 text-xs font-medium rounded-full ${
                              campaignContact.status === "completed"
                                ? "bg-green-100 text-green-800"
                                : campaignContact.status === "in_progress"
                                  ? "bg-blue-100 text-blue-800"
                                  : campaignContact.status === "failed"
                                    ? "bg-red-100 text-red-800"
                                    : "bg-gray-100 text-gray-800"
                            }`}
                          >
                            {campaignContact.status}
                          </span>
                        </div>
                        <div>
                          {campaignContact.contacts.custom_fields
                            ?.call_attempts || 0}
                        </div>
                        <div>
                          {campaignContact.contacts.custom_fields?.last_call_at
                            ? new Date(
                                campaignContact.contacts.custom_fields.last_call_at,
                              ).toLocaleDateString()
                            : "Never"}
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </CardContent>
            </Card>
          </ScrollArea>
        </TabsContent>

        <TabsContent value="settings" className="h-full">
          <ScrollArea>
            <Card>
              <CardHeader>
                <CardTitle>Campaign Settings</CardTitle>
                <CardDescription>Manage your campaign settings</CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <div>
                    <Label htmlFor="campaign-name">Campaign Name</Label>
                    <Input
                      id="campaign-name"
                      value={campaign.campaign_name}
                      className="mt-1"
                      disabled
                    />
                  </div>

                  <div>
                    <Label htmlFor="campaign-description">Description</Label>
                    <Textarea
                      id="campaign-description"
                      value={campaign.description || ""}
                      className="mt-1"
                      disabled
                    />
                  </div>

                  <div className="pt-6 border-t">
                    <h3 className="font-medium text-red-600">Danger Zone</h3>
                    <p className="text-sm text-muted-foreground mt-1 mb-4">
                      Actions here cannot be undone
                    </p>
                    <DeleteCampaignDialog campaignId={campaign.campaign_id} />
                  </div>
                </div>
              </CardContent>
            </Card>
          </ScrollArea>
        </TabsContent>
      </Tabs>

      <UploadContactsListDialog
        open={uploadDialogOpen}
        onOpenChange={setUploadDialogOpen}
        accountId={selectedAccount?.account_id || ""}
        campaignId={campaign?.campaign_id || ""}
      />
    </div>
  );
}
