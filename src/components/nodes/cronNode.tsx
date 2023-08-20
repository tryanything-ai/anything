import React, { useCallback } from "react";
import ReactFlow, { Handle, Position } from "reactflow";
import { Node } from "../nodePanel";


let node: Node = {
  nodeType: "cronNode",
  title: "Cron Node",
  alt: "Cron Node",
  nodeData: {
    worker_type: "start", 
  },
  specialData: {
    pattern: "",
  },
};

CronNode.Node = node;


export default function CronNode({ data }: { data: any }) {
  // const onChange = useCallback((evt: any) => {
  //   console.log(evt.target.value);
  // }, []);

  return (
    <div
      className={
        "bg-secondary w-40 h-20 p-4 border rounded-md text-primary-content flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <div className="text-left text-xl">Cron</div>
      <div className="text-left text-md underline">Every 5 Minutes</div>
      <Handle type="source" position={Position.Bottom} id="a" isConnectableEnd={false} />
    </div>
  );
}
