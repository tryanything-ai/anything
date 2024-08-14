"use client";
import ManageWorkflows from "@/components/workflows/manage-workflows";
import DashboardTitleWithAction from "@/components/workflows/dashboard-title-with-action";
import { Separator } from "@repo/ui/components/ui/separator";
import { useAnything } from "@/context/AnythingContext";

export default function Workflows(): JSX.Element {
  const { workflows } = useAnything();

  const createWorkflow = async () => {
    try {
      let res = await workflows.createWorkflow();
      console.log("created workflow", res);
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
