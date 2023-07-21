import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";
import toml from "toml";
import { useLocalFileContext } from "./LocalFileProvider";

interface TomlFlowContextInterface {
  toml_nodes: any[];
}

export const TomlFlowContext = createContext<TomlFlowContextInterface>({
  toml_nodes: [],
});

export const useTomlFlowContext = () => useContext(TomlFlowContext);

export const TomlFlowProvider = ({ children }: { children: ReactNode }) => {
  const [theme, setTheme] = useState(localStorage.getItem("theme") || "dark");
  const { toml: tomlFromFile } = useLocalFileContext();
  const [toml_nodes, setTomlNodes] = useState<any[]>([]);
  const [toml_edges, setTomlEdges] = useState<any[]>([]);

  useEffect(() => {
    document.body.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }, [theme]);

  useEffect(() => {
    if (tomlFromFile === "") return;
    var data = toml.parse(tomlFromFile);
    console.log("ParsedToml");
    console.dir(data);
    setTomlNodes(data.nodes);
  }, [tomlFromFile]);

  return (
    <TomlFlowContext.Provider value={{ toml_nodes }}>
      {children}
    </TomlFlowContext.Provider>
  );
};
