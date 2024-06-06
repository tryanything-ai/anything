import { cn } from "@/lib/utils"
import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";
import { Handle, HandleProps } from "reactflow";
import { useFlowNavigationContext } from "@/context/FlowNavigationProvider";
import { Node } from "@/types/flows"
import { Badge } from "@/components/ui/badge";
import { ZapIcon } from "lucide-react";

export default function BaseNode({
  id,
  data,
}: {
  id: string;
  data: Node;
}) {

  const { setNodeConfigPanel, nodeConfigPanel, nodeId, closeAllPanelsOpenOne } =
    useFlowNavigationContext();

  const toggleNodeConfig = () => {
    if (nodeConfigPanel && nodeId === id) {
      setNodeConfigPanel(false, "")
    } else {
      closeAllPanelsOpenOne("nodeConfig", id)
    }
  }
  return (
    <div
      onClick={toggleNodeConfig}
      className={cn(
        "bg-white border border-gray-300 text-primary-content flex h-20 w-80 flex-row rounded-md text-xl",
        {
          "": data.trigger === true,
        }
      )}
    >
      {data.handles ? data.handles.map((handle: HandleProps) => {
        return (
          <Handle
            key={handle.id}
            type={handle.type}
            position={handle.position}
            id={handle.id}
          />
        );
      }) : null}

      {/* Container */}
      <div className="flex h-full w-full flex-row items-center p-3">
        <BaseNodeIcon icon={data.icon} />
        <div className="flex flex-col">
          <div className="p-4">{data.node_label}</div>
        </div>
      </div>
      {data.trigger ?
        <Badge className="bg-green-200 border border-gray-200 rounded-xl h-6 w-22 mr-2 mt-2 p-1 hover:bg-green-200">
          <ZapIcon className="h-4 w-4 text-green-700" />
          {" "}<span className="text-gray-700 text-xs font-">Trigger</span>
        </Badge>
        : null}
    </div>
  );
}

