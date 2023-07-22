import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import { stringify, parse } from "iarna-toml-esm";
import { useLocalFileContext } from "./LocalFileProvider";

interface TomlFlowContextInterface {
  toml_nodes: any[];
  toml_edges: any[];
  set_toml: (data: any) => void;
}

export const TomlFlowContext = createContext<TomlFlowContextInterface>({
  toml_nodes: [],
  toml_edges: [],
  set_toml: () => {},
});

export const useTomlFlowContext = () => useContext(TomlFlowContext);

export const TomlFlowProvider = ({ children }: { children: ReactNode }) => {
  const { toml: tomlFromFile, writeToml } = useLocalFileContext();
  const [toml_nodes, setTomlNodes] = useState<any[]>([]);
  const [toml_edges, setTomlEdges] = useState<any[]>([]);

  const _setToml = (data: any) => {
    //write file to disk
    console.log("_setTomlNodes", data);
    console.log("stringify", stringify({ nodes: data }));
    const { nodes, edges } = data;
    writeToml(stringify({ nodes, edges }));
  };

  // const _setTomlEdges = (data: any) => {
  //   //write file to disk
  //   console.log("_setTomlEdges", data);
  //   console.log("stringify", stringify({ edges: data }));
  //   writeToml(stringify({ edges: data }));
  // };

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
        // set_toml_edges: _setTomlEdges,
      }}
    >
      {children}
    </TomlFlowContext.Provider>
  );
};
