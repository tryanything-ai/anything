import { useEffect, useState } from "react";
import { useFlowContext } from "../context/FlowProvider";
import { useLocalFileContext } from "../context/LocalFileProvider";
import { useParams } from "react-router-dom";
import TerminalNode from "./nodes/terminalNode";
import ModelNode from "./nodes/modelNode";
import CronNode from "./nodes/cronNode";
import JavascriptNode from "./nodes/javascriptNode";
import ManualNode from "./nodes/manualNode";

export type NodeData = {
  worker_type: string;
};

export type Node = {
  nodeType: string;
  image_src?: string;
  title?: string;
  alt: string;
  nodeData: NodeData;
  specialData?: any;
};

export const default_nodes: Node[] = [
  JavascriptNode.Node,
  CronNode.Node,
  TerminalNode.Node,
  ModelNode.Node,
  ManualNode.Node,
];

const NodePanel = () => {
  const [nodes, setNodes] = useState<Node[]>(default_nodes);
  const [flows, setFlows] = useState<Node[]>([]);
  const { flowPaths } = useLocalFileContext();
  const { flow_name } = useParams();

  useEffect(() => {
    let flows: Node[] = [];

    //make flows nodes
    flowPaths.forEach((path) => {
      //do not return the current flow
      if (path.name !== flow_name) {
        flows.push({
          nodeType: "flow",
          title: path.name ? path.name : "",
          alt: path.name ? path.name : "",
          nodeData: { worker_type: "flow" },
        });
      }
    });

    setFlows(flows);
  }, [flowPaths]);

  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      <h1 className="text-2xl font-bold">Nodes</h1>
      {nodes.map((node) => (
        <NodeButton
          key={node.nodeType}
          nodeType={node.nodeType}
          image_src={node.image_src}
          title={node.title}
          alt={node.alt}
          nodeData={node.nodeData}
          specialData={node.specialData}
        />
      ))}
      <h1 className="text-2xl font-bold mt-2">Flows</h1>
      {flows.map((node) => (
        <NodeButton
          key={node.nodeType + node.title}
          nodeType={node.nodeType}
          title={node.title}
          image_src={node.image_src}
          alt={node.alt}
          nodeData={node.nodeData}
        />
      ))}
    </div>
  );
};

const NodeButton = ({
  nodeType,
  image_src,
  title,
  alt,
  specialData,
  nodeData,
}: Node) => {
  const onDragStart = (event: any, nodeType: any) => {
    console.log("drag started", nodeType);
    event.dataTransfer.setData("application/reactflow", nodeType);
    //TODO: add special data
    event.dataTransfer.effectAllowed = "move";
  };

  return (
    <div
      className="btn btn-neutral mt-2 pb-2 max-w-md cursor-grab"
      onDragStart={(event) => onDragStart(event, nodeType)}
      draggable
    >
      {image_src ? (
        <img
          src={image_src}
          alt={alt}
          className="max-w-full max-h-full mt-2 ml-4"
        />
      ) : (
        <h1 className="text-lg truncate overflow-ellipsis">{title}</h1>
      )}
    </div>
  );
};

export default NodePanel;
