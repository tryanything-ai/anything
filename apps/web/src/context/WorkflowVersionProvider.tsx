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
import { cloneDeep, conforms, debounce } from "lodash";

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
import { createClient } from "@/lib/supabase/client";
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
  selected_node_inputs: any;
  selected_node_inputs_schema: any;
  selected_node_plugin_config: any;
  selected_node_plugin_config_schema: any;
  panel_tab: string;
  savingStatus: string;
  detailedMode: boolean;
  setDetailedMode: (mode: boolean) => void;
  setPanelTab: (tab: string) => void;
  showingActionSheet: boolean;
  setShowingActionSheet: (showing: boolean) => void;
  showExplorer: boolean;
  setShowExplorer: (showing: boolean) => void;
  explorerTab: string;
  setExplorerTab: (tab: string) => void;
  showActionSheetForEdge: (id: string) => void;
  showActionSheetToChangeTrigger: () => void;
  changeTrigger: (trigger: any) => void;
  showActionSheet: () => void;
  actionSheetMode: string;
  setActionSheetMode: (mode: string) => void;
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
  getActionIcon: (action_id: string) => string;
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
    selected_node_inputs: null,
    selected_node_inputs_schema: null,
    selected_node_plugin_config: null,
    selected_node_plugin_config_schema: null,
    panel_tab: PanelTab.CONFIG,
    savingStatus: SavingStatus.NONE,
    setPanelTab: () => {},
    showingActionSheet: false,
    showExplorer: false,
    setShowExplorer: () => {},
    explorerTab: "results",
    setExplorerTab: () => {},
    detailedMode: true,
    setDetailedMode: () => {},
    showActionSheetForEdge: () => {},
    showActionSheetToChangeTrigger: () => {},
    setShowingActionSheet: () => {},
    showActionSheet: () => {},
    setActionSheetMode: () => {},
    changeTrigger: () => {},
    actionSheetMode: "actions",
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
    getActionIcon: () => "",
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
  const [detailedMode, setDetailedMode] = useState<boolean>(true);
  //React Flow State
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);

  //Navigation State
  const [panel_tab, setPanelTab] = useState<string>(PanelTab.CONFIG);
  //Saving State
  const [savingStatus, setSavingStatus] = useState<string>(SavingStatus.NONE);
  //Action sheet for adding nodes
  const [showingActionSheet, setShowingActionSheet] = useState<boolean>(false);
  const [actionSheetMode, setActionSheetMode] = useState<string>("actions");
  const [actionSheetEdge, setActionSheetEdge] = useState<string>("");

  const [showExplorer, setShowExplorer] = useState<boolean>(false);
  const [explorerTab, setExplorerTab] = useState<string>("results");

  const showActionSheetForEdge = (id: string) => {
    console.log("Show Action Sheet for Edge: ", id);
    setActionSheetMode("actions");
    setShowingActionSheet(true);
    setActionSheetEdge(id);
  };

  const showActionSheetToChangeTrigger = () => {
    console.log("Show Action Sheet to Change Trigger");
    setActionSheetMode("triggers");
    setShowingActionSheet(true);
    setActionSheetEdge("");
  };

  const showActionSheet = () => {
    console.log("Show Action Sheet");
    setActionSheetMode("actions");
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

  const changeTrigger = (new_trigger: any) => {
    console.log("Changing Trigger", new_trigger);

    //Find the trigger node
    let triggerNode = nodes.find((node) => node.data.type === "trigger");
    if (!triggerNode) {
      console.error("No Trigger Node Found");
      return;
    }

    console.log("[CHANGE TRIGGER] Old Trigger Node: ", triggerNode);

    //New triggger node old position
    let updatedTriggerNode: Node = {
      id: new_trigger.action_id,
      type: "anything",
      position: triggerNode.position,
      data: { ...new_trigger, },
    };

    console.log("[CHANGE TRIGGER] New Trigger Node", updatedTriggerNode);

    //Update the nodes array
    let updatedNodes = nodes.map((node) => {
      if (triggerNode && node.id === triggerNode.id) {
        //Swap in new node
        return updatedTriggerNode;
      }
      return node;
    });

    // Update edges with new trigger node id if needed
    let updatedEdges = edges.map((edge) => {
      if (triggerNode && edge.source === triggerNode.id) {
        let new_edge = {
          ...edge,
          id: `${updatedTriggerNode.id}->${edge.target}`,
          source: updatedTriggerNode.id,
        };
        console.log("[CHANGE TRIGGER] New Edge", new_edge);
        return new_edge;
      }
      return edge;
    });

    saveFlowVersionImmediate(updatedNodes, updatedEdges);

    setNodes(updatedNodes);
    setEdges(updatedEdges);
  };

  const getActionIcon = (action_id: string) => {
    const node = nodes.find((node) => node.id === action_id);
    return node?.data?.icon || "";
  };

  const updateWorkflow = async (args: UpdateWorklowArgs) => {
    try {
      if (!dbFlowId || !selectedAccount) return;

      console.log("Updating Workflow", args);

      //TODO: show same saving status in header as other places
      //Save to cloud
      await api.flows.updateFlow(
        await createClient(),
        selectedAccount.account_id,
        dbFlowId,
        args,
      );

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
        await createClient(),
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
      // console.log("selectionChanges", selectionChanges);
      let selectedNode: any = selectionChanges.find(
        (nodeChange: NodeSelectionChange) => nodeChange.selected,
      );

      if (selectedNode) {
        //Set node and node data for easy access
        let selectedNodeObj: any = nodes.find(
          (node) => node.id === selectedNode.id,
        );
        // console.log("selectedNodeObj", selectedNodeObj);

        setSelectedNodeId(selectedNodeObj.id);
        setPanelTab(PanelTab.CONFIG);

        //if the user selects a trigger hide the panel
        if (selectedNodeObj?.data?.type === "trigger") {
          setShowExplorer(false);
        }
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
  const parseJsonRecursively = (value: any): any => {
    console.log(
      "[UPDATE NODE DATA] Starting parseJsonRecursively with value:",
      value,
    );

    // If it's a string, try to parse it as JSON only if it starts with { or [
    if (typeof value === "string") {
      console.log("[UPDATE NODE DATA] Processing string value:", value);
      const trimmed = value.trim();
      if (trimmed.startsWith("{") || trimmed.startsWith("[")) {
        try {
          console.log("[UPDATE NODE DATA] Attempting to parse JSON string");
          const parsed = parseJsonRecursively(JSON.parse(value));
          console.log("[UPDATE NODE DATA] Successfully parsed JSON:", parsed);
          return parsed;
        } catch (e) {
          console.log(
            "[UPDATE NODE DATA] Failed to parse JSON, returning original string",
          );
          return value;
        }
      } else {
        console.log("[UPDATE NODE DATA] Returning non-JSON string value");
        return value;
      }
    }

    // If it's an array, parse each element
    if (Array.isArray(value)) {
      console.log("[UPDATE NODE DATA] Processing array:", value);
      const result = value.map((item) => parseJsonRecursively(item));
      console.log("[UPDATE NODE DATA] Processed array result:", result);
      return result;
    }

    // If it's an object, parse each value
    if (value && typeof value === "object") {
      console.log("[UPDATE NODE DATA] Processing object:", value);
      const parsed: { [key: string]: any } = {};
      for (const key in value) {
        console.log("[UPDATE NODE DATA] Processing object key:", key);
        parsed[key] = parseJsonRecursively(value[key]);
      }
      console.log("[UPDATE NODE DATA] Processed object result:", parsed);
      return parsed;
    }

    // For all other types (number, boolean, null, undefined)
    console.log("[UPDATE NODE DATA] Returning primitive value:", value);
    return value;
  };

  const updateNodeData = async (
    update_key: string[],
    data: any[],
  ): Promise<boolean> => {
    try {
      const newNodes = cloneDeep(nodes);
      let updatedNodes = newNodes.map((node) => {
        if (node.id === selectedNodeId) {
          update_key.forEach((key, index) => {
            console.log(
              `[UPDATE NODE DATA] Before parsing ${key}:`,
              data[index],
            );
            // Parse any stringified JSON recursively
            const parsedValue = parseJsonRecursively(data[index]);

            console.log(
              `[UPDATE NODE DATA] After parsing ${key}:`,
              parsedValue,
            );
            node.data[key] = parsedValue;
          });
          console.log("[UPDATE NODE DATA] NEW_DATA_FOR_NODE", node.data);
          // setSelectedNodeId(node.id);
        }
        return node;
      });

      console.log("[UPDATE NODE DATA] ALL_NEW_NODE_DATA", updatedNodes);

      // Call saveFlowVersion with the latest state
      saveFlowVersionImmediate(updatedNodes, edges);

      // Update the state with the latest state
      setNodes(updatedNodes);

      return true;
    } catch (error) {
      console.log(
        "[UPDATE NODE DATA] error writing node config in WorkflowVersionProvider",
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

  const saveFlowVersionImmediate = (_nodes: any, _edges: any) => {
    const _workflow = makeUpdateFlow(_nodes, _edges);
    setSavingStatus(SavingStatus.SAVING);
    _saveFlowVersion(_workflow);
  };

  const _saveFlowVersion = async (_workflow: Workflow) => {
    try {
      if (!dbFlowId || !dbFlowVersionId || !selectedAccount) {
        console.log(
          "No Flow Id or Flow Version Id or Account to save flow version",
        );
        return;
      }

      const res: any = await api.flows.updateFlowVersion(
        await createClient(),
        selectedAccount.account_id,
        dbFlowId,
        dbFlowVersionId,
        _workflow,
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
        api.flows.getFlow(
          await createClient(),
          selectedAccount.account_id,
          workflowId,
        ),
        api.flows.getFlowVersionById(
          await createClient(),
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
      inputs: selectedNode?.data?.inputs || {},
      inputsSchema: selectedNode?.data?.inputs_schema || {},
      pluginConfig: selectedNode?.data?.plugin_config || {},
      pluginConfigSchema: selectedNode?.data?.plugin_config_schema || {},
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
        getActionIcon,
        db_flow_id: dbFlowId,
        db_flow_version_id: dbFlowVersionId,
        db_flow: dbFlow,
        db_flow_version: dbFlowVersion,
        flow_version_definition,
        selected_action_id: selectedNodeId,
        selected_node_data: selectedNodeInfo.data,
        selected_node_inputs: selectedNodeInfo.inputs,
        selected_node_inputs_schema: selectedNodeInfo.inputsSchema,
        selected_node_plugin_config: selectedNodeInfo.pluginConfig,
        selected_node_plugin_config_schema: selectedNodeInfo.pluginConfigSchema,
        nodes,
        edges,
        savingStatus,
        panel_tab,
        showingActionSheet,
        showExplorer,
        setShowExplorer,
        explorerTab,
        setExplorerTab,
        detailedMode,
        setDetailedMode,
        setShowingActionSheet,
        showActionSheetToChangeTrigger,
        changeTrigger,
        actionSheetMode,
        setActionSheetMode,
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
