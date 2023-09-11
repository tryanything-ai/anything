import { useEffect, useState } from "react";
import { Node } from "../utils/nodeUtils";
import { getActionNodes, getTriggerNodes } from "../utils/nodeGenerators";
import BaseNodeIcon from "./baseNodeIcon";

const NodePanel = () => {
  const [triggerNodes, setTriggerNodes] = useState<Node[]>([]);
  const [actionNodes, setActionNodes] = useState<Node[]>([]);

  useEffect(() => {
    setTriggerNodes(getTriggerNodes());
    setActionNodes(getActionNodes());
  }, []);

  //TODO: add flows in some future where we can facilitate
  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500 overflow-y-auto">
      <h1 className="text-2xl font-bold pb-2">Triggers</h1>
      {triggerNodes.map((node: Node) => (
        <NodeDnD node={node} key={node.nodePresentationData.node_label} />
      ))}
      <h1 className="text-2xl font-bold py-2">Actions</h1>
      {actionNodes.map((node: Node) => (
        <NodeDnD node={node} key={node.nodePresentationData.node_label} />
      ))}
    </div>
  );
};

const NodeDnD = ({ node }: { node: Node }) => {
  const onDragStart = (event: any) => {
    console.log("drag started", node.nodeType);
    event.dataTransfer.setData("nodeType", node.nodeType);
    event.dataTransfer.setData(
      "nodeProcessData",
      JSON.stringify(node.nodeProcessData)
    );
    event.dataTransfer.setData(
      "nodeConfigurationData",
      JSON.stringify(node.nodeConfigurationData)
    );
    event.dataTransfer.setData(
      "nodePresentationData",
      JSON.stringify(node.nodePresentationData)
    );
    event.dataTransfer.effectAllowed = "move";
  };

  return (
    <div
      className="flex flex-row mt-2 pb-2 max-w-md cursor-grab bg-white bg-opacity-5 rounded-md p-2 items-center"
      onDragStart={(event) => onDragStart(event)}
      draggable
    >
      <BaseNodeIcon
        icon={node.nodePresentationData.icon}
        className={`h-9 w-9 bg-opacity-80 ${
          node.nodeProcessData.trigger ? "text-secondary" : "text-primary"
        }`}
      />
      <h1 className="text-lg truncate overflow-ellipsis pl-2">
        {node.nodePresentationData.node_label}
      </h1>
    </div>
  );
};

export default NodePanel;
