import { Handle, Position } from "reactflow";
import { AnythingNodeProps, Node } from '../../utils/nodeUtils';
import BaseNode from "./baseNode";

let node: Node = {
  nodeType: "cronNode",
  title: "Cron Node",
  alt: "Cron Node",
  nodeData: {
    worker_type: "start",
  },
  specialData: {
    pattern: "",
  },
};

CronNode.Node = node;

export default function CronNode({ id, data }: AnythingNodeProps) {
  return (
    <BaseNode id={id} data={data}>
      <div className="">Cron</div>
      <div className="text-md underline">Every 5 Minutes</div>
      <Handle
        type="source"
        position={Position.Bottom}
        id="a"
        isConnectableEnd={false}
      />
    </BaseNode>
  );
}
