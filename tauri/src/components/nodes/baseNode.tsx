import { ReactNode, useEffect, useState } from "react";
import { useFlowContext } from "../../context/FlowProvider";
import { NodeData } from "../../utils/nodeUtils";
import { VscEllipsis, VscClose } from "react-icons/vsc";
import clsx from "clsx";
import { useFlowNavigationContext } from "../../context/FlowNavigationProvider";
import { HandleProps, Handle } from "reactflow";
import BaseNodeIcon from "../baseNodeIcon";

export default function BaseNode({
  children,
  id,
  data,
  hideIcon,
}: {
  children: ReactNode;
  id: string;
  data: NodeData;
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
      currentProcessingStatus?.flow_id === flowFrontmatter?.id
    ) {
      setProcessing(true);
    } else {
      setProcessing(false);
    }
  }, [currentProcessingStatus]);

  return (
    <div
      className={clsx(
        "bg-primary text-primary-content w-80 h-20 rounded-md flex flex-row text-xl",
        {
          "bg-secondary text-secondary-content": data.trigger === true,
        }
      )}
    >
      {data.handles.map((handle: HandleProps) => {
        return (
          <Handle
            key={handle.id}
            type={handle.type}
            position={handle.position}
            id={handle.id}
          />
        );
      })}
      {processing ? (
        <div className=" bg-white rounded-full w-10 h-10 absolute top-0 right-0 transform translate-x-1/2 -translate-y-1/2 flex items-center justify-center p-0.5 overflow-hidden shadow z-10">
          <span className="loading loading-spinner text-accent"></span>
        </div>
      ) : null}
      {/* Container */}
      <div className="p-3 flex flex-row h-full w-full items-center">
        {hideIcon ? null : <BaseNodeIcon icon={data.icon} />}
        <div className="flex flex-col">{children}</div>
        {nodeConfigPanel && nodeId === id ? (
          <button
            className="m-1 absolute top-0 right-0"
            onClick={() => setNodeConfigPanel(false, "")}
          >
            <VscClose />
          </button>
        ) : (
          <button
            className="m-1 absolute top-0 right-0"
            onClick={() => closeAllPanelsOpenOne("nodeConfig", id)}
          >
            <VscEllipsis />
          </button>
        )}
      </div>
    </div>
  );
}
