import { cn } from "@/lib/utils"
import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";
import { Handle, HandleProps } from "reactflow";
import { useFlowNavigationContext } from "@/context/FlowNavigationProvider";
import { Node } from "@/types/flows"

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
          "bg-grey text-secondary-content": data.trigger === true,
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
    </div>
  );
}

