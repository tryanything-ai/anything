import { Handle, Position } from "reactflow";
import BaseNode from "./baseNode";
import { AnythingNodeProps, Node } from "../../utils/nodeUtils";

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

export default function SendChatNode({ id, data }: AnythingNodeProps) {
  return (
   <BaseNode id={id} data={data}>
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-xl">Send Chat</div>
      <Handle type="source" position={Position.Bottom} id="b" />
    </BaseNode>
  );
}
