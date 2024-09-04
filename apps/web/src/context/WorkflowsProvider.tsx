"use client";

import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";

import api from "@/lib/anything-api";
// import { DB_WORKFLOWS_QUERY } from "@/types/supabase-anything";

export type UpdateWorklowArgs = {
  flow_name?: string;
  description?: string;
  active?: boolean;
};

export interface WorkflowsContextInterface {
  flows: any;
  createWorkflow: () => Promise<void>;
  getWorkflows: () => Promise<void>;
  getWorkflowById: (flowId: string) => Promise<any | undefined>;
  deleteWorkflow: (flowId: string) => Promise<void>;
  updateWorkflow: (flowId: string, args: UpdateWorklowArgs) => Promise<void>;
}

export const WorkflowsContext = createContext<WorkflowsContextInterface>({
  flows: [],
  createWorkflow: async () => {},
  getWorkflows: async () => {},
  getWorkflowById: async () => undefined,
  deleteWorkflow: async () => {},
  updateWorkflow: async () => {},
});

export const useWorkflowsContext = () => useContext(WorkflowsContext);

export const WorkflowsProvider = ({
  children,
}: {
  children: ReactNode;
}): JSX.Element => {
  const [flows, setFlows] = useState<any>([]);

  const createWorkflow = async (): Promise<void> => {
    try {
      await api.flows.createFlow();
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
    args: UpdateWorklowArgs,
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
      let res: any = await api.flows.getFlows();
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

  const getWorkflowById = async (flowId: string): Promise<any | undefined> => {
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
