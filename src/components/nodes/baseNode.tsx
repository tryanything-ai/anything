import { ReactNode, useEffect, useState } from "react";
import { useFlowContext } from "../../context/FlowProvider";
import { NodeData } from '../../utils/nodeUtils'; 
import clsx from "clsx";

export default function BaseNode({
  children,
  id,
  data,
}: {
  children: ReactNode;
  id: string;
  data: NodeData, 
}) {
  const { currentProcessingStatus, flowFrontmatter } = useFlowContext();
  const [processing, setProcessing] = useState(false);

  useEffect(() => {
    //FIXME: manage flow_name by ID globally here we need it to only show activity if the activity is from the right flow
    console.log("now: Processing set to true in node", currentProcessingStatus);
    console.log("now: Data id", id);
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
    className={clsx("bg-primary w-60 h-20 p-4 border rounded-md text-primary-content flex flex-col justify-center align-middle text-center text-xl", {
      "bg-secondary": data.worker_type === "start",
    })}>
      {processing ? (
        <div className=" bg-white rounded-full w-10 h-10 absolute top-0 right-0 transform translate-x-1/2 -translate-y-1/2 flex items-center justify-center p-0.5 overflow-hidden shadow z-10">
          <span className="loading loading-spinner text-accent"></span>
        </div>
      ) : null}

      {children}
    </div>
  );
}