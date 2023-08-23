import React, { useCallback } from "react";
import ReactFlow, { Handle, Position } from "reactflow";
import { Node } from "../nodePanel";

let node: Node = {
  nodeType: "receiveChatNode",
  title: "Receive Chat Node",
  alt: "Receive Chat Node",
  nodeData: {
    worker_type: "start",
  },
  specialData: {
    message: "",
  },
};

ReceiveChatNode.Node = node;

export default function ReceiveChatNode({ data }: { data: any }) {
  return (
    <div
      className={
        "bg-secondary w-40 h-20 p-4 border rounded-md text-primary-content flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-left text-xl">Receive Chat</div>
      {/* <div className="text-left text-md underline  truncate overflow-ellipsis">
        {data.command}
      </div> */}
      <Handle type="source" position={Position.Bottom} id="b" />
    </div>
  );
}
