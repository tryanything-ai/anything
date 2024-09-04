import { useEffect, useState } from "react";
import { useParams } from "next/navigation";
import api from "@/lib/anything-api";

export default function VersionsTab(): JSX.Element {
  const params = useParams<{ workflowVersionId: string; workflowId: string }>();
  const [workflowVersions, setWorkflowVersions] = useState<any[]>([]);

  const fetchVersions = async () => {
    try {
      let versions = await api.flows.getFlowVersionsForWorkflowId(
        params.workflowId,
      );
      if (versions.length > 0) {
        setWorkflowVersions(versions);
      } else {
        setWorkflowVersions([]);
        console.log("No versions found");
      }
    } catch (error) {
      console.error(error);
    }
  };

  useEffect(() => {
    if (params.workflowVersionId && params.workflowId) {
      console.log("fetching version", params.workflowVersionId);
      fetchVersions();
    }
  }, [params.workflowVersionId]);

  return (
    <div className="grid w-full items-start gap-6">
      {workflowVersions.map((version) => {
        return (
          <div key={version.id} className="grid grid-cols-12 gap-4">
            <div className="col-span-2">{version.created_at}</div>
            <div className="col-span-2">{version.updated_at}</div>
          </div>
        );
      })}
    </div>
  );
}
