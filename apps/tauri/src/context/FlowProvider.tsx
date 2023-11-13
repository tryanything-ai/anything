import {
  createContext,
  ReactNode,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
} from "react";
import { useParams } from "react-router-dom";
import {
  addEdge,
  applyEdgeChanges,
  applyNodeChanges,
  Connection,
  Edge,
  EdgeChange,
  Node,
  NodeChange,
  OnConnect,
  OnEdgesChange,
  OnNodesChange,
  ReactFlowInstance,
} from "reactflow";

import { Action, Flow, FlowFrontMatter, Trigger } from "../utils/flowTypes";
import { ProcessingStatus, SessionComplete } from "../utils/eventTypes";

import api from "../tauri_api/api";
import { useFlowsContext } from "./FlowsProvider";
import { Node as FlowNode } from "../utils/flowTypes";

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

interface FlowContextInterface {
  nodes: Node[];
  edges: Edge[];
  flowVersions: Flow[];
  flowFrontmatter: FlowFrontMatter | undefined;
  currentProcessingStatus: ProcessingStatus | undefined;
  currentProcessingSessionId: string | undefined;
  onNodesChange: OnNodesChange;
  onEdgesChange: OnEdgesChange;
  onConnect: OnConnect;
  toml: string;
  getTrigger: () => Trigger | undefined;
  onDragOver: (event: any) => void;
  onDrop: (event: any, reactFlowWrapper: any) => void;
  addNode: (position: { x: number; y: number }, specialData?: any) => void;
  setReactFlowInstance: (instance: ReactFlowInstance | null) => void;
  updateFlowFrontmatter: (flow_name: string, keysToUpdate: any) => void;
}

export const FlowContext = createContext<FlowContextInterface>({
  toml: "",
  nodes: [],
  edges: [],
  flowVersions: [],
  flowFrontmatter: undefined,
  currentProcessingStatus: undefined,
  currentProcessingSessionId: undefined,
  onNodesChange: () => {},
  onEdgesChange: () => {},
  onConnect: () => {},
  onDragOver: () => {},
  onDrop: () => {},
  addNode: () => {},
  setReactFlowInstance: () => {},
  updateFlowFrontmatter: () => {},
  getTrigger: () => undefined,
});

export const useFlowContext = () => useContext(FlowContext);

export const FlowProvider = ({ children }: { children: ReactNode }) => {
  const { updateFlow } = useFlowsContext();
  const { flow_name } = useParams();
  const [hydrated, setHydrated] = useState<boolean>(false);
  const [firstLook, setFirstLook] = useState<boolean>(true);
  const [nodes, setNodes] = useState<Node[] | undefined>();
  const [edges, setEdges] = useState<Edge[] | undefined>();
  const [flowVersions, setFlowVersions] = useState<Flow[] | undefined>();
  const [flowFrontmatter, setFlowFrontmatter] = useState<
    FlowFrontMatter | undefined
  >();
  const [toml, setToml] = useState<string>("");
  // State for managing current processing for manual triggers and ebugging
  const [currentProcessingStatus, setCurrentProcessingStatus] = useState<
    ProcessingStatus | undefined
  >();
  const [currentProcessingSessionId, setCurrentProcessingSessionId] = useState<
    string | undefined
  >();
  const [sessionComplete, setSessionComplete] = useState<
    SessionComplete | undefined
  >();
  const [reactFlowInstance, setReactFlowInstance] =
    useState<ReactFlowInstance | null>(null);
  const timerRef = useRef<NodeJS.Timeout | null>(null);

  const addNode = (position: { x: number; y: number }, specialData?: any) => {
    const nextId = findNextNodeId(nodes);

    const newNode: Node = {
      id: nextId,
      type: "superNode",
      position,
      data: { ...specialData },
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

      const nodeData: FlowNode = JSON.parse(
        event.dataTransfer.getData("nodeData")
      );

      if (typeof nodeData === "undefined" || !nodeData) {
        return;
      }

      if (!reactFlowInstance) throw new Error("reactFlowInstance is undefined");

      let position = reactFlowInstance.project({
        x: event.clientX - reactFlowBounds.left,
        y: event.clientY - reactFlowBounds.top,
      });

      addNode(position, nodeData);
    },
    [addNode]
  );

  const updateFlowFrontmatter = async (
    flow_name: string,
    keysToUpdate: any
  ) => {
    try {
      // if we are updating name in TOML we also need to update the folder name
      if (keysToUpdate.name) {
        await updateFlow(flow_name, keysToUpdate.name);
      }
      let flow_frontmatter = { ...flowFrontmatter, ...keysToUpdate };
      setFlowFrontmatter(flow_frontmatter);
    } catch (error) {
      console.log("error updating flow frontmatter", error);
    }
  };

  const hydrateFlow = async () => {
    try {
      console.log("Fetch Flow By Name", flow_name);
      if (!flow_name) return;
      let { flow } = await api.flows.getFlowByName<any>(flow_name);
      console.log(
        "FLow Result in flow provider",
        JSON.stringify(flow, null, 3)
      );
      //save versions
      setFlowVersions(flow.versions);

      //TODO: add flows and edges for real from flow version
      setNodes([]);
      setEdges([]);

      let fm = flow;
      delete fm.versions;
      //TODO: gross fix thow we do this
      fm.version = flow.latest_version_id;
      console.log("FrontMatter saved", JSON.stringify(fm, null, 3));
      setFlowFrontmatter(fm);

      //TODO: maybe last edited to pull in the version they where looking at last?
      setHydrated(true);
      //TODO: get current version, maybe all versions
    } catch (e) {
      console.log("error in fetch flow", JSON.stringify(e, null, 3));
    }
  };

  const getTrigger = () => {
    if (!nodes) return undefined;
    let triggerNode = nodes.find((node) => node.data.trigger === true);
    return triggerNode.data;
  };

  //TODO: integrate here vs in flwos
  //   const readNodeConfig = async (
  //   flowName: string,
  //   nodeId: string,
  // ): Promise<Node | undefined> => {
  //   try {
  //     return await api.flows.readNodeConfig(nodeId, flowName);
  //   } catch (error) {
  //     console.log("error reading node config in FlowProvider", error);
  //   }
  // };

  // const writeNodeConfig = async (
  //   flowId: string,
  //   nodeId: string,
  //   data: any
  // ): Promise<Node | undefined> => {
  //   try {
  //     return api.flows.writeNodeConfig(flowId, nodeId, data);
  //   } catch (error) {
  //     console.log("error writing node config in FlowProvider", error);
  //   }
  // };

  const synchronise = async () => {
    try {
      let trigger;
      let actions = [];

      nodes.forEach((node) => {
        if (node.data.trigger === true) {
          trigger = { ...node.data, presentation: node.position } as Trigger;
        } else {
          actions.push({ ...node.data, presentation: node.position } as Action);
        }
      });

      let newFlow: Flow = {
        ...(flowFrontmatter as FlowFrontMatter),
        trigger,
        actions,
        edges: edges as Edge[],
      };

      console.log("New Flow Definition", newFlow);

      let res = await api.flows.updateFlowVersion(
        flowFrontmatter.flow_id,
        newFlow
      );

      console.log("res in updateFlowVersion", res);

      // console.log("writing to toml");
      // console.log(newToml);
      //don't write if nothing has changed in react state
      // if (newToml === toml) return;
      // setToml(newToml);
      // await writeToml(newToml);
      //TODO: updateFlow in Rust
    } catch (error) {
      console.log("error in synchronise", error);
    }
  };

  //Synchronise
  useEffect(() => {
    //Ugly but works to prevent write on load
    if (!hydrated) return;
    if (!nodes || !edges || !flowFrontmatter) return;
    if (firstLook) {
      setFirstLook(false);
      return;
    }
    // Clear any existing timers
    if (timerRef.current) {
      clearTimeout(timerRef.current);
    }

    // Set a new timer to write to TOML file
    timerRef.current = setTimeout(async () => {
      synchronise();
    }, 200);

    // Clean up
    return () => {
      if (timerRef.current) {
        clearTimeout(timerRef.current);
      }
    };
  }, [nodes, edges, flowFrontmatter]);

  //Watch event processing for fun ui updates
  useEffect(() => {
    let unlistenFromEventProcessing = api.subscribeToEvent(
      "event_processing",
      (event: any) => {
        setCurrentProcessingStatus(event);
      }
    );
    let unlistenSessionComplete = api.subscribeToEvent(
      "session_complete",
      (event: any) => {
        setSessionComplete(event);
      }
    );

    return () => {
      unlistenFromEventProcessing.then((unlisten) => unlisten());
      unlistenSessionComplete.then((unlisten) => unlisten());
    };
  }, [currentProcessingSessionId]);

  //Hydrate all flow data on navigation
  //User params fetches url params from React-Router-Dom
  useEffect(() => {
    if (!flow_name) return;
    hydrateFlow();
  }, [flow_name]);

  return (
    <FlowContext.Provider
      value={{
        nodes,
        edges,
        flowVersions,
        flowFrontmatter,
        currentProcessingStatus,
        currentProcessingSessionId,
        onConnect,
        onNodesChange,
        onEdgesChange,
        onDragOver,
        onDrop,
        toml,
        addNode,
        getTrigger,
        setReactFlowInstance,
        updateFlowFrontmatter,
      }}
    >
      {children}
    </FlowContext.Provider>
  );
};
