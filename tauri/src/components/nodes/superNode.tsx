import BaseNode from "./baseNode";
import { AnythingNodeProps } from "../../utils/nodeUtils";

export default function SuperNode({ id, data }: AnythingNodeProps) {
  return (
    <BaseNode id={id} data={data}>
      <div className="p-4">{data.title}</div>
    </BaseNode>
  );
}
