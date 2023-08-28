import { Handle, Position } from "reactflow";
import BaseNode from "./baseNode";
import { AnythingNodeProps, Node } from "../../utils/nodeUtils";

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

export default function VectorNode({ id, data }: AnythingNodeProps) {
  return (
    <BaseNode id={id} data={data}>
      <Handle type="target" position={Position.Top} id="a" />
      <div className="">Vector Node</div>
      <Handle type="source" position={Position.Bottom} id="c" />
    </BaseNode>
  );
}
