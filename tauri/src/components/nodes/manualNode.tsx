import { useState } from "react";
import { AnythingNodeProps, Node } from "../../utils/nodeUtils";
import { useSqlContext, EventInput } from "../../context/SqlProvider";
import { useParams } from "react-router-dom";
import { VscPlayCircle } from "react-icons/vsc";
import clsx from "clsx";
import BaseNode from "./baseNode";
import { useFlowContext } from "../../context/FlowProvider";

let node: Node = {
  nodeType: "manualNode",
  nodeConfigurationData: {},
  nodePresentationData: {
    title: "Manual Node",
    alt: "Manual Node",
    icon: "",
    handles: [],
  },
  nodeProcessData: {
    worker_type: "start",
  },
};

ManualNode.Node = node;

export default function ManualNode({ id, data }: AnythingNodeProps) {
  const { addEvent } = useSqlContext();
  const { flowFrontmatter } = useFlowContext(); 
  const { flow_name } = useParams();
  const [loading, setLoading] = useState(false);
  const createEvent = async () => {
    if (flow_name === undefined) return;
    if (flowFrontmatter === undefined) return;
    setLoading(true);
    let event: EventInput = {
      flow_id: flowFrontmatter.id,
      flow_name: flow_name,
      flow_version: "0.0.1",
      node_id: id,
      node_type: "manualNode", //node type, lets the machine know it should boostrap the
      stage: "dev",
      worker_type: "start",
      event_status: "PENDING", //EVENT STATUS
      session_status: "PENDING", //SESSION STATUS
      created_at: new Date().toISOString(),
      data: "",
    };

    console.log("Adding event", event);

    addEvent(event);

    //TODO: real user feedback on loading state
    //set loading for 1 second for fun
    setTimeout(() => {
      setLoading(false);
    }, 1000);
  };

  return (
    <BaseNode id={id} data={data} hideIcon>
      <div className="flex flex-row items-center">
        <div className="h-full w-16">
          <button
            className={clsx(loading && "bg-green-500 rounded-full")}
            onClick={() => createEvent()}
          >
            <VscPlayCircle className=" h-12 w-12" />
          </button>
        </div>
        <div className="text-lg">Manual Trigger</div>
      </div>
    </BaseNode>
  );
}
