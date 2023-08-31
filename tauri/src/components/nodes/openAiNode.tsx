import { Handle, Position } from "reactflow";
import BaseNode from "./baseNode";
import { AnythingNodeProps, Node } from "../../utils/nodeUtils";

let node: Node = {
  nodeType: "openAiNode",
  title: "OpenAI Node",
  alt: "OpenAI Node",
  nodeData: {
    worker_type: "rest",
  },
  specialData: {
    url: "",
    params: [], 
  },
};

OpenAiNode.Node = node;

export default function OpenAiNode({ id, data }: AnythingNodeProps) {
  return (
    <BaseNode id={id} data={data}>
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-xl">Open AI Node</div>
      <Handle type="source" position={Position.Bottom} id="b" />
    </BaseNode>
  );
}
