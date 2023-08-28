import { Handle, Position, NodeProps } from "reactflow";
import { Node } from "../nodePanel";
import BaseNode from "./baseNode";

let node: Node = {
  nodeType: "openAiNode",
  title: "OpenAI Node",
  alt: "OpenAI Node",
  nodeData: {
    worker_type: "rest",
  },
  specialData: {
    url: "",
  },
};

OpenAiNode.Node = node;

type NodeData = {
  value: number;
};

export default function OpenAiNode({ id }: NodeProps<NodeData>) {
  return (
    <BaseNode id={id} flow_id="flow_id">
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-left text-xl">Open AI Node</div>
      <Handle type="source" position={Position.Bottom} id="b" />
    </BaseNode>
  );
}
