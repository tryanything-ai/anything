// import { AnythingNodeProps } from "../../utils/flowTypes";
// import BaseNode from "./baseNode";

// export default function SuperNode({ id, data }: AnythingNodeProps) {
//   return (
//     <BaseNode id={id} data={data}>
//       <div className="p-4">{data.node_label}</div>
//     </BaseNode>
//   );
// }
import { cn } from "@/lib/utils"
import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";
// import clsx from "clsx";
import { ReactNode, useEffect, useState } from "react";
// import { VscClose, VscGear } from "react-icons/vsc";
import { Handle, HandleProps } from "reactflow";

import { useFlowNavigationContext } from "../../context/FlowNavigationProvider";
import { useFlowContext } from "../../context/FlowProvider";
import { Node } from "../../utils/flowTypes";

export default function BaseNode({

  id,
  data,
  hideIcon,
}: {
  id: string;
  data: Node;
  hideIcon?: boolean;
}) {
  const { currentProcessingStatus, flowFrontmatter } = useFlowContext();
  const [processing, setProcessing] = useState(false);
  const { setNodeConfigPanel, nodeConfigPanel, nodeId, closeAllPanelsOpenOne } =
    useFlowNavigationContext();

  useEffect(() => {
    if (
      currentProcessingStatus &&
      currentProcessingStatus?.node_id === id &&
      currentProcessingStatus?.flow_id === flowFrontmatter?.flow_id
    ) {
      setProcessing(true);
    } else {
      setProcessing(false);
    }
  }, [currentProcessingStatus]);

  return (
    <div
      className={cn(
        "bg-primary text-primary-content flex h-20 w-80 flex-row rounded-md text-xl",
        {
          "bg-secondary text-secondary-content": data.trigger === true,
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
      {processing ? (
        <div className=" absolute right-0 top-0 z-10 flex h-10 w-10 -translate-y-1/2 translate-x-1/2 transform items-center justify-center overflow-hidden rounded-full bg-white p-0.5 shadow">
          <span className="loading loading-spinner text-accent"></span>
        </div>
      ) : null}
      {/* Container */}
      <div className="flex h-full w-full flex-row items-center p-3">
        {hideIcon ? null : <BaseNodeIcon icon={data.icon} />}
        <div className="flex flex-col">
          <div className="p-4">{data.node_label}</div>
        </div>
        {nodeConfigPanel && nodeId === id ? (
          <button
            className="absolute right-0 top-0 m-1"
            onClick={() => setNodeConfigPanel(false, "")}
          >
            <VscClose />
          </button>
        ) : (
          <button
            className="absolute right-0 top-0 m-1"
            onClick={() => closeAllPanelsOpenOne("nodeConfig", id)}
          >
            <VscGear />
          </button>
        )}
      </div>
    </div>
  );
}

