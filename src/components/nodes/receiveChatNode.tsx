import { Handle, Position } from "reactflow";
import BaseNode from "./baseNode";
import { AnythingNodeProps, Node } from "../../utils/nodeUtils";

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

export default function ReceiveChatNode({ id, data }: AnythingNodeProps) {
  return (
    <BaseNode id={id} data={data}>
      <Handle type="target" position={Position.Top} id="a" />
      <div className="">Receive Chat</div>
      <Handle type="source" position={Position.Bottom} id="b" />
    </BaseNode>
  );
}
