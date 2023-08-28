import React, { useCallback } from "react";
import ReactFlow, { Handle, Position, NodeProps } from "reactflow";
import { Node } from "../nodePanel";
import BaseNode from "./baseNode";

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

type NodeData = {
  value: number;
};

export default function CronNode({ id }: NodeProps<NodeData>) {
  // const onChange = useCallback((evt: any) => {
  //   console.log(evt.target.value);
  // }, []);

  return (
    <BaseNode id={id} flow_id="flow_id">
      <div className="text-left text-xl">Cron</div>
      <div className="text-left text-md underline">Every 5 Minutes</div>
      <Handle
        type="source"
        position={Position.Bottom}
        id="a"
        isConnectableEnd={false}
      />
    </BaseNode>
  );
}
