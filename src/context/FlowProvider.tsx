import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import {
  Connection,
  Edge,
  EdgeChange,
  Node,
  NodeChange,
  addEdge,
  OnNodesChange,
  OnEdgesChange,
  OnConnect,
  applyNodeChanges,
  applyEdgeChanges,
} from "reactflow";
import { useLocalFileContext } from "./LocalFileProvider";
import { useTauriContext } from "./TauriProvider";
import { readTextFile } from "@tauri-apps/api/fs";
import { stringify, parse } from "iarna-toml-esm";
import { watchImmediate } from "tauri-plugin-fs-watch-api";

import { useParams } from "react-router-dom";
interface FlowContextInterface {
  nodes: Node[];
  edges: Edge[];
  onNodesChange: OnNodesChange;
  onEdgesChange: OnEdgesChange;
  onConnect: OnConnect;
  toml: string;
}

export const FlowContext = createContext<FlowContextInterface>({
  nodes: [],
  edges: [],
  onNodesChange: () => {},
  onEdgesChange: () => {},
  onConnect: () => {},
  toml: "",
});

export const useFlowContext = () => useContext(FlowContext);

export const FlowProvider = ({ children }: { children: ReactNode }) => {
  const { appDocuments } = useTauriContext();
  const { flow_name } = useParams();
  const [initalTomlLoaded, setInitialTomlLoaded] = useState<boolean>(false);

  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const [toml, setToml] = useState<string>("");

  const onNodesChange: OnNodesChange = (nodeChanges: NodeChange[]) => {
    setNodes((nodes) => applyNodeChanges(nodeChanges, nodes));
  };

  //When the edge is changed
  const onEdgesChange: OnEdgesChange = (edgeChanges: any) => {
    setEdges((edges) => applyEdgeChanges(edgeChanges, edges));
  };

  //When a node is connected to an edge etc
  const onConnect: OnConnect = (params: any) => {
    setEdges((edges) => addEdge(params, edges));
  };

  const readToml = async () => {
    return await readTextFile(
      appDocuments + "/flows/" + flow_name + "/flow.toml"
    );
  };

  const loadToml = async () => {
    try {
      let new_toml = await readToml();
      if (toml === new_toml) return; //don't update if the toml is the same
      setToml(new_toml);
      let parsedToml = parse(new_toml);
      setNodes(parsedToml.nodes as any);
      setEdges(parsedToml.edges as any);
      setInitialTomlLoaded(true);
    } catch (error) {
      console.log("error loading toml in FlowProvider", error);
    }
  };

  //Load TOML into State the first time
  useEffect(() => {
    console.log("Flow Name");
    if (flow_name && !initalTomlLoaded) {
      loadToml();
    }
  }, [flow_name]);

  //Update Toml as side effect of nodes and edges changing
  useEffect(() => {
    //this will probably have some duplications
    let newToml = stringify({ nodes: nodes as any, edges: edges as any });
    console.log(
      "Updating toml in useEffect in flowProvider as side effect of drag editor"
    );
    //TODO: take numbers and make them not so long
    setToml(newToml);
  }, [edges, nodes]);

  useEffect(() => {
    let stopWatching = () => {};
    let path = `${appDocuments}/flows/${flow_name}/flow.toml`;

    console.log(`Watching ${path} for changes`);

    const watchThisFile = async () => {
      stopWatching = await watchImmediate(path, (event) => {
        console.log(
          "File watchImmediate in FlowProvider triggered: ",
          JSON.stringify(event, null, 3)
        );
        console.log("Updating Node State as Side Effect of Updated TOML file");
        loadToml();
      });
    };

    watchThisFile();
    return () => {
      stopWatching();
    };
  }, []);

  return (
    <FlowContext.Provider
      value={{ nodes, edges, onConnect, onNodesChange, onEdgesChange, toml }}
    >
      {children}
    </FlowContext.Provider>
  );
};
