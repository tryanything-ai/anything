import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import { stringify, parse } from "iarna-toml-esm";
import { useLocalFileContext } from "./LocalFileProvider";
import { useNodes, useReactFlow } from "reactflow";

function findNextNodeId(nodes: any): string {
  // Initialize the maxId to 0
  let maxId = 0;

  console.log("nodes in FindNextNodeId", nodes);

  // Loop through the nodes and find the maximum numeric ID value
  nodes.forEach((node: any) => {
    const numericId = parseInt(node.id, 10);
    console.log("numericId", numericId);
    if (!isNaN(numericId) && numericId > maxId) {
      maxId = numericId;
    }
  });

  // Increment the maxId to get the next ID for the new node
  const nextId = (maxId + 1).toString();

  return nextId;
}

interface TomlFlowContextInterface {
  toml_nodes: any[];
  toml_edges: any[];
  // editor: string;
  // setEditor: (editor: string) => void;
  set_toml: (data: any) => void;
  addNode: () => void;
}

export const TomlFlowContext = createContext<TomlFlowContextInterface>({
  toml_nodes: [],
  toml_edges: [],
  // editor: "drag",
  // setEditor: () => {},
  set_toml: () => {},
  addNode: () => {},
});

export const useTomlFlowContext = () => useContext(TomlFlowContext);

export const TomlFlowProvider = ({ children }: { children: ReactNode }) => {
  const { toml: tomlFromFile, writeToml } = useLocalFileContext();

  const [toml_nodes, setTomlNodes] = useState<any[]>([]);
  const [toml_edges, setTomlEdges] = useState<any[]>([]);
  // const [editor, setEditor] = useState<string>("drag");
  // const nodes = useNodes();
  // const reactFlowInstance = useReactFlow();

  const addNode = () => {
    // const id = findNextNodeId(nodes);
    // const newNode = {
    //   id,
    //   position: {
    //     x: Math.random() * 500,
    //     y: Math.random() * 500,
    //   },
    //   data: {
    //     label: `Node ${id}`,
    //   },
    // };
    // reactFlowInstance.addNodes(newNode); //TODO: fix this
  };

  const _setToml = (data: any) => {
    //write file to disk
    console.log("_setTomlNodes", data);
    console.log("stringify", stringify({ nodes: data }));
    const { nodes, edges } = data;
    writeToml(stringify({ nodes, edges }));
  };

  useEffect(() => {
    try {
      if (tomlFromFile === "") return;
      var data = parse(tomlFromFile);
      console.log("ParsedToml");
      console.dir(data);
      setTomlNodes(data.nodes as any);
      setTomlEdges(data.edges as any);
    } catch (e) {
      console.log("Error parsing toml. Toml is probably incorrect");
      console.dir(e);
    }
  }, [tomlFromFile]);

  return (
    <TomlFlowContext.Provider
      value={{
        toml_nodes,
        set_toml: _setToml,
        toml_edges,
        addNode,
        // editor,
        // setEditor,
      }}
    >
      {children}
    </TomlFlowContext.Provider>
  );
};
