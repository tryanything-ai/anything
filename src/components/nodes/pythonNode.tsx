import { Handle, Position, NodeProps } from "reactflow";

import { Node } from "../nodePanel";
import BaseNode from "./baseNode";

let node: Node = {
  nodeType: "pythonNode",
  title: "Python Node",
  alt: "Python Node",
  nodeData: {
    worker_type: "rest",
  },
  specialData: {
    url: "",
  },
};

PythonNode.Node = node;

type NodeData = {
  value: number;
};

//Node that acts as the beginning of a flow or one of many beginnings of a flow
export default function PythonNode({ id }: NodeProps<NodeData>) {
  return (
    <BaseNode id={id} flow_id="flow_id">
      <Handle type="target" position={Position.Top} id="a" />
      <img
        src={"/python-logo.svg"}
        alt="Python Logo"
        className="max-w-full max-h-full mt-2 ml-4"
      />
      <Handle type="source" position={Position.Bottom} id="b" />
    </BaseNode>
  );
}
