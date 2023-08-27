import React, { useCallback } from "react";
import ReactFlow, { Handle, Position, NodeProps } from "reactflow";
import { Node } from "../nodePanel";
import BaseNode from "./baseNode";

let node: Node = {
  nodeType: "sendChatNode",
  title: "Send Chat Node",
  alt: "Send Chat Node",
  nodeData: {
    worker_type: "app_chat", 
  },
  specialData: {
    command: "",
  },
};

SendChatNode.Node = node;

type NodeData = {
  value: number;
};

export default function SendChatNode({ id }: NodeProps<NodeData>) {
  return (
   <BaseNode id={id} flow_id="flow_id">
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-left text-xl">Send Chat</div>
      {/* <div className="text-left text-md underline  truncate overflow-ellipsis">
        {data.command}
      </div> */}
      <Handle type="source" position={Position.Bottom} id="b" />
    </BaseNode>
  );
}
