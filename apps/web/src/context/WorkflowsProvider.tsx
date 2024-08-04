"use client";

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
  flow_name?: string;
  description?: string;
  active?: boolean;
};

export interface WorkflowsContextInterface {
  flows: DB_WORKFLOWS_QUERY;
  createWorkflow: () => Promise<void>;
  getWorkflows: () => Promise<void>;
  getWorkflowById: (flowId: string) => Promise<DB_WORKFLOWS_QUERY | undefined>;
  deleteWorkflow: (flowId: string) => Promise<void>;
  updateWorkflow: (flowId: string, args: UpdateWorklowArgs) => Promise<void>;
}

export const WorkflowsContext = createContext<WorkflowsContextInterface>({
  flows: [],
  createWorkflow: async () => {},
  getWorkflows: async () => {},
  getWorkflowById: async () => undefined,
  deleteWorkflow: async () => {},
  updateWorkflow: async () => {}
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

  const updateWorkflow = async (
    flowId: string,
    args: UpdateWorklowArgs
  ): Promise<void> => {
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
      } else {
        setFlows([]);
      }
    } catch (error) {
      console.error("Error getting flows", error);
    }
  };

  const getWorkflowById = async (
    flowId: string
  ): Promise<DB_WORKFLOWS_QUERY | undefined> => {
    console.log("[WORKFLOWSPROVIDER]: Getting Flow by ID from State");
    return await api.flows.getFlow(flowId);
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
