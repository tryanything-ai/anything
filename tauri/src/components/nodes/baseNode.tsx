import { ReactNode, useEffect, useState } from "react";
import { useFlowContext } from "../../context/FlowProvider";
import { NodeData } from "../../utils/nodeUtils";
import { VscEllipsis, VscClose } from "react-icons/vsc";
import clsx from "clsx";
import { useNavigationContext } from "../../context/NavigationProvider";
import { HandleProps, Handle } from "reactflow";

export default function BaseNode({
  children,
  id,
  data,
}: {
  children: ReactNode;
  id: string;
  data: NodeData;
}) {
  const { currentProcessingStatus, flowFrontmatter } = useFlowContext();
  const [processing, setProcessing] = useState(false);
  const { setNodeConfigPanel, nodeConfigPanel, nodeId } =
    useNavigationContext();

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
        "bg-primary w-60 h-20 rounded-md flex flex-row justify-center align-middle text-center text-xl",
        {
          "bg-secondary": data.worker_type === "start",
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
      <div className="flex flex-col p-4">{children}</div>

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
          onClick={() => setNodeConfigPanel(true, id)}
        >
          <VscEllipsis />
        </button>
      )}
    </div>
  );
}
