"use client"

import {
    createContext,
    ReactNode,
    useContext,
    useEffect,
    useState,
} from "react";

import api from "@/lib/anything-api";
import { DB_WORKFLOWS_QUERY } from "@/types/supabase-anything";

export type UpdateWorklowArgs = {
    flow_name: string;
    active: boolean;
    version?: string;
};

export interface WorkflowsContextInterface {
    flows: DB_WORKFLOWS_QUERY,
    createWorkflow: () => Promise<void>;
    getWorkflows: () => Promise<void>;
    getWorkflowById: (flowId: string) => Promise<DB_WORKFLOWS_QUERY | undefined>;
    deleteWorkflow: (flowId: string) => Promise<void>;
    updateWorkflow: (flowId: string, args: UpdateWorklowArgs) => Promise<void>;
}

export const WorkflowsContext = createContext<WorkflowsContextInterface>({
    flows: [],
    createWorkflow: async () => { },
    getWorkflows: async () => { },
    getWorkflowById: async () => undefined,
    deleteWorkflow: async () => { },
    updateWorkflow: async () => { }
});

export const useWorkflowsContext = () => useContext(WorkflowsContext);

export const WorkflowsProvider = ({ children }: { children: ReactNode }) => {
    const [flows, setFlows] = useState<DB_WORKFLOWS_QUERY>([]);

    const createWorkflow = async (): Promise<void> => {
        try {
            //TODO Move to DB to fix collision problem
            let flowName = "Flow" + " " + (flows.length + 1);
            console.log("Creating new Flow in FlowsProvider");
            await api.flows.createFlow(flowName);
        } catch (error) {
            console.log("error creating new flow in FlowsProvider", error);
            console.error(error);
        } finally {
            await getWorkflows();
        }
    };

    const deleteWorkflow = async (flowId: string): Promise<void> => {
        try {
            await api.flows.deleteFlow(flowId);
        } catch (error) {
            console.error(error);
        } finally {
            await getWorkflows();
        }
    };

    const updateWorkflow = async (flowId: string, args: UpdateWorklowArgs): Promise<void> => {
        try {
            await api.flows.updateFlow(flowId, args);
        } catch (error) {
            console.error(error);
        } finally {
            await getWorkflows();
        }
    };

    const getWorkflows = async (): Promise<void> => {
        console.log("Getting Flows from API");
        try {
            let res: DB_WORKFLOWS_QUERY = await api.flows.getFlows();
            console.log("getFlows:", res);
            if (res.length > 0) {
                setFlows(res);
            }

        } catch (error) {
            console.error("Error getting flows", error);
        }
    };

    const getWorkflowById = async (flowId: string): Promise<DB_WORKFLOWS_QUERY | undefined> => {
        console.log("Getting Flow by ID from State");
        let res = flows.find((flow) => flow.flow_id === flowId);
        console.log("getFlowById:", res);
        return res;
    };

    // Hydrate flows on launch
    useEffect(() => {
        getWorkflows();
    }, []);

    return (
        <WorkflowsContext.Provider
            value={{
                flows,
                createWorkflow,
                getWorkflows,
                getWorkflowById,
                deleteWorkflow,
                updateWorkflow,
            }}
        >
            {children}
        </WorkflowsContext.Provider>
    );
};
