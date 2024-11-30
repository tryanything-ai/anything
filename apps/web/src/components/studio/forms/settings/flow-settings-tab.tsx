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

  //BUG: This is not downloading the latest version of the workflow
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
      {/* <div className="flex flex-row justify-between m-4">
        <div>Detailed Editor</div>
        <Switch
          checked={workflow.detailedMode}
          onCheckedChange={() =>
            workflow.setDetailedMode(!workflow.detailedMode)
          }
        />
      </div> */}

      <div className="flex flex-col gap-4 mx-2">
        <div>
          <div className="text-sm font-medium ml-1">Call Deployed Workflow</div>
          <div className="flex items-center gap-2">
            <code className="flex-1 p-2 bg-muted rounded-md text-sm overflow-x-auto">
              {`${process.env.NEXT_PUBLIC_ANYTHING_API_URL}/api/v1/workflow/${workflow.db_flow_id}/start`}
            </code>
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                navigator.clipboard.writeText(
                  `${process.env.NEXT_PUBLIC_ANYTHING_API_URL}/api/v1/workflow/${workflow.db_flow_id}/start`,
                );
              }}
            >
              Copy
            </Button>
          </div>
        </div>

        <div>
          <div className="text-sm font-medium ml-1">
            Call Specific Version of Workflow
          </div>
          <div className="flex items-center gap-2">
            <code className="flex-1 p-2 bg-muted rounded-md text-sm overflow-x-auto">
              {`${process.env.NEXT_PUBLIC_ANYTHING_API_URL}/api/v1/workflow/${workflow.db_flow_id}/version/${workflow.db_flow_version_id}/start`}
            </code>
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                navigator.clipboard.writeText(
                  `${process.env.NEXT_PUBLIC_ANYTHING_API_URL}/api/v1/workflow/${workflow.db_flow_id}/version/${workflow.db_flow_version_id}/start`,
                );
              }}
            >
              Copy
            </Button>
          </div>
        </div>

        <p className="text-xs text-muted-foreground">
          Use these endpoints to trigger your workflow via API. Send a POST
          request with your data as JSON in the request body.{" "}
          <span className="font-bold underline text-black">
            {" "}
            Workflow must include webhook trigger to be called via API.
          </span>
        </p>
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
