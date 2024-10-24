"use client";
import ManageWorkflows from "@/components/workflows/manage-workflows";
import DashboardTitleWithAction from "@/components/workflows/dashboard-title-with-action";
import { Separator } from "@repo/ui/components/ui/separator";
import api from "@repo/anything-api";
import { useRouter } from "next/navigation";
import { useAnything } from "@/context/AnythingContext";

export default function Workflows(): JSX.Element {
  const router = useRouter();
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const createWorkflow = async () => {
    if (!selectedAccount) {
      console.error("No account selected");
      return;
    }

    try {
      let res = await api.flows.createFlow(selectedAccount.account_id);
      console.log("created workflow", res);
      router.push(
        `/workflows/${res.workflow_id}/${res.workflow_version_id}/editor`,
      );
    } catch (error) {
      console.error("error creating workflow", error);
    }
  };

  const exploreTemplates = () => {
    window.open("https://tryanything.xyz/templates/workflows", "_blank");
  };

  return (
    <div className="space-y-6 w-full">
      <DashboardTitleWithAction
        title="Workflows"
        description="Manage workflows."
        actions={[
          { label: "Create New Workflow", onClick: createWorkflow },
          { label: "Explore Templates", onClick: exploreTemplates },
        ]}
      />
      <Separator />
      <ManageWorkflows />
    </div>
  );
}
