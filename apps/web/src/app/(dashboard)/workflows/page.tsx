"use client";
import ManageWorkflows from "@/components/workflows/manage-workflows";
import DashboardTitleWithAction from "@/components/workflows/dashboard-title-with-action";
import { Separator } from "@repo/ui/components/ui/separator";
import api from "@/lib/anything-api";
import { useRouter } from "next/navigation";

export default function Workflows(): JSX.Element {
  const router = useRouter();

  const createWorkflow = async () => {
    try {
      let res = await api.flows.createFlow();
      console.log("created workflow", res);
      router.push(
        `/workflows/${res.workflow_id}/${res.workflow_version_id}/editor`,
      );
    } catch (error) {
      console.error("error creating workflow", error);
    }
  };
  return (
    <div className="space-y-6 w-full">
      <DashboardTitleWithAction
        title="Workflows"
        description="Manage workflows."
        action={createWorkflow}
      />
      <Separator />
      <ManageWorkflows />
    </div>
  );
}
