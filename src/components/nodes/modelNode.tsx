import { Handle, Position} from "reactflow";
import BaseNode from "./baseNode";
import { AnythingNodeProps, Node } from "../../utils/nodeUtils";

let node: Node = {
  nodeType: "modelNode",
  title: "Model Node",
  alt: "Model Node",
  nodeData: {
    worker_type: "local_model",
  },
  specialData: {
    filename: "",
    prompt: "",
    variables: [],
  },
};

ModelNode.Node = node;

export default function ModelNode({ id, data }: AnythingNodeProps) {
  return (
    <BaseNode id={id} data={data}>
      <Handle type="target" position={Position.Top} id="a" />
      <div className="">Local Model</div>
      <Handle type="source" position={Position.Bottom} id="c" />
    </BaseNode>
  );
}
