import {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
  useCallback,
  useRef,
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

  // Loop through the nodes and find the maximum numeric ID value
  nodes.forEach((node: any) => {
    const numericId = parseInt(node.id, 10);
    if (!isNaN(numericId) && numericId > maxId) {
      maxId = numericId;
    }
  });
  // Increment the maxId to get the next ID for the new node
  const nextId = (maxId + 1).toString();

  return nextId;
}

type FlowFrontMatter = {
  name: string;
  id: string;
  version: string;
  author: string;
  description: string;
};

interface FlowContextInterface {
  nodes: Node[];
  edges: Edge[];
  flowFrontmatter: FlowFrontMatter | undefined;
  currentProcessingStatus: ProcessingStatus | undefined;
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

export const FlowContext = createContext<FlowContextInterface>({
  nodes: [],
  edges: [],
  flowFrontmatter: undefined,
  currentProcessingStatus: undefined,
  onNodesChange: () => {},
  onEdgesChange: () => {},
  onConnect: () => {},          
  onDragOver: () => {},
  onDrop: () => {},
  toml: "",
  addNode: () => {},
  setReactFlowInstance: () => {},
  updateFlowFrontmatter: () => {},
});

export const useFlowContext = () => useContext(FlowContext);

type ProcessingStatus = {
  message: String;
  event_id: String;
  node_id: String;
  flow_id: String;
  session_id: String;
};

type SessionComplete = {
  event_id: String;
  node_id: String;
  flow_id: String;
  session_id: String;
};

export const FlowProvider = ({ children }: { children: ReactNode }) => {
  const { appDocuments } = useTauriContext();
  const { renameFlowFiles } = useLocalFileContext();
  const { subscribeToEvent } = useEventLoopContext();
  const { flow_name } = useParams();
  const [initialTomlLoaded, setInitialTomlLoaded] = useState<boolean>(false);
  const [loadingToml, setLoadingToml] = useState<boolean>(false);
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);
  const [flowFrontmatter, setFlowFrontmatter] = useState<
    FlowFrontMatter | undefined
  >();
  const [toml, setToml] = useState<string>("");
  const [currentProcessingStatus, setCurrentProcessingStatus] = useState<
    ProcessingStatus | undefined
  >();
  const [sessionComplete, setSessionComplete] = useState<
    SessionComplete | undefined
  >();
  const [reactFlowInstance, setReactFlowInstance] =
    useState<ReactFlowInstance | null>(null);
  const timerRef = useRef<NodeJS.Timeout | null>(null);

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
      return [...nodes, newNode];
    });
  };

  const onNodesChange: OnNodesChange = (nodeChanges: NodeChange[]) => {
    console.log("onNodesChange nodeChanges", nodeChanges);
    setNodes((nodes) => {
      return applyNodeChanges(nodeChanges, nodes);
    });
  };

  const onEdgesChange: OnEdgesChange = (edgeChanges: EdgeChange[]) => {
    setEdges((edges) => {
      return applyEdgeChanges(edgeChanges, edges);
    });
  };

  const onConnect: OnConnect = (params: Connection) => {
    setEdges((edges) => {
      return addEdge(params, edges);
    });
  };

  const onDragOver = useCallback((event: DragEvent) => {
    event.preventDefault();
    if (event.dataTransfer === null) return;
    event.dataTransfer.dropEffect = "move";
  }, []);

  const onDrop = useCallback(
    (event: DragEvent, reactFlowWrapper: any) => {
      event.preventDefault();
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

  const updateFlowFrontmatter = async (
    flow_name: string,
    keysToUpdate: any
  ) => {
    try {
      //if we are updating name in TOML we also need to update the folder name
      if (keysToUpdate.name) {
        await renameFlowFiles(flow_name, keysToUpdate.name);
      }
      let flow_frontmatter = { ...flowFrontmatter, ...keysToUpdate };
      //TODO: check if name change causes race condition
      setFlowFrontmatter(flow_frontmatter);
    } catch (error) {
      console.log("error updating flow frontmatter", error);
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

  //we have heard there is new toml
  const updateStateFromToml = async () => {
    try {
      let new_toml = await readToml();
      if (!new_toml) throw new Error("new_toml is undefined");
      //don't update if nothing has changed in toml file
      if (new_toml === toml) return;
      setToml(new_toml);
      let parsedToml = parse(new_toml);

      if (!parsedToml.nodes) {
        parsedToml.nodes = [];
      }
      setNodes(parsedToml.nodes as any);
      if (!parsedToml.edges) {
        parsedToml.edges = [];
      }

      setNodes(parsedToml.nodes as any);
      setEdges(parsedToml.edges as any);
      setFlowFrontmatter(parsedToml.flow as FlowFrontMatter);
    } catch (error) {
      console.log("error loading toml in FlowProvider", error);
    }
  };

  useEffect(() => {
    const fetchData = async () => {
      if (flow_name && appDocuments && !initialTomlLoaded && !loadingToml) {
        console.log("hydrating initial TOML");
        setLoadingToml(true);
        await updateStateFromToml();
        setInitialTomlLoaded(true);
        setLoadingToml(false);
      }
    };

    fetchData();
  }, [flow_name, appDocuments, initialTomlLoaded]);

  //Debounced write state to toml
  useEffect(() => {
    // Clear any existing timers
    if (timerRef.current) {
      clearTimeout(timerRef.current);
    }

    // Set a new timer to write to TOML file
    timerRef.current = setTimeout(async () => {
      if (!initialTomlLoaded || loadingToml) return;

      let newToml = stringify({
        flow: flowFrontmatter as FlowFrontMatter,
        nodes: nodes as any,
        edges: edges as any,
      });
      console.log("writing to toml");
      console.log(newToml);
      //don't write if nothing has changed in react state
      if (newToml === toml) return;
      setToml(newToml);
      await writeToml(newToml);
    }, 200);

    // Clean up
    return () => {
      if (timerRef.current) {
        clearTimeout(timerRef.current);
      }
    };
  }, [nodes, edges, flowFrontmatter]);

  //Watch TOML file for changes
  useEffect(() => {
    if (!initialTomlLoaded) return;
    let stopWatching = () => {};
    let path = `${appDocuments}/flows/${flow_name}/flow.toml`;

    console.log(`Watching ${path} for changes`);

    const watchThisFile = async () => {
      stopWatching = await watchImmediate(path, (event) => {
        console.log("TOML file changed");
        updateStateFromToml();
      });
    };

    watchThisFile();
    return () => {
      stopWatching();
    };
  }, [initialTomlLoaded]);

  //Watch event processing for fun ui updates
  useEffect(() => {
    subscribeToEvent("event_processing", (event: any) => {
      setCurrentProcessingStatus(event);
    });
    subscribeToEvent("session_complete", (event: any) => {
      setSessionComplete(event);
    });
  }, []);

  return (
    <FlowContext.Provider
      value={{
        nodes,
        edges,
        flowFrontmatter,
        currentProcessingStatus,
        onConnect,
        onNodesChange,
        onEdgesChange,
        onDragOver,
        onDrop,
        toml,
        addNode,
        setReactFlowInstance,
        updateFlowFrontmatter,
      }}
    >
      {children}
    </FlowContext.Provider>
  );
};
