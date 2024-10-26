import { useState } from "react";
import { useAnything } from "@/context/AnythingContext";
import { Label } from "@repo/ui/components/ui/label";
import { Input } from "@repo/ui/components/ui/input";
import { SubmitHandler, useForm } from "react-hook-form";
import { useRouter } from "next/navigation";
import DeleteFlowDialog from "./delete-flow-dialog";
import { Switch } from "@repo/ui/components/ui/switch";
import WorkflowSettingsForm from "./workflow-settings-form";
import { Button } from "@repo/ui/components/ui/button";

type Inputs = {
  flow_name: string;
};

export default function WorkflowSettingsTab(): JSX.Element {
  const { workflow } = useAnything();
  const [loading, setLoading] = useState(false);
  const navigate = useRouter();

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<Inputs>();

  const downloadWorkflowJson = () => {
    const jsonString = JSON.stringify(
      workflow.flow_version_definition,
      null,
      2,
    );
    const blob = new Blob([jsonString], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const link = document.createElement("a");
    link.href = url;
    link.download = `${workflow.db_flow.flow_name || "workflow"}.json`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  return (
    <div className="grid w-full items-start gap-6">
      <WorkflowSettingsForm />
      <div className="flex flex-row justify-between m-4">
        <div>Detailed Editor</div>
        <Switch
          checked={workflow.detailedMode}
          onCheckedChange={() =>
            workflow.setDetailedMode(!workflow.detailedMode)
          }
        />
      </div>

      <div className="absolute bottom-14 w-full mb-2">
        <Button
          variant={"secondary"}
          className="w-full"
          onClick={downloadWorkflowJson}
        >
          Download Workflow as JSON
        </Button>
      </div>
      <div className="absolute bottom-0 w-full mb-2">
        <DeleteFlowDialog workflowId={workflow.db_flow_id} />
      </div>
    </div>
  );
}
