"use client";
import { useContext } from "react";
import { createContext, ReactNode, useEffect, useState } from "react";
import { useAccountsContext } from "./AccountsContext";
import api from "@/lib/anything-api";
import { useRouter, useParams } from "next/navigation";

export interface WorkflowVersionControlContextInterface {
  versions: any[];
  refresh: () => void;
}

export const useWorkflowVersionControlContext = () =>
  useContext(WorkflowVersionControlContext);

export const WorkflowVersionControlContext =
  createContext<WorkflowVersionControlContextInterface>({
    versions: [],
    refresh: () => {},
  });

export const WorkflowVersionControlProvider = ({
  children,
}: {
  children: ReactNode;
}): JSX.Element => {
  const { selectedAccount } = useAccountsContext();
  const params = useParams<{ workflowVersionId: string; workflowId: string }>();
  const [versions, setVersions] = useState<any[]>([]);

  const fetchVersions = async () => {
    try {
      if (!selectedAccount) return;
      const versions = await api.flows.getFlowVersionsForWorkflowId(
        selectedAccount.account_id,
        params.workflowId,
      );
      setVersions(versions);
    } catch (error) {
      console.error(error);
    }
  };

  const refresh = () => {
    fetchVersions();
  };

  useEffect(() => {
    if (params.workflowVersionId && params.workflowId) {
      console.log("fetching version", params.workflowVersionId);
      fetchVersions();
    }
  }, [params.workflowVersionId]);

  return (
    <WorkflowVersionControlContext.Provider value={{ versions, refresh }}>
      {children}
    </WorkflowVersionControlContext.Provider>
  );
};
