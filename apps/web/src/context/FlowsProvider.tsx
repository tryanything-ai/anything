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

export type UpdateFlowArgs = {
    flow_name: string;
    active: boolean;
    version?: string;
};

export interface FlowsContextInterface {
    flows: DB_WORKFLOWS_QUERY,
    createNewFlow: () => Promise<void>;
    getFlows: () => Promise<void>;
    getFlowById: (flowId: string) => Promise<DB_WORKFLOWS_QUERY | undefined>;
    deleteFlow: (flowId: string) => Promise<void>;
    updateFlow: (flowId: string, args: UpdateFlowArgs) => Promise<void>;
    stopExecution: () => Promise<void>;
}

export const FlowsContext = createContext<FlowsContextInterface>({
    flows: [],
    createNewFlow: async () => { },
    getFlows: async () => { },
    getFlowById: async () => undefined,
    deleteFlow: async () => { },
    updateFlow: async () => { },
    stopExecution: async () => { },
});

export const useFlowsContext = () => useContext(FlowsContext);

export const FlowsProvider = ({ children }: { children: ReactNode }) => {
    const [flows, setFlows] = useState<DB_WORKFLOWS_QUERY>([]);

    const createNewFlow = async (): Promise<void> => {
        try {
            //TODO Move to DB to fix collision problem
            let flowName = "Flow" + " " + (flows.length + 1);
            console.log("Creating new Flow in FlowsProvider");
            await api.flows.createFlow(flowName);
        } catch (error) {
            console.log("error creating new flow in FlowsProvider", error);
            console.error(error);
        } finally {
            await getFlows();
        }
    };

    const deleteFlow = async (flowId: string): Promise<void> => {
        try {
            await api.flows.deleteFlow(flowId);
        } catch (error) {
            console.error(error);
        } finally {
            await getFlows();
        }
    };

    const updateFlow = async (flowId: string, args: UpdateFlowArgs): Promise<void> => {
        try {
            await api.flows.updateFlow(flowId, args);
        } catch (error) {
            console.error(error);
        } finally {
            await getFlows();
        }
    };

    const getFlows = async (): Promise<void> => {
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

    const getFlowById = async (flowId: string): Promise<DB_WORKFLOWS_QUERY | undefined> => {
        console.log("Getting Flow by ID from State");
        let res = flows.find((flow) => flow.flow_id === flowId);
        console.log("getFlowById:", res);
        return res;
    };

    const stopExecution = async (): Promise<void> => {
        await api.flows.stopExecution();
    };

    // Hydrate flows on launch
    useEffect(() => {
        getFlows();
    }, []);

    return (
        <FlowsContext.Provider
            value={{
                flows,
                createNewFlow,
                getFlows,
                getFlowById,
                deleteFlow,
                updateFlow,
                stopExecution,
            }}
        >
            {children}
        </FlowsContext.Provider>
    );
};
