import { Handle, Position, NodeProps } from "reactflow";
import { Node } from "../nodePanel";
import BaseNode from "./baseNode";

let node: Node = {
  nodeType: "vectorNode",
  title: "Vector Node",
  alt: "Vector Node",
  nodeData: {
    worker_type: "vector",
  },
  specialData: {
    db: "",
  },
};

VectorNode.Node = node;

type NodeData = {
  value: number;
};

export default function VectorNode({ id, data }: NodeProps<NodeData>) {
  return (
    <BaseNode id={id} flow_id="flow_id">
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-center text-xl">{data.value}</div>
      <Handle type="target" position={Position.Right} id="b" />
      <Handle type="source" position={Position.Bottom} id="c" />
    </BaseNode>
  );
}
