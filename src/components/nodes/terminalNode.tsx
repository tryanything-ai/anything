import React, { useCallback } from "react";
import { Handle, Position, NodeProps } from "reactflow";
import { Node } from "../nodePanel";
import BaseNode from "./baseNode";

let node: Node = {
  nodeType: "terminalNode",
  title: "Terminal Node",
  alt: "Terminal Node",
  nodeData: {
    worker_type: "terminal", 
  },
  specialData: {
    command: "",
  },
};

TerminalNode.Node = node;

type NodeData = {
  command: string;
};

export default function TerminalNode({ id, data }: NodeProps<NodeData>) {
  return (
    <BaseNode id={id} flow_id={"flow_id"}>
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-left text-xl">CLI Command</div>
      <div className="text-left text-md underline  truncate overflow-ellipsis">
        {data.command}
      </div>
      <Handle type="source" position={Position.Bottom} id="b" />
    </BaseNode>
  );
}
