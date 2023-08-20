import { Handle, Position } from "reactflow";
import { Node } from "../nodePanel";
import { useSqlContext, EventInput } from "../../context/SqlProvider";
import { useParams } from "react-router-dom";

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

  const createEvent = async () => {
    if (flow_name === undefined) return;
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
    addEvent(event);
  };

  return (
    <div
      className={
        "bg-secondary w-40 h-20 p-4 border rounded-md text-primary-content flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <div className="text-left text-xl">Manual Node</div>
      <button className="btn btn-secondary" onClick={() => createEvent()}>
        Call
      </button>
      <Handle
        type="source"
        position={Position.Bottom}
        id="a"
        isConnectableEnd={false}
      />
    </div>
  );
}
