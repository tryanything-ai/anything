import { useEffect, useState } from "react";
import { useFlowContext } from "../context/FlowProvider";
import { useLocalFileContext } from "../context/LocalFileProvider";
import { useParams } from "react-router-dom";
import TerminalNode from "./nodes/terminalNode";

export type Node = {
  nodeType: string;
  image_src?: string;
  title?: string;
  alt: string;
  specialData?: any;
};

export const default_nodes: Node[] = [
  {
    nodeType: "default",
    title: "Default Node",
    alt: "Default Node",
  },
  {
    nodeType: "javascriptNode",
    image_src: "/js-logo.svg",
    alt: "JS Logo",
  },
  {
    nodeType: 'cronNode', 
    title: 'Cron Node',
    alt: 'Cron Node'
  },TerminalNode.Node
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
          nodeType={node.nodeType}
          image_src={node.image_src}
          title={node.title}
          alt={node.alt}
          specialData={node.specialData}
        />
      ))}
      <h1 className="text-2xl font-bold mt-2">Flows</h1>
      {flows.map((node) => (
        <NodeButton
          nodeType={node.nodeType}
          title={node.title}
          image_src={node.image_src}
          alt={node.alt}
        />
      ))}
    </div>
  );
};

const NodeButton = ({ nodeType, image_src, title, alt, specialData }: Node) => {
  const { addNode } = useFlowContext();
  return (
    <button
      onClick={() => addNode(nodeType, specialData)}
      className="btn btn-neutral mt-2 pb-2 max-w-md"
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
    </button>
  );
};

export default NodePanel;
