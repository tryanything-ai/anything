import { Node } from "@anything/utils";

import BaseNodeIcon from "./baseNodeIcons";

export const BaseNodeWeb = ({
  node: { trigger, node_label, icon },
}: {
  node: Node;
}) => {
  return (
    <div className="mt-2 flex max-w-md cursor-grab flex-row items-center rounded-md bg-white bg-opacity-5 p-2 pb-2">
      <BaseNodeIcon
        icon={icon}
        className={` ${trigger ? "text-pink-500" : ""}`}
      />
      <h1 className="truncate overflow-ellipsis pl-4 text-lg">{node_label}</h1>
    </div>
  );
};
