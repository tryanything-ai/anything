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
  set_toml_nodes: (data: any) => void;
}

export const TomlFlowContext = createContext<TomlFlowContextInterface>({
  toml_nodes: [],
  set_toml_nodes: () => {},
});

export const useTomlFlowContext = () => useContext(TomlFlowContext);

export const TomlFlowProvider = ({ children }: { children: ReactNode }) => {
  const { toml: tomlFromFile, writeToml } = useLocalFileContext();
  const [toml_nodes, setTomlNodes] = useState<any[]>([]);
  const [toml_edges, setTomlEdges] = useState<any[]>([]);

  const _setTomlNodes = (data: any) => {
    //write file to disk
    console.log("_setTomlNodes", data);
    console.log("stringify", stringify({ nodes: data }));

    writeToml(stringify({ nodes: data }));
  };

  useEffect(() => {
    if (tomlFromFile === "") return;
    var data = parse(tomlFromFile);
    console.log("ParsedToml");
    console.dir(data);
    setTomlNodes(data.nodes as any);
  }, [tomlFromFile]);

  return (
    <TomlFlowContext.Provider
      value={{ toml_nodes, set_toml_nodes: _setTomlNodes }}
    >
      {children}
    </TomlFlowContext.Provider>
  );
};
