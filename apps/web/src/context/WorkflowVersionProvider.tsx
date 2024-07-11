"use client"

import {
    createContext,
    ReactNode,
    useCallback,
    useContext,
    useEffect,
    useState
} from "react";

import { useParams } from 'next/navigation'
import { cloneDeep, debounce } from 'lodash';

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

import api from "@/lib/anything-api"
import { Action, AnythingNodeProps, Workflow } from "@/types/workflows";

import { findConflictFreeId } from "@/lib/studio/helpers";
import { useWorkflowsContext } from "./WorkflowsProvider";

export enum PanelTab {
    SETTINGS = "settings",
    CONFIG = "config"
}

export enum SavingStatus {
    SAVING = "saving...",
    SAVED = "saved!",
    NONE = ""
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
    savingStatus: string;
    setPanelTab: (tab: string) => void;
    showingActionSheet: boolean;
    setShowingActionSheet: (showing: boolean) => void;
    showActionSheetForEdge: (id: string) => void;
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
    updateNodeData: (update_key: string[], data: any[]) => Promise<boolean>;

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
    savingStatus: SavingStatus.NONE,
    setPanelTab: () => { },
    showingActionSheet: false,
    showActionSheetForEdge: () => { },
    setShowingActionSheet: () => { },
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
    updateNodeData: async () => false,
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
    // const [selectedNodeData, setSelectedNodeData] = useState<Action | undefined>(undefined)
    //Internal for ReactFlow and Flow Definition Management
    // const [hydrated, setHydrated] = useState<boolean>(false);
    // const [firstLook, setFirstLook] = useState<boolean>(true);
    const [nodes, setNodes] = useState<Node[]>([]);
    const [edges, setEdges] = useState<Edge[]>([]);
    // const [flowVersions, setFlowVersions] = useState<Workflow[]>([]);

    //Navigation State
    const [panel_tab, setPanelTab] = useState<string>(PanelTab.CONFIG);
    //Saving State
    const [savingStatus, setSavingStatus] = useState<string>(SavingStatus.NONE);
    //Action sheet for adding nodes
    const [showingActionSheet, setShowingActionSheet] = useState<boolean>(false);
    const [actionSheetEdge, setActionSheetEdge] = useState<string>("");

    const [reactFlowInstance, setReactFlowInstance] =
        useState<ReactFlowInstance | null>(null);
    // const timerRef = useRef<NodeJS.Timeout | null>(null);

    const showActionSheetForEdge = (id: string) => {
        console.log("Show Action Sheet for Edge: ", id);
        setShowingActionSheet(true);
        setActionSheetEdge(id);
    }

    const resetState = () => {
        console.log("Resetting State in WorkflowVersionProvider");
        setNodes([]);
        setEdges([]);
        setDbFlowVersionId("");
        setDbFlowId("");
        setDbFlow({});
        setDbFlowVersion({});
        setFlowVersionDefinition({});
        // setHydrated(false);
        setSelectedNodeId("");
        // setSelectedNodeData(undefined);
        // setFirstLook(false);
    }

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

    const addActionTemplateAtEdge = (id: string, action_template: Action) => {

        let planned_node_name = action_template.label;

        const conflictFreeId = findConflictFreeId(nodes, planned_node_name);
        console.log("conflictFreeId", conflictFreeId);

        //TODO: update all the effected edges
        //Update all the effected positions for the nodes
        // console.log("special data", specialData);
        // const newNode: Node = {
        //     id: conflictFreeId,
        //     type: "anything",
        //     position,
        //     data: { ...specialData, node_name: conflictFreeId },
        // };

        // setNodes((nodes) => {
        //     return [...nodes, newNode];
        // });
    }

    const set_panel_tab = (tab: string) => {
        //Used to make nice navigation in side panel
        if (tab === PanelTab.SETTINGS) {
            // setSelectedNodeData(undefined);
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

        // let nonDimmensionChanges = nodeChanges.filter((nodeChange) => nodeChange.type !== "dimensions") as NodeSelectionChange[];
        // get the id of the node with selected = true
        if (selectionChanges.length > 0) {
            console.log("selectionChanges", selectionChanges);
            let selectedNode = selectionChanges.find((nodeChange: NodeSelectionChange) => nodeChange.selected);

            if (selectedNode) {
                //Set node and node data for easy access
                setSelectedNodeId(selectedNode.id);
                let selectedNodeData = nodes.find((node) => node.id === selectedNode.id)?.data;
                // setSelectedNodeData(selectedNodeData);
                setPanelTab(PanelTab.CONFIG);
            } else {
                setSelectedNodeId("");
                // setSelectedNodeData(undefined);
            }
        }

        let new_nodes = applyNodeChanges(nodeChanges, nodes);
        console.log("new_nodes", new_nodes);

        let unPersistedChanges: NodeSelectionChange[] =
            nodeChanges.filter((nodeChange) => nodeChange.type === "dimensions" || nodeChange.type === "select") as NodeSelectionChange[];

        if (unPersistedChanges.length === 0) {
            console.log("Saving Node Update because not dimmension or select changes")
            saveFlowVersion(new_nodes, edges);
        } else {
            console.log("Skipping Save because dimmension or select changes")
        }

        setNodes((nodes) => {
            return new_nodes;
        });
    };

    const onEdgesChange: OnEdgesChange = (edgeChanges: EdgeChange[]) => {
        let new_edges = applyEdgeChanges(edgeChanges, edges);

        //find the node with selected = true
        let selectionChanges: EdgeSelectionChange[] =
            edgeChanges.filter((edgeChange) => edgeChange.type === "select") as EdgeSelectionChange[];

        if (selectionChanges.length === 0) {
            console.log("Saving Edge Update because not select changes")
            saveFlowVersion(nodes, new_edges);
        } else {
            console.log("Skipping Save because select changes")
        }

        setEdges((edges) => {
            console.log("onEdgesChange edgeChanges", edgeChanges);
            return new_edges;
        });
    };

    const onConnect: OnConnect = (params: any) => {

        params.type = "anything";
        let new_edges = addEdge(params, edges);

        saveFlowVersion(nodes, new_edges);

        setEdges(() => { return new_edges });
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

    const updateNodeData = async (
        update_key: string[],
        data: any[]
    ): Promise<boolean> => {
        try {
            console.log("writeNodeConfig");
    
            let updatedNodes;
            setNodes((prevNodes) => {
                const newNodes = cloneDeep(prevNodes);
    
                updatedNodes = newNodes.map((node) => {
                    if (node.id === selectedNodeId) {
                        update_key.forEach((key, index) => {
                            node.data[key] = data[index];
                        });
                    }
                    return node;
                });
    
                return updatedNodes;
            });
    
            // Wait for the state update to complete
            await new Promise((resolve) => setTimeout(resolve, 0));
    
            // Call saveFlowVersion with the latest state
            await saveFlowVersion(updatedNodes, edges);
    
            return true;
        } catch (error) {
            console.log("error writing node config in WorkflowVersionProvider", error);
            return false;
        }
    };
    // const updateNodeData = async (
    //     update_key: string[],
    //     data: any[]
    // ): Promise<boolean> => {
    //     try {
    //         console.log("writeNodeConfig");

    //         setNodes((prevNodes) => {


    //         }); 

    //         let new_nodes = cloneDeep(nodes);

    //         let updatedNodes = new_nodes.map((node) => {
    //             if (node.id === selectedNodeId) {

    //                 // Clone the node and its data
    //                 // let new_node = { ...node, data: { ...node.data } };

    //                 update_key.forEach((key, index) => {
    //                     node.data[key] = data[index];
    //                 });

    //                 // console.log("New Node Data in updateNodeData", node.data);
    //                 return node;
    //             } else {
    //                 return node;
    //             }
    //         });

    //         saveFlowVersion(updatedNodes, edges);
    //         setNodes(updatedNodes);
    //         return true;

    //     } catch (error) {
    //         console.log("error writing node config in WorkflowVersionProvider", error);
    //         return false;
    //     }
    // };

    const getFlowDefinitionsFromReactFlowState = (): Workflow => {

        let actions: any[] = [];

        //Loop through all nodes and reformat them to what actions prefer
        nodes.forEach((node) => {
            let freshNode = {
                ...node.data,
                presentation: {
                    position: node.position,
                }
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
    }

    const debouncedSaveFlowVersion = useCallback(
        debounce(async (workflow: Workflow) => {
            try {
                const res = await api.flows.updateFlowVersion(dbFlowId, dbFlowVersionId, workflow);
                console.log('Flow Saved: ', res);
                setSavingStatus(SavingStatus.SAVED);
                setTimeout(() => setSavingStatus(SavingStatus.NONE), 2000); // Clear the status after 2 seconds
                // await hydrateFlow(); //This means we would have to hand manage syncing of 
                //things like selected node and its dependences liek selected_node_data etc etc
                //Maybe difficult!
            } catch (error) {
                console.log('error in saveFlowVersion', error);
            }
        }, 1000),
        [dbFlowId, dbFlowVersionId]
    );


    const saveFlowVersion = (nodes: any, edges: any) => {
        const workflow = makeUpdateFlow(nodes, edges);
        setSavingStatus(SavingStatus.SAVING);
        debouncedSaveFlowVersion(workflow);
    };

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

                console.log("Hydrated Flow");
                // setHydrated(true);
            } else {
                console.log("No Flow Definition in Flow Version in hydrateFlow");
            }

        } catch (e) {
            console.log("error in fetch flow", JSON.stringify(e, null, 3));
        }
    }

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


    // Selector for selected node data
    const selectedNodeData = nodes.find((node) => node.id === selectedNodeId)?.data;

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
                savingStatus,
                panel_tab,
                showingActionSheet,
                setShowingActionSheet,
                showActionSheetForEdge,
                setPanelTab: set_panel_tab,
                onConnect,
                onNodesChange,
                onEdgesChange,
                onDragOver,
                onDrop,
                addNode,
                setReactFlowInstance,
                updateNodeData,
                getFlowDefinitionsFromReactFlowState,
            }}
        >
            {children}
        </WorkflowVersionContext.Provider>
    );
};
