import { useEffect, useState } from "react";
import { Node } from "../utils/nodeUtils";
import { getActionNodes, getTriggerNodes } from "../utils/nodeGenerators";
import BaseNodeIcon from "./baseNodeIcon";
import React, { ChangeEvent, MouseEventHandler } from "react";

import { VscChevronDown, VscChevronUp } from "react-icons/vsc";
import BaseSearch from "./baseSearch";

const NodePanel = () => {
  const [triggerNodes, setTriggerNodes] = useState<Node[]>([]);
  const [actionNodes, setActionNodes] = useState<Node[]>([]);
  const [showActions, setShowActions] = useState(true);
  const [showTriggers, setShowTriggers] = useState(true);
  const [searchValue, setSearchValue] = useState("");

  useEffect(() => {
    setTriggerNodes(getTriggerNodes());
    setActionNodes(getActionNodes());
  }, []);

  const setResults = (results: Node[]) => {
    console.log("results", results);
    setTriggerNodes(results.filter((node) => node.nodeProcessData.trigger));
    setActionNodes(results.filter((node) => !node.nodeProcessData.trigger));
  };

  return (
    <div className="max-h-screen overflow-y-auto p-4 hide-scrollbar">
      <div className="py-4">
        <BaseSearch
          data={[...triggerNodes, ...actionNodes]}
          //TODO: create flat interface for nodes to power search
          searchKey={["nodePresentationData"]}
          onResultsChange={(results) => setResults(results)}
        />
      </div>

      <h1
        onClick={() => setShowTriggers(!showTriggers)}
        className="h-12 py-2 text-xl font-bold pb-2 flex flex-row justify-between"
      >
        Triggers
        {showTriggers ? <VscChevronDown /> : <VscChevronUp />}
      </h1>
      <div
        className={`overflow-hidden transition-max-height duration-500 ease-in-out pb-2 ${
          showTriggers ? "max-h-auto" : "max-h-0"
        }`}
      >
        {triggerNodes.map((node: Node) => (
          <NodeDnD node={node} key={node.nodePresentationData.node_label} />
        ))}
      </div>
      <h1
        onClick={() => setShowActions(!showActions)}
        className="text-xl py-2 font-bold pb-2 flex flex-row justify-between"
      >
        Actions
        {showActions ? <VscChevronDown /> : <VscChevronUp />}
      </h1>
      <div
        className={`overflow-hidden transition-max-height duration-500 ease-in-out pb-2 ${
          showActions ? "max-h-auto" : "max-h-0"
        }`}
      >
        {actionNodes.map((node: Node) => (
          <NodeDnD node={node} key={node.nodePresentationData.node_label} />
        ))}
      </div>
    </div>
  );
};

const NodeDnD = ({ node }: { node: Node }) => {
  const onDragStart = (event: any) => {
    let nodeType;

    if (!node.nodePresentationData.nodeType) {
      nodeType = "superNode";
    } else {
      nodeType = node.nodePresentationData.nodeType;
    }

    console.log("drag started", nodeType);

    event.dataTransfer.setData("nodeType", nodeType);
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
