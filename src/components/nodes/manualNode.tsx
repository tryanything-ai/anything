import { useState } from "react";
import { Handle, Position } from "reactflow";
import { Node } from "../nodePanel";
import { useSqlContext, EventInput } from "../../context/SqlProvider";
import { useParams } from "react-router-dom";
import { VscPlayCircle } from "react-icons/vsc";
import clsx from "clsx";

let node: Node = {
  nodeType: "manualNode",
  title: "Manual Node",
  alt: "Manual Node",
  nodeData: {
    worker_type: "start",
  },
  specialData: {},
};

ManualNode.Node = node;

export default function ManualNode({ data }: { data: any }) {
  const { addEvent } = useSqlContext();
  const { flow_name } = useParams();
  const [loading, setLoading] = useState(false);
  const createEvent = async () => {
    if (flow_name === undefined) return;
    setLoading(true);
    let event: EventInput = {
      flow_id: flow_name, //flow_id
      flow_name: flow_name,
      flow_version: "0.0.1",
      node_id: "node_id",
      node_type: "manualNode", //node type, lets the machine know it should boostrap the
      stage: "dev",
      worker_type: "start",
      event_status: "PENDING", //EVENT STATUS
      session_status: "PENDING", //SESSION STATUS
      created_at: new Date().toISOString(),
      data: { test: true },
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
    <div
      className={
        "bg-secondary w-60 h-20 p-4 border rounded-md text-primary-content flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <div className="flex flex-row items-center">
        <div className="h-full w-16">
          <button
            className={clsx(loading && "bg-green-500 rounded-full")}
            onClick={() => createEvent()}
          >
            <VscPlayCircle className=" h-12 w-12" />
          </button>
        </div>
        <div className="text-left text-lg">Manual Trigger</div>
      </div>

      <Handle
        type="source"
        position={Position.Bottom}
        id="a"
        isConnectableEnd={false}
      />
    </div>
  );
}
