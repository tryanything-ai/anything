import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
  useCallback,
} from "react";

import {
  // ReactFlowInstance,
  Project,
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
  ReactFlowInstance,
} from "reactflow";
import { useTauriContext } from "./TauriProvider";
import { readTextFile, writeTextFile } from "@tauri-apps/api/fs";
import { stringify, parse } from "iarna-toml-esm";
import { watchImmediate } from "tauri-plugin-fs-watch-api";
import { useParams } from "react-router-dom";
import { useLocalFileContext } from "./LocalFileProvider";
import { useEventLoopContext } from "./EventLoopProvider";

function findNextNodeId(nodes: any): string {
  // Return 1 if there are no nodes
  if (!nodes) {
    console.log("no nodes in FindNextNodeId, returning id 1");
    return "1";
  }
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
  flowFrontmatter: any;
  onNodesChange: OnNodesChange;
  onEdgesChange: OnEdgesChange;
  onConnect: OnConnect;
  toml: string;
  onDragOver: (event: any) => void;
  onDrop: (event: any, reactFlowWrapper: any) => void;
  addNode: (type: string, specialData?: any) => void;
  setReactFlowInstance: (instance: ReactFlowInstance | null) => void;
  updateFlowFrontmatter: (flow_name: string, keysToUpdate: any) => void;
}

//BUG: sometimes whole toml gets deleted if we wiggle too many things. race condition?
//BUG: sometimes edges aren't deleted when we delete a node that is connected.
export const FlowContext = createContext<FlowContextInterface>({
  nodes: [],
  edges: [],
  flowFrontmatter: {},
  onNodesChange: () => {},
  onEdgesChange: () => {},
  onConnect: () => {},
  onDragOver: () => {},
  onDrop: () => {},
  toml: "",
  addNode: () => {},
  setReactFlowInstance: () => { },
  updateFlowFrontmatter: () => { },
});

export const useFlowContext = () => useContext(FlowContext);

export const FlowProvider = ({ children }: { children: ReactNode }) => {
  const { appDocuments } = useTauriContext();
  const { renameFlowFiles } = useLocalFileContext();
  const { subscribeToEvent } = useEventLoopContext(); 
  const { flow_name } = useParams();
  const [initalTomlLoaded, setInitialTomlLoaded] = useState<boolean>(false);
  const [loadingToml, setLoadingToml] = useState<boolean>(false);
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const [flowFrontmatter, setFlowFrontmatter] = useState<any>({});
  const [toml, setToml] = useState<string>("");
  const [reactFlowInstance, setReactFlowInstance] =
    useState<ReactFlowInstance | null>(null);

  const addNode = (
    type: string,
    position: { x: number; y: number },
    specialData?: any
  ) => {
    const nextId = findNextNodeId(nodes);
    const newNode: Node = {
      id: nextId,
      type,
      position,
      data: { label: `Node ${nextId}`, ...specialData },
    };

    setNodes((nodes) => {
      console.log(
        "nodes in setNodes in addNode" + JSON.stringify(nodes, null, 3)
      );
      return [...nodes, newNode];
    });
  };

  //TODO: some sort of bug when we have no nodes but we don't remove all edges
  const onNodesChange: OnNodesChange = (nodeChanges: NodeChange[]) => {
    console.log("onNodesChange nodeChanges", nodeChanges);
    setNodes((nodes) => {
      let new_nodes = applyNodeChanges(nodeChanges, nodes);
      let new_toml = stringify({
        flow: flowFrontmatter,
        nodes: new_nodes as any,
        edges: edges as any,
      });
      writeToml(new_toml);
      return new_nodes;
    });
  };

  //When the edge is changed
  const onEdgesChange: OnEdgesChange = (edgeChanges: any) => {
    setEdges((edges) => {
      let new_edges = applyEdgeChanges(edgeChanges, edges);
      let new_toml = stringify({
        flow: flowFrontmatter,
        nodes: nodes as any,
        edges: new_edges as any,
      });
      writeToml(new_toml);
      return new_edges;
    });
  };
  //TODO: need to protect against "undefined" as a state we sync anywhere.

  //When a node is connected to an edge etc
  const onConnect: OnConnect = (params: any) => {
    console.log("onConnect params", params);
    //TODO: protect against multiple connections to the same node
    setEdges((edges) => {
      let new_edges = addEdge(params, edges);
      let new_toml = stringify({
        flow: flowFrontmatter,
        nodes: nodes as any,
        edges: new_edges as any,
      });
      writeToml(new_toml);
      return new_edges;
    });
  };

  const onDragOver = useCallback((event: DragEvent) => {
    event.preventDefault();
    if (event.dataTransfer === null) return;
    console.log("dragging over");
    event.dataTransfer.dropEffect = "move";
  }, []);

  const onDrop = useCallback(
    (event: DragEvent, reactFlowWrapper: any) => {
      event.preventDefault();
      console.log("onDrop event", event);

      const reactFlowBounds = reactFlowWrapper.current.getBoundingClientRect();
      if (event.dataTransfer === null) return;
      const nodeType = event.dataTransfer.getData("nodeType");
      const nodeData = JSON.parse(event.dataTransfer.getData("nodeData"));
      const specialData = JSON.parse(event.dataTransfer.getData("specialData"));

      if (typeof nodeType === "undefined" || !nodeType) {
        return;
      }
      if (typeof nodeData === "undefined" || !nodeData) {
        return;
      }
      if (typeof specialData === "undefined" || !specialData) {
        return;
      }
   
      if (!reactFlowInstance) throw new Error("reactFlowInstance is undefined");

      let position = reactFlowInstance.project({
        x: event.clientX - reactFlowBounds.left,
        y: event.clientY - reactFlowBounds.top,
      });

      addNode(nodeType, position, { ...nodeData, ...specialData });
    },
    [addNode]
  );

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

  const writeToml = async (toml: string, explicit_flow_name?: string) => {
    if (!appDocuments || !flow_name) {
      throw new Error("appDocuments or flow_name is undefined");
    }
    console.log("writing toml in FlowProvider");
    let name = explicit_flow_name ? explicit_flow_name : flow_name;
    return await writeTextFile(
      appDocuments + "/flows/" + name + "/flow.toml",
      toml
    );
  };

  //we have heard there is new toml
  const updateToml = async () => {
    try {
      let new_toml = await readToml();
      if (toml === new_toml) return; //don't update if the toml is the same
      if (!new_toml) {
        console.log("new_toml is undefined in updateToml");
        setToml("");
        setNodes([]);
        setEdges([]);
      } else {
        console.log("updating toml from file watcher");
        setToml(new_toml);
        let parsedToml = parse(new_toml);
        console.log("parsedToml", parsedToml);

        if (!parsedToml.nodes) {
          console.log("no nodes in parsedToml");
          parsedToml.nodes = [];
        }
        setNodes(parsedToml.nodes as any);
        if (!parsedToml.edges) {
          console.log("no edges in parsedToml");
          parsedToml.edges = [];
        }
        setEdges(parsedToml.edges as any);
        setFlowFrontmatter(parsedToml.flow);
      }
    } catch (error) {
      console.log("error loading toml in FlowProvider", error);
    }
  };

  const loadTomlOnStart = async () => {
    try {
      setLoadingToml(true);
      let new_toml = await readToml();
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

        //TODO: this should be a function since we do it here and in UpdateToml
        if (!parsedToml.nodes) {
          console.log("no nodes in parsedToml");
          parsedToml.nodes = [];
        }
        setNodes(parsedToml.nodes as any);
        if (!parsedToml.edges) {
          console.log("no edges in parsedToml");
          parsedToml.edges = [];
        }

        setNodes(parsedToml.nodes as any);
        setEdges(parsedToml.edges as any);
        //TODO: handle missing frontmatter
        setFlowFrontmatter(parsedToml.flow);
        setInitialTomlLoaded(true);
      }
    } catch (error) {
      console.log("error loading toml in FlowProvider", error);
    }
  };

  const updateFlowFrontmatter = async (flow_name: string, keysToUpdate: any) => {
    try {

      //if we are updating name we also need to update the folder name
      if(keysToUpdate.name) {
       await renameFlowFiles(flow_name, keysToUpdate.name);
      }

      let flow_frontmatter = { ...flowFrontmatter, ...keysToUpdate };

      let newToml = stringify({
        flow: flow_frontmatter,
        nodes: nodes as any,
        edges: edges as any,
      });

      //write to file
      setToml(newToml);
      setFlowFrontmatter(flow_frontmatter);
      //TODO: code smell. we write to file and add the "explicity fileName" because we don't know how navigation will effect this
      //since writeToml uses navigation state to manage teh file name to write too. 
      await writeToml(newToml, keysToUpdate.name);

   
      //TODO: change file name if the kye is flow_name changes
      // renameFlowFiles(keysToUpdate.flow_name, )
    } catch (error) {
      console.log("error updating flow frontmatter", error);
    }
  };

 
  //listen to event to show processing state in nodes
  useEffect(() => {
    subscribeToEvent("event_processing", (event: any) => {
      console.log("received event about processing");
      console.log(event);
      // setLoading(true);
      // setProgress(event.progress);
      // setMessage(event.message);
    });
  }, []);

  //Load TOML into State the first time
  useEffect(() => {
    if (flow_name && !initalTomlLoaded && appDocuments && !loadingToml) {
      loadTomlOnStart();
    }
  }, [flow_name, appDocuments, initalTomlLoaded]);

  useEffect(() => {
    if (!initalTomlLoaded) return;
    let stopWatching = () => {};
    let path = `${appDocuments}/flows/${flow_name}/flow.toml`;

    console.log(`Watching ${path} for changes`);

    const watchThisFile = async () => {
      stopWatching = await watchImmediate(path, (event) => {
        console.log(
          "File watchImmediate in FlowProvider triggered: ",
          JSON.stringify(event, null, 3)
        );
        console.log(
          "Want to update Node State as Side Effect of Updated TOML file"
        );
        updateToml();
      });
    };

    watchThisFile();
    return () => {
      stopWatching();
    };
  }, [initalTomlLoaded]);

  return (
    <FlowContext.Provider
      value={{
        nodes,
        edges,
        flowFrontmatter,
        onConnect,
        onNodesChange,
        onEdgesChange,
        onDragOver,
        onDrop,
        toml,
        addNode,
        setReactFlowInstance,
        updateFlowFrontmatter
      }}
    >
      {children}
    </FlowContext.Provider>
  );
};
