"use client";

import {
  createContext,
  ReactNode,
  useCallback,
  useContext,
  useEffect,
  useState,
  useMemo,
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
  getIncomers,
  getOutgoers,
  getConnectedEdges,
} from "reactflow";

import api from "@repo/anything-api";
import { Action, Workflow } from "@/types/workflows";

import { findConflictFreeId } from "@/lib/studio/helpers";
import { useAccounts } from "./AccountsContext";
import { useWorkflowVersionControl } from "./WorkflowVersionControlContext";

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
  selected_action_id: string;
  selected_node_data: Action | undefined;
  selected_node_variables: any;
  selected_node_variables_schema: any;
  selected_node_input: any;
  selected_node_input_schema: any;
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
    selected_action_id: "",
    selected_node_data: undefined,
    selected_node_variables: null,
    selected_node_variables_schema: null,
    selected_node_input: null,
    selected_node_input_schema: null,
    panel_tab: PanelTab.CONFIG,
    savingStatus: SavingStatus.NONE,
    setPanelTab: () => {},
    showingActionSheet: false,
    detailedMode: true,
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
    deleteNode: () => {},
    updateNodeData: async () => false,
    updateWorkflow: async () => {},
    publishWorkflowVersion: async () => {},
  });

export const useWorkflowVersion = () => useContext(WorkflowVersionContext);

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
  const { selectedAccount } = useAccounts();
  const { refresh } = useWorkflowVersionControl();

  //Easy Access State
  const [dbFlow, setDbFlow] = useState<any>({});
  const [dbFlowVersion, setDbFlowVersion] = useState<any>({});
  const [flow_version_definition, setFlowVersionDefinition] = useState<any>({});

  //Easy Access Id's
  const [dbFlowVersionId, setDbFlowVersionId] = useState<string>("");
  const [dbFlowId, setDbFlowId] = useState<string>("");
  const [selectedNodeId, setSelectedNodeId] = useState<string>("");
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
  };

  const addNode = (node_data: any, position?: { x: number; y: number }) => {
    //call helper function if edge_id is present
    if (actionSheetEdge && actionSheetEdge !== "") {
      console.log("Adding Node at Edge", actionSheetEdge);
      addActionTemplateAtEdge(actionSheetEdge, node_data);
      return;
    }

    let planned_action_id;

    console.log("Node Data", node_data);
    //set action_id
    if (node_data) {
      planned_action_id = node_data.action_id;
    }

    if (!position) {
      position = { x: 300, y: 300 };
    }

    const conflictFreeId = findConflictFreeId(nodes, planned_action_id);
    console.log("conflictFreeId", conflictFreeId);
    console.log("special data", node_data);
    const newNode: Node = {
      id: conflictFreeId,
      type: "anything",
      position,
      data: { ...node_data, action_id: conflictFreeId },
    };

    let udpatedNodes = [...nodes, newNode];

    saveFlowVersionImmediate(udpatedNodes, edges);

    setNodes(() => udpatedNodes);
  };

  const updateWorkflow = async (args: UpdateWorklowArgs) => {
    try {
      if (!dbFlowId || !selectedAccount) return;

      console.log("Updating Workflow", args);

      //Save to cloud
      await api.flows.updateFlow(selectedAccount.account_id, dbFlowId, args);

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
      if (!dbFlowId || !dbFlowVersionId || !selectedAccount) {
        console.error(
          "No Flow Id or Flow Version Id or Account to publish workflow version",
        );
        return;
      }

      //Save to cloud
      await api.flows.publishFlowVersion(
        selectedAccount?.account_id,
        dbFlowId,
        dbFlowVersionId,
      );

      //Update state here
      setDbFlowVersion((prevFlow: any) => {
        return {
          ...prevFlow,
          published: true,
          published_at: new Date().toISOString(),
        };
      });

      setDbFlow((prevFlow: any) => {
        return {
          ...prevFlow,
          active: true,
        };
      });

      refresh();
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

    const planned_action_id = action_template.action_id;
    const conflictFreeId = findConflictFreeId(newNodes, planned_action_id);

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
      data: { ...action_template, action_id: conflictFreeId },
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

  const set_panel_tab = (tab: string) => {
    //Used to make nice navigation in side panel
    if (tab === PanelTab.SETTINGS) {
      //Clear selected node
      setSelectedNodeId("");

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
        setSelectedNodeId(selectedNodeObj.id);
        setPanelTab(PanelTab.CONFIG);
      } else {
        setSelectedNodeId("");
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
  //TODO: INVESTIGATE THIS - I think we should have ACTIONS managed more seperate then NODES.
  //This is likely where we are causing weird issues with state.
  const updateNodeData = async (
    update_key: string[],
    data: any[],
  ): Promise<boolean> => {
    try {
      console.log("[UPDATE NODE DATA SYNC]", update_key, data);

      // setNodes((prevNodes) => {
      const newNodes = cloneDeep(nodes);

      let updatedNodes = newNodes.map((node) => {
        if (node.id === selectedNodeId) {
          update_key.forEach((key, index) => {
            console.log(
              `[UPDATING NODE DATA] Updating Node Data in updateNodeData for ${node.id}:${key} with:`,
              data[index],
            );
            node.data[key] = data[index];
          });
          console.log("NEW_DATA_FOR_NODE", node.data);
          setSelectedNodeId(node.id);
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
    return {
      actions: nodes.map((node: any) => {
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
      console.log("error in saveFlowVersionDebounced", error);
    }
  };

  const saveFlowVersionImmediate = (nodes: any, edges: any) => {
    const workflow = makeUpdateFlow(nodes, edges);
    setSavingStatus(SavingStatus.SAVING);
    _saveFlowVersion(workflow);
  };

  const _saveFlowVersion = async (workflow: Workflow) => {
    try {
      if (!dbFlowId || !dbFlowVersionId || !selectedAccount) {
        console.log(
          "No Flow Id or Flow Version Id or Account to save flow version",
        );
        return;
      }

      const res: any = await api.flows.updateFlowVersion(
        selectedAccount.account_id,
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

        router.replace(
          `/workflows/${dbFlowId}/${returned_flow.flow_version_id}/editor`,
        );
      } else {
        // console.log("Flow Version Ids match.");
        console.log("Updated existing draft flow version");
      }

      setSavingStatus(SavingStatus.SAVED);
      setTimeout(() => setSavingStatus(SavingStatus.NONE), 1500); // Clear the status after 2 seconds

      //Maybe difficult!
    } catch (error) {
      console.log("error in _saveFlowVersion", error);
    }
  };

  const _debouncedSaveFlowVersion = useCallback(
    debounce(async (workflow: Workflow) => {
      try {
        await _saveFlowVersion(workflow);
      } catch (error) {
        console.log("error in _debouncedSaveFlowVersion", error);
      }
    }, 1000),
    [dbFlowId, dbFlowVersionId],
  );

  const hydrateFlow = async () => {
    try {
      console.log("Fetch Flow By Id in new hydrate flow: ", workflowId);
      if (!workflowId || !selectedAccount || !workflowVersionId) return;

      const [workflow_response, version] = await Promise.all([
        api.flows.getFlow(selectedAccount.account_id, workflowId),
        api.flows.getFlowVersionById(
          selectedAccount.account_id,
          workflowId,
          workflowVersionId,
        ),
      ]);

      if (!workflow_response || !version || !version.flow_definition) return;

      setDbFlow(workflow_response[0]);
      setDbFlowVersion(version);
      setFlowVersionDefinition(version.flow_definition);

      if (
        version.flow_definition.actions &&
        version.flow_definition.actions.length !== 0
      ) {
        let _nodes: Node[] = version.flow_definition.actions.map(
          (action: any) => {
            let position = action.presentation?.position || { x: 0, y: 0 };

            return {
              position,
              data: action,
              id: action.action_id,
              type: "anything",
            };
          },
        );
        console.log("Nodes in hydrate flow", _nodes);
        setNodes(_nodes);

        if (
          version.flow_definition.edges &&
          version.flow_definition.edges.length !== 0
        ) {
          let _edges: Edge[] = version.flow_definition.edges.map(
            (edge: any) => {
              return edge;
            },
          );
          console.log("Edges in hydrate flow", _edges);
          setEdges(_edges);
        } else {
          console.log("No Edges in Flow Definition in hydrateFlow");
        }

        console.log("Hydrated Flow");
      } else {
        console.log("No Actions found in Flow Version in hydrateFlow");
      }
    } catch (e) {
      console.log("error in fetch flow", JSON.stringify(e, null, 3));
    }
  };

  const selectedNodeInfo = useMemo(() => {
    const selectedNode = nodes.find((node) => node.id === selectedNodeId);
    console.log("[useMemo] Selected Node Info", selectedNode);
    return {
      data: selectedNode?.data,
      variables: selectedNode?.data?.variables || {},
      variablesSchema: selectedNode?.data?.variables_schema || {},
      input: selectedNode?.data?.input || {},
      inputSchema: selectedNode?.data?.input_schema || {},
    };
  }, [nodes, selectedNodeId]);

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

  return (
    <WorkflowVersionContext.Provider
      value={{
        db_flow_id: dbFlowId,
        db_flow_version_id: dbFlowVersionId,
        db_flow: dbFlow,
        db_flow_version: dbFlowVersion,
        flow_version_definition,
        selected_action_id: selectedNodeId,
        selected_node_data: selectedNodeInfo.data,
        selected_node_variables: selectedNodeInfo.variables,
        selected_node_variables_schema: selectedNodeInfo.variablesSchema,
        selected_node_input: selectedNodeInfo.input,
        selected_node_input_schema: selectedNodeInfo.inputSchema,
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
        updateNodeData,
        updateWorkflow,
        publishWorkflowVersion,
      }}
    >
      {children}
    </WorkflowVersionContext.Provider>
  );
};
