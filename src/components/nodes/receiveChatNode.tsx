import { Handle, Position, NodeProps } from "reactflow";
import { Node } from "../nodePanel";
import BaseNode from "./baseNode";

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

type NodeData = {
  value: number;
};

export default function ReceiveChatNode({ id }: NodeProps<NodeData>) {
  return (
    <BaseNode id={id} flow_id="flow_id"
    >
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-left text-xl">Receive Chat</div>
      {/* <div className="text-left text-md underline  truncate overflow-ellipsis">
        {data.command}
      </div> */}
      <Handle type="source" position={Position.Bottom} id="b" />
    </BaseNode>
  );
}
