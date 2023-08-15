import React, { useCallback } from "react";
import ReactFlow, { Handle, Position } from "reactflow";
import { Node } from "../nodePanel";

let node: Node = {
  nodeType: "terminalNode",
  title: "Terminal Node",
  alt: "Terminal Node",
  specialData: {
    command: "",
  },
};

TerminalNode.Node = node;

export default function TerminalNode({ data }: { data: any }) {
  return (
    <div
      className={
        "bg-primary w-40 h-20 p-4 border rounded-md text-primary-content flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-left text-xl">CLI Command</div>
      <div className="text-left text-md underline  truncate overflow-ellipsis">
        {data.command}
      </div>
      <Handle type="source" position={Position.Bottom} id="b" />
    </div>
  );
}
