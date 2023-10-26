import { Node } from "@/types/flow";
import BaseNodeIcon from "@/components/baseNodeIcons";

export const BaseNodeWeb = ({
  node: { trigger, node_label, icon },
}: {
  node: Node;
}) => {
  return (
    <div className="flex flex-row mt-2 pb-2 max-w-md cursor-grab bg-white bg-opacity-5 rounded-md p-2 items-center">
      <BaseNodeIcon
        icon={icon}
        className={` ${trigger ? "text-pink-500" : ""}`}
      />
      <h1 className="text-lg truncate overflow-ellipsis pl-4">{node_label}</h1>
    </div>
  );
};
