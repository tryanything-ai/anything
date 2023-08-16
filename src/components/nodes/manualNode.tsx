import React, { useCallback } from "react";
import ReactFlow, { Handle, Position } from "reactflow";
import { Node } from "../nodePanel";
import { useSqlContext } from "../../context/SqlProvider";
import { useParams } from "react-router-dom";

let node: Node = {
  nodeType: "manualNode",
  title: "Manual Node",
  alt: "Manual Node",
  nodeData: {
    start: true,
  },
  specialData: {},
};

ManualNode.Node = node;

export default function ManualNode({ data }: { data: any }) {
  const { addEvent } = useSqlContext();
  const { flow_name } = useParams();

  const createEvent = async () => {
    if (flow_name === undefined) return;

    addEvent(
      "1",
      flow_name,
      "0.0.0",
      "dev",
      "PENDING",
      new Date().toISOString(),
      {}
    );

    //send event to sql
    //let event system process it by running something in rust
    //at end of that last event it will create the next event if more work exists in the flow to be done.
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
