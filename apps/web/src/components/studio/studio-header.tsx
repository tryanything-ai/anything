"use client";

import { useState } from "react";
import { Button } from "@repo/ui/components/ui/button";
import { useRouter } from "next/navigation";
import { Send, XIcon } from "lucide-react";
import { ShareDialog } from "@/components/studio/share-dialog";
import { useAnything } from "@/context/AnythingContext";
import WorkflowToggle from "../workflows/workflow-toggle";
import { useParams } from "next/navigation";
import { useEffect } from "react";
import api from "@/lib/anything-api";
// flow_name={workflow?.db_flow.flow_name || ""}
// savingStatus={workflow.savingStatus}

export default function StudioHeader(): JSX.Element {
  const router = useRouter();
  const params = useParams<{ workflowVersionId: string; workflowId: string }>();

  const { workflow } = useAnything();

  const [version, setVersion] = useState<any>(null);

  const handleBack = () => {
    router.push(`/workflows/${params.workflowId}`);
  };

  const fetchVersion = async () => {
    try {
      let version = await api.flows.getFlowVersionById(
        params.workflowId,
        params.workflowVersionId,
      );
      setVersion(version);
    } catch (error) {
      console.error(error);
    }
  };

  useEffect(() => {
    if (params.workflowVersionId && params.workflowId) {
      console.log("fetching version", params.workflowVersionId);
      fetchVersion();
    }
  }, [params.workflowVersionId]);

  return (
    <header className="sticky top-0 z-10 flex h-[57px] items-center gap-1 border-b bg-background px-4">
      <div className="border-b p-2">
        <Button
          onClick={handleBack}
          variant="outline"
          size="icon"
          aria-label="Home"
        >
          <XIcon className="size-5 fill-foreground" />
        </Button>
      </div>
      <h1 className="text-xl font-semibold inline">
        {workflow?.db_flow.flow_name || ""}{" "}
        <span className="text-sm font-normal">
          {"  "}
          {workflow.savingStatus}
        </span>
      </h1>
      <WorkflowToggle
        active={workflow.db_flow.active}
        workflow_id={workflow.db_flow_id}
      />
      <ShareDialog />
      {version && version.published ? (
        <Button
          variant="outline"
          size="sm"
          disabled={true}
          // onClick={() => workflow.publishWorkflowVersion()}
          className="gap-1.5 text-sm bg-green-400 disabled:opacity-100"
        >
          <Send className="size-3.5" />
          Published
        </Button>
      ) : (
        <Button
          variant="outline"
          size="sm"
          onClick={() => workflow.publishWorkflowVersion()}
          className="gap-1.5 text-sm bg-gray-200 hover:bg-green-400"
        >
          <Send className="size-3.5" />
          Publish
        </Button>
      )}
    </header>
  );
}

function ShareIcon(props: any) {
  return (
    <svg
      {...props}
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" />
      <polyline points="16 6 12 2 8 6" />
      <line x1="12" x2="12" y1="2" y2="15" />
    </svg>
  );
}
