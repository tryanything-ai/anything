"use client";

import {
  createContext,
  ReactNode,
  useCallback,
  useContext,
  useEffect,
  useState,
} from "react";

import { useParams, useRouter } from "next/navigation";
import { cloneDeep, debounce } from "lodash";

import {
  addEdge,
  applyEdgeChanges,
  applyNodeChanges,
  Edge,
  EdgeChange,
  EdgeSelectionChange,
  Node,
  NodeChange,
  NodeSelectionChange,
  OnConnect,
  OnEdgesChange,
  OnNodesChange,
  ReactFlowInstance,
  getIncomers,
  getOutgoers,
  getConnectedEdges,
} from "reactflow";

import api from "@/lib/anything-api";
import { Action, Workflow } from "@/types/workflows";

import { findConflictFreeId } from "@/lib/studio/helpers";

export enum PanelTab {
  SETTINGS = "settings",
  CONFIG = "config",
  TESTING = "testing",
}

export enum SavingStatus {
  SAVING = "saving...",
  SAVED = "saved!",
  NONE = "",
}

export type UpdateWorklowArgs = {
  flow_name?: string;
  description?: string;
  active?: boolean;
};

export interface WorkflowVersionContextInterface {
  db_flow_version_id: string;
  db_flow_id: string;
  db_flow: any;
  db_flow_version: any;
  flow_version_definition: any;
  selected_node_id: string;
  selected_node_data: Action | undefined;
  selected_node_variables: any;
  selected_node_variables_schema: any;
  panel_tab: string;
  savingStatus: string;
  detailedMode: boolean;
  setDetailedMode: (mode: boolean) => void;
  setPanelTab: (tab: string) => void;
  showingActionSheet: boolean;
  setShowingActionSheet: (showing: boolean) => void;
  showActionSheetForEdge: (id: string) => void;
  showActionSheet: () => void;
  nodes: Node[];
  edges: Edge[];
  onNodesChange: OnNodesChange;
  onEdgesChange: OnEdgesChange;
  onConnect: OnConnect;
  addNode: (node_data: any, position: { x: number; y: number }) => void;
  setReactFlowInstance: (instance: ReactFlowInstance | null) => void;
  deleteNode: (id: string) => void;
  updateNodeData: (update_key: string[], data: any[]) => Promise<boolean>;
  updateWorkflow: (args: UpdateWorklowArgs) => Promise<void>;
  publishWorkflowVersion: () => Promise<void>;
}

export const WorkflowVersionContext =
  createContext<WorkflowVersionContextInterface>({
    db_flow_version_id: "",
    db_flow_id: "",
    db_flow: {},
    db_flow_version: {},
    flow_version_definition: {},
    selected_node_id: "",
    selected_node_data: undefined,
    selected_node_variables: null,
    selected_node_variables_schema: null,
    panel_tab: PanelTab.CONFIG,
    savingStatus: SavingStatus.NONE,
    setPanelTab: () => {},
    showingActionSheet: false,
    detailedMode: false,
    setDetailedMode: () => {},
    showActionSheetForEdge: () => {},
    setShowingActionSheet: () => {},
    showActionSheet: () => {},
    nodes: [],
    edges: [],
    onNodesChange: () => {},
    onEdgesChange: () => {},
    onConnect: () => {},
    addNode: () => {},
    setReactFlowInstance: () => {},
    deleteNode: () => {},
    updateNodeData: async () => false,
    updateWorkflow: async () => {},
    publishWorkflowVersion: async () => {},
  });

export const useWorkflowVersionContext = () =>
  useContext(WorkflowVersionContext);

export const WorkflowVersionProvider = ({
  children,
}: {
  children: ReactNode;
}): JSX.Element => {
  const { workflowId, workflowVersionId } = useParams<{
    workflowId: string;
    workflowVersionId: string;
  }>();

  const router = useRouter();

  // const { getWorkflowById } = useWorkflowsContext();

  //Easy Access State
  const [dbFlow, setDbFlow] = useState<any>({});
  const [dbFlowVersion, setDbFlowVersion] = useState<any>({});
  const [flow_version_definition, setFlowVersionDefinition] = useState<any>({});

  //Easy Access Id's
  const [dbFlowVersionId, setDbFlowVersionId] = useState<string>("");
  const [dbFlowId, setDbFlowId] = useState<string>("");
  const [selectedNodeId, setSelectedNodeId] = useState<string>("");
  const [selectedNodeData, setSelectedNodeData] = useState<Action | undefined>(
    undefined,
  );
  const [selectedNodeVariables, setSelectedNodeVariables] = useState<any>({});
  const [selectedNodeVariablesSchema, setSelectedNodeVariablesSchema] =
    useState<any>({});
  const [detailedMode, setDetailedMode] = useState<boolean>(false);
  //React Flow State
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);

  //Navigation State
  const [panel_tab, setPanelTab] = useState<string>(PanelTab.CONFIG);
  //Saving State
  const [savingStatus, setSavingStatus] = useState<string>(SavingStatus.NONE);
  //Action sheet for adding nodes
  const [showingActionSheet, setShowingActionSheet] = useState<boolean>(false);
  const [actionSheetEdge, setActionSheetEdge] = useState<string>("");

  const [reactFlowInstance, setReactFlowInstance] =
    useState<ReactFlowInstance | null>(null);

  const showActionSheetForEdge = (id: string) => {
    console.log("Show Action Sheet for Edge: ", id);
    setShowingActionSheet(true);
    setActionSheetEdge(id);
  };

  const showActionSheet = () => {
    console.log("Show Action Sheet");
    setShowingActionSheet(true);
    setActionSheetEdge("");
  };

  const resetState = () => {
    console.log("Resetting State in WorkflowVersionProvider");
    setNodes([]);
    setEdges([]);
    setDbFlowVersionId("");
    setDbFlowId("");
    setDbFlow({});
    setDbFlowVersion({});
    setFlowVersionDefinition({});
    setSelectedNodeId("");
    setSelectedNodeData(undefined);
    setSelectedNodeVariables({});
    setSelectedNodeVariablesSchema({});
  };

  const addNode = (node_data: any, position?: { x: number; y: number }) => {
    //call helper function if edge_id is present
    if (actionSheetEdge && actionSheetEdge !== "") {
      console.log("Adding Node at Edge", actionSheetEdge);
      addActionTemplateAtEdge(actionSheetEdge, node_data);
      return;
    }

    let planned_node_id;

    console.log("Node Data", node_data);
    //set node_id
    if (node_data) {
      planned_node_id = node_data.node_id;
    }

    if (!position) {
      position = { x: 300, y: 300 };
    }

    const conflictFreeId = findConflictFreeId(nodes, planned_node_id);
    console.log("conflictFreeId", conflictFreeId);
    console.log("special data", node_data);
    const newNode: Node = {
      id: conflictFreeId,
      type: "anything",
      position,
      data: { ...node_data, node_id: conflictFreeId },
    };

    let udpatedNodes = [...nodes, newNode];

    saveFlowVersionImmediate(udpatedNodes, edges);

    setNodes(() => udpatedNodes);
  };

  const updateWorkflow = async (args: UpdateWorklowArgs) => {
    try {
      if (!dbFlowId) return;

      console.log("Updating Workflow", args);

      //Save to cloud
      await api.flows.updateFlow(dbFlowId, args);

      //Update state here
      setDbFlow((prevFlow: any) => {
        return {
          ...prevFlow,
          ...args,
        };
      });
    } catch (error) {
      console.error(error);
    } finally {
    }
  };

  const publishWorkflowVersion = async () => {
    try {
      if (!dbFlowId) return;
      if (!dbFlowVersionId) return;

      //Save to cloud
      await api.flows.publishFlowVersion(dbFlowId, dbFlowVersionId);

      //Update state here
      setDbFlowVersion((prevFlow: any) => {
        return {
          ...prevFlow,
          published: true,
          published_at: new Date().toISOString(),
        };
      });
    } catch (error) {
      console.error(error);
    }
  };

  const addActionTemplateAtEdge = (id: string, action_template: any) => {
    const newNodes = cloneDeep(nodes);
    const newEdges = cloneDeep(edges);

    const edge = newEdges.find((edge) => edge.id === id);
    if (!edge) return;

    const { source, target } = edge;

    const planned_node_id = action_template.node_id;
    const conflictFreeId = findConflictFreeId(newNodes, planned_node_id);

    const sourceNode = newNodes.find((node) => node.id === source);
    const targetNode = newNodes.find((node) => node.id === target);

    if (!sourceNode || !targetNode) return;
    // Determine the new node position, for simplicity place it at the middle of source and target nodes
    const position = {
      x: (sourceNode.position.x + targetNode.position.x) / 2,
      y: (sourceNode.position.y + targetNode.position.y) / 2,
    };

    const newNode = {
      id: conflictFreeId,
      type: "anything",
      position,
      data: { ...action_template, node_id: conflictFreeId },
    };

    // Add the new node to the nodes array
    newNodes.push(newNode);

    // Create new edges connecting the new node
    const newEdge1 = {
      id: `${source}->${conflictFreeId}`,
      source: source,
      target: conflictFreeId,
      sourceHandle: "b",
      targetHandle: "a",
      type: "anything",
    };

    const newEdge2 = {
      id: `${conflictFreeId}->${target}`,
      source: conflictFreeId,
      target: target,
      sourceHandle: "b",
      targetHandle: "a",
      type: "anything",
    };

    // Remove the original edge and add the new edges
    const updatedEdges = newEdges.filter((edge) => edge.id !== id);
    updatedEdges.push(newEdge1, newEdge2);

    saveFlowVersionImmediate(newNodes, updatedEdges);

    setNodes(() => newNodes);
    setEdges(() => updatedEdges);
  };

  const deleteNode = (id: string) => {
    let new_nodes = cloneDeep(nodes);
    let new_edges = cloneDeep(edges);

    const node = new_nodes.find((node) => node.id === id);
    if (!node) return;

    const incomers = getIncomers(node, new_nodes, new_edges);
    const outgoers = getOutgoers(node, new_nodes, new_edges);
    const connectedEdges = getConnectedEdges([node], new_edges);

    const remainingEdges = new_edges.filter(
      (edge) => !connectedEdges.includes(edge),
    );

    const createdEdges = incomers.flatMap(({ id: source }) =>
      outgoers.map(({ id: target }, index: number) => ({
        id: `${source}->${target}_${index}`,
        source,
        sourceHandle: "b",
        targetHandle: "a",
        target,
        type: "anything", // Ensure new edges have the correct type
      })),
    );

    new_nodes = new_nodes.filter((node) => node.id !== id);
    new_edges = [...remainingEdges, ...createdEdges];

    saveFlowVersionImmediate(new_nodes, new_edges);

    setNodes(new_nodes);
    setEdges(new_edges);
  };

  const fanOutLocalSelectedNodeData = (node: any) => {
    console.log("Fan Out Local Node Data", node);

    if (node?.id) {
      setSelectedNodeId(() => node.id);
    } else {
      setSelectedNodeId(() => "");
      console.log("No Node Id in Fan Out Local Node Data");
    }
    if (node?.data) {
      setSelectedNodeData(() => node.data);
    } else {
      setSelectedNodeData(() => undefined);
      console.log("No Node Data in Fan Out Local Node Data");
    }
    if (node?.data?.variables) {
      setSelectedNodeVariables(() => node.data.variables);
    } else {
      setSelectedNodeVariables(() => null);
      console.log("No Node Variables in Fan Out Local Node Data");
    }
    if (node?.data?.variables_schema) {
      setSelectedNodeVariablesSchema(() => node.data.variables_schema);
    } else {
      setSelectedNodeVariablesSchema(() => null);
      console.log("No Node Variables Schema in Fan Out Local Node Data");
    }
  };

  const set_panel_tab = (tab: string) => {
    //Used to make nice navigation in side panel
    if (tab === PanelTab.SETTINGS) {
      // setSelectedNodeData(undefined);
      // setSelectedNodeId("");
      fanOutLocalSelectedNodeData(null);

      //Trick to clear selection inisde ReactFlow
      onNodesChange([
        {
          id: selectedNodeId,
          type: "select",
          selected: false,
        },
      ]);
    }
    setPanelTab(tab);
  };

  const onNodesChange: OnNodesChange = (nodeChanges: NodeChange[]) => {
    console.log("onNodesChange nodeChanges", nodeChanges);

    //find the node with selected = true
    let selectionChanges: NodeSelectionChange[] = nodeChanges.filter(
      (nodeChange) => nodeChange.type === "select",
    ) as NodeSelectionChange[];

    // let nonDimmensionChanges = nodeChanges.filter((nodeChange) => nodeChange.type !== "dimensions") as NodeSelectionChange[];
    // get the id of the node with selected = true
    if (selectionChanges.length > 0) {
      console.log("selectionChanges", selectionChanges);
      let selectedNode: any = selectionChanges.find(
        (nodeChange: NodeSelectionChange) => nodeChange.selected,
      );

      if (selectedNode) {
        //Set node and node data for easy access
        let selectedNodeObj: any = nodes.find(
          (node) => node.id === selectedNode.id,
        );
        fanOutLocalSelectedNodeData(selectedNodeObj);
        setPanelTab(PanelTab.CONFIG);
      } else {
        fanOutLocalSelectedNodeData(null);
      }
    }

    let new_nodes = applyNodeChanges(nodeChanges, nodes);
    console.log("new_nodes", new_nodes);

    let unPersistedChanges: NodeSelectionChange[] = nodeChanges.filter(
      (nodeChange) =>
        nodeChange.type === "dimensions" || nodeChange.type === "select",
    ) as NodeSelectionChange[];

    if (unPersistedChanges.length === 0) {
      console.log(
        "Saving Node Update because not dimmension or select changes",
      );
      saveFlowVersionDebounced(new_nodes, edges);
    } else {
      console.log("Skipping Save because dimmension or select changes");
    }

    setNodes((nodes) => {
      return new_nodes;
    });
  };

  const onEdgesChange: OnEdgesChange = (edgeChanges: EdgeChange[]) => {
    let new_edges = applyEdgeChanges(edgeChanges, edges);

    //find the node with selected = true
    let selectionChanges: EdgeSelectionChange[] = edgeChanges.filter(
      (edgeChange) => edgeChange.type === "select",
    ) as EdgeSelectionChange[];

    if (selectionChanges.length === 0) {
      console.log("Saving Edge Update because not select changes");
      saveFlowVersionDebounced(nodes, new_edges);
    } else {
      console.log("Skipping Save because select changes");
    }

    setEdges((edges) => {
      console.log("onEdgesChange edgeChanges", edgeChanges);
      return new_edges;
    });
  };

  const onConnect: OnConnect = (params: any) => {
    params.type = "anything";
    let new_edges = addEdge(params, edges);

    saveFlowVersionDebounced(nodes, new_edges);

    setEdges(() => {
      return new_edges;
    });
  };

  const updateNodeData = async (
    update_key: string[],
    data: any[],
  ): Promise<boolean> => {
    try {
      console.log("updateNodeData:", update_key, data);

      // setNodes((prevNodes) => {
      const newNodes = cloneDeep(nodes);

      let updatedNodes = newNodes.map((node) => {
        if (node.id === selectedNodeId) {
          update_key.forEach((key, index) => {
            console.log(
              `Updating Node Data in updateNodeData for ${node.id}:${key} with:`,
              data[index],
            );
            node.data[key] = data[index];
          });
          console.log("NEW_DATA_FOR_NODE", node.data);
          fanOutLocalSelectedNodeData(node);
        }
        return node;
      });

      console.log("ALL_NEW_NODE_DATA", updatedNodes);

      // Call saveFlowVersion with the latest state
      saveFlowVersionImmediate(updatedNodes, edges);

      // Update the state with the latest state
      setNodes(updatedNodes);

      return true;
    } catch (error) {
      console.log(
        "error writing node config in WorkflowVersionProvider",
        error,
      );
      return false;
    }
  };

  const makeUpdateFlow = (nodes: any, edges: any) => {
    let new_nodes = cloneDeep(nodes);
    return {
      actions: new_nodes.map((node: any) => {
        return {
          ...node.data,
          presentation: {
            position: node.position,
          },
        };
      }),
      edges: edges,
    };
  };

  const saveFlowVersionDebounced = async (nodes: any, edges: any) => {
    try {
      setSavingStatus(SavingStatus.SAVING);
      await _debouncedSaveFlowVersion(makeUpdateFlow(nodes, edges));
    } catch (error) {
      console.log("error in saveFlowVersion", error);
    }
  };

  const saveFlowVersionImmediate = (nodes: any, edges: any) => {
    const workflow = makeUpdateFlow(nodes, edges);
    setSavingStatus(SavingStatus.SAVING);
    _saveFlowVersion(workflow);
  };

  const _saveFlowVersion = async (workflow: Workflow) => {
    try {
      const res: any = await api.flows.updateFlowVersion(
        dbFlowId,
        dbFlowVersionId,
        workflow,
      );

      console.log("Flow Version Saved!");

      let returned_flow = JSON.parse(res)[0];

      console.log("Returned Flow", returned_flow);

      if (returned_flow.flow_version_id !== dbFlowVersionId) {
        console.log("Flow Version Ids DO NOT match.");
        console.log("Update on published flow generated a NEW DRAFT version");

        // setDbFlowVersionId(returned_flow.flow_version_id);
        // setDbFlowVersion(returned_flow);
        // setFlowVersionDefinition(returned_flow.flow_definition);

        // Set Next.js rout e to workflows/workflow_id/version/version_id
        // const newRoute = `/workflows/${dbFlowId}/version/${returned_flow.flow_version_id}`;
        // window.history.pushState(null, '', newRoute);
        router.replace(
          `/workflows/${dbFlowId}/${returned_flow.flow_version_id}/editor`,
        );
      } else {
        // console.log("Flow Version Ids match.");
        console.log("Updated existing draft flow version");
      }

      setSavingStatus(SavingStatus.SAVED);
      setTimeout(() => setSavingStatus(SavingStatus.NONE), 1500); // Clear the status after 2 seconds
      // NOTES:
      // await hydrateFlow(); //This means we would have to hand manage syncing of
      //things like selected node and its dependences like selected_node_data etc etc
      //Maybe difficult!
    } catch (error) {
      console.log("error in saveFlowVersion", error);
    }
  };

  const _debouncedSaveFlowVersion = useCallback(
    debounce(async (workflow: Workflow) => {
      try {
        await _saveFlowVersion(workflow);
      } catch (error) {
        console.log("error in saveFlowVersion", error);
      }
    }, 1000),
    [dbFlowId, dbFlowVersionId],
  );

  const hydrateFlow = async () => {
    try {
      console.log("Fetch Flow By Id in new hydrate flow: ", workflowId);
      if (!workflowId) return;
      let workflow_response = await api.flows.getFlow(workflowId);
      // let workflow_response = await getWorkflowById(workflowId);

      if (!workflow_response) return;
      let flow = workflow_response[0];
      let flow_version = flow.flow_versions[0];
      console.log("New Hydreate in Workflow Provider", flow);
      console.log("Version in New Hydrate Flow", flow_version);
      console.log(
        "Flow Definition in New Hydrate Flow",
        flow_version.flow_definition,
      );

      setDbFlow(flow);

      setDbFlowVersion(flow_version);
      setFlowVersionDefinition(flow_version.flow_definition);
      if (flow_version && flow_version.flow_definition) {
        if (
          flow_version.flow_definition.actions &&
          flow_version.flow_definition.actions.length !== 0
        ) {
          let _nodes: Node[] = flow_version.flow_definition.actions.map(
            (action: any) => {
              let position = action.presentation?.position || { x: 0, y: 0 };

              return {
                position,
                data: action,
                id: action.node_id,
                type: "anything",
              };
            },
          );
          console.log("Nodes in hydrate flow", _nodes);
          setNodes(_nodes);
        } else {
          console.log("SKIPPING: No Actions in Flow Definition in hydrateFlow");
        }

        if (
          flow_version.flow_definition.edges &&
          flow_version.flow_definition.edges.length !== 0
        ) {
          let _edges: Edge[] = flow_version.flow_definition.edges.map(
            (edge: any) => {
              return edge;
            },
          );
          console.log("Edges in hydrate flow", _edges);
          setEdges(_edges);
        } else {
          console.log("SKIPPING: No Edges in Flow Definition in hydrateFlow");
        }

        console.log("Hydrated Flow");
        // setHydrated(true);
      } else {
        console.log("No Flow Definition in Flow Version in hydrateFlow");
      }
    } catch (e) {
      console.log("error in fetch flow", JSON.stringify(e, null, 3));
    }
  };

  //Hydrate on Navigation and clean on remove
  useEffect(() => {
    //TODO: why does this always write when we start the flow? How can we prevent this?
    if (!workflowId || !workflowVersionId) return;
    resetState();
    setDbFlowVersionId(workflowVersionId);
    setDbFlowId(workflowId);
    hydrateFlow();
    return () => {
      resetState();
    };
  }, [workflowId, workflowVersionId]);
  // }, []);

  return (
    <WorkflowVersionContext.Provider
      value={{
        db_flow_id: dbFlowId,
        db_flow_version_id: dbFlowVersionId,
        db_flow: dbFlow,
        db_flow_version: dbFlowVersion,
        flow_version_definition,
        selected_node_id: selectedNodeId,
        selected_node_data: selectedNodeData,
        selected_node_variables: selectedNodeVariables,
        selected_node_variables_schema: selectedNodeVariablesSchema,
        nodes,
        edges,
        savingStatus,
        panel_tab,
        showingActionSheet,
        detailedMode,
        setDetailedMode,
        setShowingActionSheet,
        showActionSheetForEdge,
        showActionSheet,
        setPanelTab: set_panel_tab,
        onConnect,
        onNodesChange,
        onEdgesChange,
        addNode,
        deleteNode,
        setReactFlowInstance,
        updateNodeData,
        updateWorkflow,
        publishWorkflowVersion,
      }}
    >
      {children}
    </WorkflowVersionContext.Provider>
  );
};
