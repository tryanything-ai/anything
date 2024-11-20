import { useParams } from "next/navigation";
import { format } from "date-fns";
import { Badge } from "@repo/ui/components/ui/badge";
import Link from "next/link";
import { useAnything } from "@/context/AnythingContext";

export default function VersionsTab(): JSX.Element {
  const params = useParams<{ workflowVersionId: string; workflowId: string }>();

  const {
    version_control: { versions },
  } = useAnything();
  return (
    <div className="grid w-full items-start gap-2">
      {versions.map((version) => (
        <Link
          key={version.flow_version_id}
          href={`/workflows/${params.workflowId}/${version.flow_version_id}/editor`}
          className="flex flex-col  rounded-md border border-gray-200 p-4 cursor-pointer"
        >
          <div className="grid grid-cols-6 gap-4">
            <div className="col-span-3">
              {format(new Date(version.created_at), "MMM d, yyyy")}
            </div>
            <div className="col-span-3 text-right">
              {version.flow_version_id === params.workflowVersionId && (
                <Badge className="inline-flex items-center px-3 py-1 rounded-full bg-blue-200 text-blue-800 hover:bg-blue-200">
                  Viewing
                </Badge>
              )}
              {version.published && (
                <Badge className="inline-flex items-center px-3 py-1 rounded-full bg-green-200 text-green-800 hover:bg-green-200 ml-1">
                  Live
                </Badge>
              )}
            </div>
          </div>
          {/* Bottom Row */}
          <div className="flex flex-col gap-1 mt-2">
            <div className="flex items-center gap-2">
              <span className="text-sm text-gray-400">Workflow ID:</span>
              <code className="text-sm text-gray-500 select-all">{version.flow_id}</code>
            </div>
            <div className="flex items-center gap-2">
              <span className="text-sm text-gray-400">Version ID:</span>
              <code className="text-sm text-gray-500 select-all">{version.flow_version_id}</code>
            </div>
          </div>
        </Link>
      ))}
    </div>
  );
}
