import { useEffect, useState } from "react";
import { useFlowContext } from "../context/FlowProvider";
import { useLocalFileContext } from "../context/LocalFileProvider";

type Node = {
  nodeType: string;
  image_src?: string;
  title?: string;
  alt: string;
};

export const default_nodes: Node[] = [
  {
    nodeType: "default",
    title: "Default Node",
    alt: "Default Node",
  },
  {
    nodeType: "pythonNode",
    image_src: "/python-logo.svg",
    alt: "Python Logo",
  },
  {
    nodeType: "javascriptNode",
    image_src: "/js-logo.svg",
    alt: "JS Logo",
  },
];

const NodePanel = () => {
  const [nodes, setNodes] = useState<Node[]>(default_nodes);
  const [flows, setFlows] = useState<Node[]>([]);
  const { flowPaths } = useLocalFileContext();

  useEffect(() => {
    //make flows nodes
    const flows = flowPaths.map((path) => {
      return {
        nodeType: "flow",
        title: path.name ? path.name : "",
        alt: path.name ? path.name : "",
      };
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

const NodeButton = ({ nodeType, image_src, title, alt }: Node) => {
  const { addNode } = useFlowContext();
  return (
    <button
      onClick={() => addNode(nodeType)}
      className="btn btn-neutral mt-2 pb-2"
    >
      {image_src ? (
        <img
          src={image_src}
          alt={alt}
          className="max-w-full max-h-full mt-2 ml-4"
        />
      ) : (
        <h1 className="text-lg">{title}</h1>
      )}
    </button>
  );
};

export default NodePanel;
