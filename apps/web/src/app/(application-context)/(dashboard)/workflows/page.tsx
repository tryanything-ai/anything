"use client";
import ManageWorkflows from "@/components/workflows/manage-workflows";
import DashboardTitleWithAction from "@/components/workflows/dashboard-title-with-action";
import { Separator } from "@repo/ui/components/ui/separator";
import api from "@repo/anything-api";
import { useRouter } from "next/navigation";
import { useAnything } from "@/context/AnythingContext";
import { useState } from "react";
import NewWorkflowDialog from "@/components/dashboard/new-workflow-dialog";
import { createClient } from "@/lib/supabase/client";
export default function Workflows(): JSX.Element {
  const router = useRouter();
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const {
    accounts: { selectedAccount },
  } = useAnything();
  const createWorkflow = async (name: string, description: string) => {
    if (!selectedAccount) {
      console.error("No account selected");
      return;
    }

    if (!name || name.trim() === "") {
      console.error("Workflow name cannot be empty");
      return;
    }

    try {
      let res = await api.flows.createFlow(
        await createClient(),
        selectedAccount.account_id,
        name.trim(),
        description.trim(),
      );
      console.log("created workflow", res);
      setShowCreateDialog(false);
      router.push(
        `/workflows/${res.workflow_id}/${res.workflow_version_id}/editor`,
      );
    } catch (error) {
      console.error("error creating workflow", error);
    }
  };

  const exploreTemplates = () => {
    window.open("https://www.tryanything.xyz/templates/workflows", "_blank");
  };

  return (
    <div className="space-y-6 w-full">
      <DashboardTitleWithAction
        title="Workflows"
        description="Manage workflows."
        actions={[
          {
            label: "Create New Workflow",
            onClick: () => setShowCreateDialog(true),
          },
          // { label: "Explore Templates", onClick: exploreTemplates },
        ]}
      />
      <Separator />
      <ManageWorkflows />
      <NewWorkflowDialog
        open={showCreateDialog}
        onOpenChange={setShowCreateDialog}
        onCreateWorkflow={createWorkflow}
      />
    </div>
  );
}
