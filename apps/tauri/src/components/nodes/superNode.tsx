import { AnythingNodeProps } from "../../utils/flowTypes";
import BaseNode from "./baseNode";

export default function SuperNode({ id, data }: AnythingNodeProps) {
  return (
    <BaseNode id={id} data={data}>
      <div className="p-4">{data.node_label}</div>
    </BaseNode>
  );
}
