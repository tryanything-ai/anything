import { Handle, Position } from "reactflow";
import BaseNode from "./baseNode";
import { AnythingNodeProps, Node } from "../../utils/nodeUtils";

let node: Node = {
  nodeType: "terminalNode",
  
  nodeData: {
    title: "Terminal Node",
     alt: "Terminal Node",
    worker_type: "terminal",
  },
  specialData: {
    command: "",
  },
};

TerminalNode.Node = node;

export default function TerminalNode({ id, data }: AnythingNodeProps) {
  return (
    <BaseNode id={id} data={data}>
      <Handle type="target" position={Position.Top} id="a" />
      <div className="">CLI Command</div>
      <div className="text-md underline  truncate overflow-ellipsis">
        {data.command}
      </div>
      <Handle type="source" position={Position.Bottom} id="b" />
    </BaseNode>
  );
}
