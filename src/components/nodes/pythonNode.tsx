import { Handle, Position } from "reactflow";
import BaseNode from "./baseNode";
import { AnythingNodeProps, Node } from "../../utils/nodeUtils";

let node: Node = {
  nodeType: "pythonNode",
  title: "Python Node",
  alt: "Python Node",
  nodeData: {
    worker_type: "python",
  },
  specialData: {
    code: "",
  },
};

PythonNode.Node = node;

type NodeData = {
  value: number;
};

//Node that acts as the beginning of a flow or one of many beginnings of a flow
export default function PythonNode({ id, data}: AnythingNodeProps) {
  return (
    <BaseNode id={id} data={data}>
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
