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
import { useTauriContext } from "./TauriProvider";
import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { stringify, parse } from "iarna-toml-esm";
import { watchImmediate } from "tauri-plugin-fs-watch-api";
import { useParams } from "react-router-dom";
import { debounce } from "lodash";

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

interface FlowContextInterface {
  nodes: Node[];
  edges: Edge[];
  onNodesChange: OnNodesChange;
  onEdgesChange: OnEdgesChange;
  onConnect: OnConnect;
  toml: string;
  addNode: () => void;
}

export const FlowContext = createContext<FlowContextInterface>({
  nodes: [],
  edges: [],
  onNodesChange: () => {},
  onEdgesChange: () => {},
  onConnect: () => {},
  toml: "",
  addNode: () => {},
});

export const useFlowContext = () => useContext(FlowContext);

export const FlowProvider = ({ children }: { children: ReactNode }) => {
  const { appDocuments } = useTauriContext();
  const { flow_name } = useParams();
  const [initalTomlLoaded, setInitialTomlLoaded] = useState<boolean>(false);

  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const [toml, setToml] = useState<string>("");

  const addNode = () => {
    const nextId = findNextNodeId(nodes);
    const newNode: Node = {
      id: nextId,
      type: "default",
      position: {
        x: Math.random() * 500,
        y: Math.random() * 500,
      },
      data: { label: `Node ${nextId}` },
    };
    setNodes((nodes) => [...nodes, newNode]);
  };

  const onNodesChange: OnNodesChange = (nodeChanges: NodeChange[]) => {
    console.log("onNodesChange", nodeChanges);
    setNodes((nodes) => applyNodeChanges(nodeChanges, nodes));
  };

  //When the edge is changed
  const onEdgesChange: OnEdgesChange = (edgeChanges: any) => {
    console.log("onEdgesChange", edgeChanges);
    setEdges((edges) => applyEdgeChanges(edgeChanges, edges));
  };

  //When a node is connected to an edge etc
  const onConnect: OnConnect = (params: any) => {
    console.log("onConnect", params);
    setEdges((edges) => addEdge(params, edges));
  };

  const readToml = async () => {
    try {
      if (!appDocuments || !flow_name) {
        throw new Error("appDocuments or flow_name is undefined");
      }
      console.log("reading toml in FlowProvider");
      return await readTextFile(
        appDocuments + "/flows/" + flow_name + "/flow.toml"
      );
    } catch (error) {
      console.log("error reading toml in FlowProvider", error);
    }
  };

  const writeToml = async (toml: string) => {
    if (!appDocuments || !flow_name) {
      throw new Error("appDocuments or flow_name is undefined");
    }
    console.log("writing toml in FlowProvider");
    return await writeTextFile(
      appDocuments + "/flows/" + flow_name + "/flow.toml",
      toml
    );
  };

  const loadToml = async () => {
    try {
      let new_toml = await readToml();
      // if (toml === new_toml) return; //don't update if the toml is the same
      if (!new_toml) {
        console.log("new_toml is undefined");
        setToml("");
        setNodes([]);
        setEdges([]);
        setInitialTomlLoaded(true);
      } else {
        console.log("setting toml in FlowProvider");
        setToml(new_toml);
        let parsedToml = parse(new_toml);
        console.log("parsedToml", parsedToml);

        setNodes(parsedToml.nodes as any);
        setEdges(parsedToml.edges as any);
        setInitialTomlLoaded(true);
      }
    } catch (error) {
      console.log("error loading toml in FlowProvider", error);
    }
  };

  //Load TOML into State the first time
  useEffect(() => {
    // console.log("Flow Name");
    if (flow_name && !initalTomlLoaded && appDocuments) {
      loadToml();
    }
  }, [flow_name, appDocuments, initalTomlLoaded]);

  //Update Toml as side effect of nodes and edges changing
  useEffect(() => {
    //this will probably have some duplications
    if (initalTomlLoaded) {
      const debouncedSave = debounce(({ edges, nodes }) => {
        // Replace this with the function you want to debounce
        // console.log(`Saving value ${newValue} to the database...`);
        let newToml = stringify({ nodes: nodes as any, edges: edges as any });
        console.log(
          "Updating toml in useEffect in flowProvider as side effect of drag editor"
        );
        //TODO: take numbers and make them not so long
        //TODO: allow for other fields to be added to the toml
        setToml(newToml);
        writeToml(newToml);
      }, 100); // we debounce with a delay of 500ms

      // Call the debounced function every time the value changes
      debouncedSave({ edges, nodes });

      // Cleanup function. This will be called when the component is unmounted or before the next effect function is called
      return () => {
        debouncedSave.cancel(); // Cancel any pending debounced function call
      };
    }
  }, [edges, nodes]);

  // useEffect(() => {
  //   if (!initalTomlLoaded) return;
  //   let stopWatching = () => {};
  //   let path = `${appDocuments}/flows/${flow_name}/flow.toml`;

  //   console.log(`Watching ${path} for changes`);

  //   const watchThisFile = async () => {
  //     stopWatching = await watchImmediate(path, (event) => {
  //       console.log(
  //         "File watchImmediate in FlowProvider triggered: ",
  //         JSON.stringify(event, null, 3)
  //       );
  //       console.log("Updating Node State as Side Effect of Updated TOML file");
  //       loadToml();
  //     });
  //   };

  //   watchThisFile();
  //   return () => {
  //     stopWatching();
  //   };
  // }, [initalTomlLoaded]);

  return (
    <FlowContext.Provider
      value={{
        nodes,
        edges,
        onConnect,
        onNodesChange,
        onEdgesChange,
        toml,
        addNode,
      }}
    >
      {children}
    </FlowContext.Provider>
  );
};
