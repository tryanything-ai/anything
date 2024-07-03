"use client"

import {
    createContext,
    ReactNode,
    useCallback,
    useContext,
    useEffect,
    useRef,
    useState,
} from "react";

import { useParams } from 'next/navigation'

import {
    addEdge,
    applyEdgeChanges,
    applyNodeChanges,
    Connection,
    Edge,
    EdgeChange,
    Node,
    NodeChange,
    NodeSelectionChange,
    OnConnect,
    OnEdgesChange,
    OnNodesChange,
    ReactFlowInstance,
} from "reactflow";

import api from "@/lib/anything-api"
import { Action, AnythingNodeProps, Workflow } from "@/types/workflows";

import { findConflictFreeId } from "@/lib/studio/helpers";
import { useWorkflowsContext } from "./WorkflowsProvider";
import { set } from "date-fns";

export enum PanelTab {
    SETTINGS = "settings",
    CONFIG = "config"
}

export interface WorkflowVersionContextInterface {
    db_flow_version_id: string;
    db_flow_id: string;
    db_flow: any,
    db_flow_version: any,
    flow_version_definition: any;
    selected_node_id?: string;
    selected_node_data?: Action;
    panel_tab: string;
    setPanelTab: (tab: string) => void;
    nodes: Node[];
    edges: Edge[];
    onNodesChange: OnNodesChange;
    onEdgesChange: OnEdgesChange;
    onConnect: OnConnect;
    onDragOver: (event: any) => void;
    onDrop: (event: any, reactFlowWrapper: any) => void;
    addNode: (position: { x: number; y: number }, specialData?: any) => void;
    setReactFlowInstance: (instance: ReactFlowInstance | null) => void;
    // readNodeConfig: (nodeId: string) => Promise<Action | undefined>;
    writeNodeConfig: (nodeId: string, data: any) => Promise<Action | undefined>;
    getFlowDefinitionsFromReactFlowState: () => Workflow;
}

export const WorkflowVersionContext = createContext<WorkflowVersionContextInterface>({
    db_flow_version_id: "",
    db_flow_id: "",
    db_flow: {},
    db_flow_version: {},
    flow_version_definition: {},
    selected_node_id: undefined,
    panel_tab: PanelTab.CONFIG,
    setPanelTab: () => { },
    nodes: [],
    edges: [],
    onNodesChange: () => { },
    onEdgesChange: () => { },
    onConnect: () => { },
    onDragOver: () => { },
    onDrop: () => { },
    addNode: () => { },
    setReactFlowInstance: () => { },
    // readNodeConfig: async () => undefined,
    writeNodeConfig: async () => undefined,
    getFlowDefinitionsFromReactFlowState: () => {
        return {
            // Define the structure of Flow here if it's needed for the initial value
        } as Workflow;
    },
});

export const useWorkflowVersionContext = () => useContext(WorkflowVersionContext);

export const WorkflowVersionProvider = ({ children }: { children: ReactNode }) => {
    const { workflowId, workflowVersionId } = useParams<{ workflowId: string; workflowVersionId: string }>()
    const { getWorkflowById } = useWorkflowsContext();

    //Easy Access State
    const [dbFlow, setDbFlow] = useState<any>({})
    const [dbFlowVersion, setDbFlowVersion] = useState<any>({})
    const [flow_version_definition, setFlowVersionDefinition] = useState<any>({})
    //Easy Access Id's
    const [dbFlowVersionId, setDbFlowVersionId] = useState<string>("")
    const [dbFlowId, setDbFlowId] = useState<string>("")
    const [selectedNodeId, setSelectedNodeId] = useState<string>("")
    const [selectedNodeData, setSelectedNodeData] = useState<Action | undefined>(undefined)
    //Internal for ReactFlow and Flow Definition Management
    const [hydrated, setHydrated] = useState<boolean>(false);
    const [firstLook, setFirstLook] = useState<boolean>(true);
    const [nodes, setNodes] = useState<Node[]>([]);
    const [edges, setEdges] = useState<Edge[]>([]);
    const [flowVersions, setFlowVersions] = useState<Workflow[]>([]);

    //Navigation State
    const [panel_tab, setPanelTab] = useState<string>(PanelTab.CONFIG);

    const [reactFlowInstance, setReactFlowInstance] =
        useState<ReactFlowInstance | null>(null);
    const timerRef = useRef<NodeJS.Timeout | null>(null);

    const addNode = (position: { x: number; y: number }, specialData?: any) => {
        let planned_node_name;

        //set node_name
        if (specialData) {
            planned_node_name = specialData.node_name;
        }

        const conflictFreeId = findConflictFreeId(nodes, planned_node_name);
        console.log("conflictFreeId", conflictFreeId);
        console.log("special data", specialData);
        const newNode: Node = {
            id: conflictFreeId,
            type: "anything",
            position,
            data: { ...specialData, node_name: conflictFreeId },
        };

        setNodes((nodes) => {
            return [...nodes, newNode];
        });
    };

    const set_panel_tab = (tab: string) => {
        //Used to make nice navigation in side panel
        if (tab === PanelTab.SETTINGS) {
            setSelectedNodeData(undefined);
            setSelectedNodeId("");
            //Trick to clear selection inisde ReactFlow
            onNodesChange([{
                id: selectedNodeId,
                type: 'select',
                selected: false
            }]);
        }
        setPanelTab(tab);
    }

    const onNodesChange: OnNodesChange = (nodeChanges: NodeChange[]) => {
        console.log("onNodesChange nodeChanges", nodeChanges);

        //find the node with selected = true
        let selectionChanges: NodeSelectionChange[] =
            nodeChanges.filter((nodeChange) => nodeChange.type === "select") as NodeSelectionChange[];
        // get the id of the node with selected = true
        if (selectionChanges.length > 0) {
            console.log("selectionChanges", selectionChanges);
            let selectedNode = selectionChanges.find((nodeChange: NodeSelectionChange) => nodeChange.selected);

            if (selectedNode) {
                //Set node and node data for easy access
                setSelectedNodeId(selectedNode.id);
                let selectedNodeData = nodes.find((node) => node.id === selectedNode.id)?.data;
                setSelectedNodeData(selectedNodeData);
                setPanelTab(PanelTab.CONFIG);
            } else {
                setSelectedNodeId("");
                setSelectedNodeData(undefined);
            }
        }

        setNodes((nodes) => {
            return applyNodeChanges(nodeChanges, nodes);
        });
    };

    const onEdgesChange: OnEdgesChange = (edgeChanges: EdgeChange[]) => {
        setEdges((edges) => {
            console.log("onEdgesChange edgeChanges", edgeChanges);
            return applyEdgeChanges(edgeChanges, edges);
        });
    };

    const onConnect: OnConnect = (params: Connection) => {
        setEdges((edges) => {
            // console.log("onEdgesChange edgeChanges", edgeChanges);
            console.log("onConnect params", params);
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

            const nodeData: Action = JSON.parse(
                event.dataTransfer.getData("nodeData")
            );

            console.log("Dropped nodeData", nodeData);

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

    const writeNodeConfig = async (
        nodeId: string,
        data: any
    ): Promise<Action | undefined> => {
        try {
            let updatedNodes = nodes.map((node) => {
                // console.log("node in writeNodeConfig", node);
                if (node.id === nodeId) {
                    return { ...node, data };
                } else {
                    return node;
                }
            });
            setNodes(updatedNodes);
            //TODO: actually update state.
            let reactFlowNode = nodes.find((node) => node.id === nodeId);
            return reactFlowNode?.data;
        } catch (error) {
            console.log("error writing node config in FlowProvider", error);
        }
    };

    const getFlowDefinitionsFromReactFlowState = (): Workflow => {

        let actions: any[] = [];

        //Loop through all nodes and reformat them to what actions prefer
        nodes.forEach((node) => {
            let freshNode = {
                ...node.data,
                presentation: {
                    position: node.position,
                },
            };

            if (node.data.trigger) {
                // trigger = freshNode as Trigger;
            } else {
                actions.push(freshNode as Action);
            }
        });

        //create shape needed for backend
        let newFlow: Workflow = {
            actions: actions as Action[],
            edges: edges as Edge[],
        };

        console.log("New Flow Definition", newFlow);

        return newFlow;
    };

    const synchronize = async () => {
        try {
            //TODO: hash to comapre and only run if dif?
            console.log("Synchronising Flow in FlowProivders.tsx");
            let newFlow = getFlowDefinitionsFromReactFlowState();

            console.log("newFlow in synchronize", newFlow);

            //send
            let res = await api.flows.updateFlowVersion(
                dbFlowId,
                dbFlowVersionId,
                newFlow
            );

            console.log("Flow Synchronized");
            // console.log("res in updateFlowVersion", res);
        } catch (error) {
            console.log("error in synchronise", error);
        }
    };

    //Buffer editor write to DB
    useEffect(() => {
        //Ugly but works to prevent write on load
        if (!hydrated) { console.log("Not hydrated, skipping useEffect"); return; }
        if (firstLook) {
            console.log("First Look, skipping useEffect");
            setFirstLook(false);
            return;
        }
        // Clear any existing timers
        if (timerRef.current) {
            clearTimeout(timerRef.current);
        }

        // Set a new timer to write to flow to backend
        timerRef.current = setTimeout(async () => {
            synchronize();
        }, 100);

        // Clean up
        return () => {
            if (timerRef.current) {
                clearTimeout(timerRef.current);
            }
        };
    }, [nodes, edges]);

    const hydrateFlow = async () => {
        try {
            console.log("Fetch Flow By Id in new hydrate flow: ", workflowId);
            if (!workflowId) return;
            let workflow_response = await getWorkflowById(workflowId);

            if (!workflow_response) return;
            let flow = workflow_response[0];
            let flow_version = flow.flow_versions[0];
            console.log("New Hydreate in Workflow Provider", flow);
            console.log('Version in New Hydrate Flow', flow_version);
            console.log('Flow Definition in New Hydrate Flow', flow_version.flow_definition);

            setDbFlow(flow);
            // let flow_version = flow.flow_versions[0];
            setDbFlowVersion(flow_version);
            setFlowVersionDefinition(flow_version.flow_definition);
            if (flow_version && flow_version.flow_definition) {

                if (flow_version.flow_definition.actions && flow_version.flow_definition.actions.length !== 0) {
                    let _nodes: Node[] = flow_version.flow_definition.actions.map((action) => {

                        let position = action.presentation?.position || { x: 0, y: 0 };

                        return {
                            position,
                            data: action,
                            id: action.node_id,
                            type: "anything",
                        };
                    });
                    console.log("Nodes in hydrate flow", _nodes);
                    setNodes(_nodes);
                } else {
                    console.log("SKIPPING: No Actions in Flow Definition in hydrateFlow");
                }

                if (flow_version.flow_definition.edges && flow_version.flow_definition.edges.length !== 0) {
                    let _edges: Edge[] = flow_version.flow_definition.edges.map((edge) => {
                        return edge;
                    });
                    console.log("Edges in hydrate flow", _edges);
                    setEdges(_edges);
                } else {
                    console.log("SKIPPING: No Edges in Flow Definition in hydrateFlow");
                }

                console.log("Hydrated Flow - setHydrated(true)");
                setHydrated(true);
            } else {
                console.log("No Flow Definition in Flow Version in hydrateFlow");
            }

        } catch (e) {
            console.log("error in fetch flow", JSON.stringify(e, null, 3));
        }
    }

    //Hydrate on Navigation
    useEffect(() => {
        //TODO: why does this always write when we start the flow? How can we prevent this?
        if (!workflowId || !workflowVersionId) return;
        setDbFlowVersionId(workflowVersionId);
        setDbFlowId(workflowId);
        hydrateFlow();
        // hydrateFlow(); //TODO: reimplement loading from JSON. 
    }, [workflowId, workflowVersionId]);

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
                nodes,
                edges,
                panel_tab,
                setPanelTab: set_panel_tab,
                onConnect,
                onNodesChange,
                onEdgesChange,
                onDragOver,
                onDrop,
                addNode,
                setReactFlowInstance,
                writeNodeConfig,
                getFlowDefinitionsFromReactFlowState,
            }}
        >
            {children}
        </WorkflowVersionContext.Provider>
    );
};
