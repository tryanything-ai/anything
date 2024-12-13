"use client";
import { useContext } from "react";
import { createContext, ReactNode, useEffect, useState } from "react";
import { useAccounts } from "./AccountsContext";
import api from "@repo/anything-api";
import { useRouter, useParams } from "next/navigation";
import { createClient } from "@/lib/supabase/client";
export interface WorkflowVersionControlContextInterface {
  versions: any[];
  refresh: () => void;
}

export const useWorkflowVersionControl = () =>
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
  const { selectedAccount } = useAccounts();
  const params = useParams<{ workflowVersionId: string; workflowId: string }>();
  const [versions, setVersions] = useState<any[]>([]);

  const fetchVersions = async () => {
    try {
      if (!selectedAccount) return;
      const versions = await api.flows.getFlowVersionsForWorkflowId(
        await createClient(),
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
