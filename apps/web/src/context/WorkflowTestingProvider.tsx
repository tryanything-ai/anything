"use client";

import {
  createContext,
  ReactNode,
  useContext,
  useState,
  useRef,
  useEffect,
} from "react";
import { useWorkflowVersion } from "./WorkflowVersionProvider";
import api from "@repo/anything-api";
import {
  StartWorkflowTestResult,
  TaskRow,
  WorklfowTestSessionResult,
  createWorkflowTestingWebSocket,
  WorkflowTestingUpdate,
} from "@repo/anything-api";
import { useAccounts } from "./AccountsContext";
import { createClient } from "@/lib/supabase/client";
import { v4 as uuidv4 } from "uuid";

export enum TestingMode {
  ACTION = "action",
  WORKFLOW = "workflow",
}

export interface WorkflowTestingContextInterface {
  testingMode: TestingMode;
  testingAction: boolean;
  workflowTestingSessionId?: string;
  worklowTestingSessionTasks: TaskRow[];
  testStartedTime: string;
  testFinishedTime: string;
  testAction: (action_id: string) => Promise<void>;
  testingWorkflow: boolean;
  testWorkflow: () => Promise<void>;
  resetState: () => void;
}

export const WorkflowTestingContext =
  createContext<WorkflowTestingContextInterface>({
    testingMode: TestingMode.ACTION,
    testingAction: false,
    testStartedTime: "",
    testFinishedTime: "",
    workflowTestingSessionId: "",
    worklowTestingSessionTasks: [],
    testAction: async () => {},
    testingWorkflow: false,
    testWorkflow: async () => {},
    resetState: () => {},
  });

export const useWorkflowTesting = () => useContext(WorkflowTestingContext);

export const WorkflowTestingProvider = ({
  children,
}: {
  children: ReactNode;
}): JSX.Element => {
  const { setPanelTab, db_flow_id, db_flow_version_id } = useWorkflowVersion();

  const { selectedAccount } = useAccounts();

  const [testingMode, setTestingMode] = useState<TestingMode>(
    TestingMode.ACTION,
  );
  const [workflowTestingSessionId, setWorkflowTestingSessionId] =
    useState<string>("");
  const [worklowTestingSessionTasks, setWorkflowTestingSessionTasks] = useState<
    TaskRow[]
  >([]);
  const [testingAction, setTestingAction] = useState<boolean>(false);
  const [testingWorkflow, setTestingWorkflow] = useState<boolean>(false);
  const [testStartedTime, setTestStartedTime] = useState<string>("");
  const [testFinishedTime, setTestFinishedTime] = useState<string>("");

  // WebSocket connection ref
  const wsRef = useRef<WebSocket | null>(null);

  // Cleanup WebSocket on unmount
  useEffect(() => {
    return () => {
      if (wsRef.current) {
        wsRef.current.close();
        wsRef.current = null;
      }
    };
  }, []);

  const resetState = () => {
    setTestingMode(TestingMode.ACTION);
    setWorkflowTestingSessionId("");
    setWorkflowTestingSessionTasks([]);
    setTestingAction(false);
    setTestingWorkflow(false);
    setTestStartedTime("");
    setTestFinishedTime("");

    // Close WebSocket connection if it exists
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }
  };

  const refreshTaskData = async () => {
    if (
      !selectedAccount ||
      !db_flow_id ||
      !db_flow_version_id ||
      !workflowTestingSessionId
    ) {
      return;
    }

    try {
      const result = await api.testing.getTestingResults(
        await createClient(),
        selectedAccount.account_id,
        db_flow_id,
        db_flow_version_id,
        workflowTestingSessionId,
      );

      if (result?.tasks) {
        console.log(
          "[WEBSOCKET] Refreshed task data from API:",
          result.tasks.length,
          "tasks",
        );
        setWorkflowTestingSessionTasks(result.tasks);
      }
    } catch (error) {
      console.error("[WEBSOCKET] Error refreshing task data:", error);
    }
  };

  const subscribeToWorkflowUpdates = async (flow_session_id: string) => {
    if (!selectedAccount) {
      console.error("No account selected for WebSocket subscription");
      return;
    }

    try {
      console.log(
        "[WEBSOCKET] Subscribing to workflow updates for session:",
        flow_session_id,
      );

      const ws = await createWorkflowTestingWebSocket(
        await createClient(),
        selectedAccount.account_id,
        flow_session_id,
        (update: WorkflowTestingUpdate) => {
          console.log("[WEBSOCKET] Received workflow update:", update);

          switch (update.type) {
            case "connection_established":
              console.log("[WEBSOCKET] Connection established");
              break;

            case "session_state":
              // Initial session state with current tasks
              if (update.tasks) {
                console.log(
                  "[WEBSOCKET] Setting initial session state with tasks:",
                  update.tasks.length,
                );
                setWorkflowTestingSessionTasks(update.tasks);
              }
              break;

            case "workflow_update":
              switch (update.update_type) {
                case "task_created":
                case "task_updated":
                case "task_completed":
                case "task_failed":
                  // Simple update - refresh task data from API
                  if (
                    update.data?.needs_refresh &&
                    db_flow_id &&
                    db_flow_version_id &&
                    workflowTestingSessionId
                  ) {
                    console.log(
                      "[WEBSOCKET] Task update received, refreshing data from API",
                    );
                    refreshTaskData();
                  }
                  break;

                case "workflow_completed":
                case "workflow_failed":
                  // Workflow finished - do final refresh
                  console.log("[WEBSOCKET] Workflow completed");
                  if (
                    update.data?.needs_refresh &&
                    db_flow_id &&
                    db_flow_version_id &&
                    workflowTestingSessionId
                  ) {
                    refreshTaskData();
                  }
                  setTestingWorkflow(false);
                  setTestFinishedTime(new Date().toISOString());

                  // Close WebSocket connection
                  if (wsRef.current) {
                    wsRef.current.close();
                    wsRef.current = null;
                  }
                  break;
              }
              break;
          }
        },
        (error) => {
          console.error("[WEBSOCKET] WebSocket error:", error);
          setTestingWorkflow(false);
        },
        (event) => {
          console.log(
            "[WEBSOCKET] WebSocket closed:",
            event.code,
            event.reason,
          );
          wsRef.current = null;
        },
      );

      wsRef.current = ws;
    } catch (error) {
      console.error(
        "[WEBSOCKET] Failed to create WebSocket connection:",
        error,
      );
      setTestingWorkflow(false);
    }
  };

  const testWorkflow = async () => {
    try {
      resetState(); // Reset state before testing

      setTestStartedTime(new Date().toISOString());
      if (!db_flow_id || !db_flow_version_id) {
        console.log("No flow or version id to test workflow");
        return;
      }

      setTestingWorkflow(true);
      setTestingMode(TestingMode.WORKFLOW);
      setPanelTab("testing");

      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }

      const trigger_session_id = uuidv4();
      const flow_session_id = uuidv4();

      let results: StartWorkflowTestResult = await api.testing.testWorkflow(
        await createClient(),
        selectedAccount.account_id,
        db_flow_id,
        db_flow_version_id,
        trigger_session_id,
        flow_session_id,
      );

      if (!results) {
        console.log("No results returned from testing workflow");
        return;
      } else {
        console.log("Testing workflow results:", results);
        setWorkflowTestingSessionId(flow_session_id);
      }
      // Start WebSocket subscription for real-time updates
      // subscribeToWorkflowUpdates(flow_session_id);
    } catch (error) {
      console.error(error);
    } finally {
      setTestingWorkflow(false);
    }
  };

  const testAction = async (action_id: string) => {
    try {
      if (!db_flow_id || !db_flow_version_id || !selectedAccount) {
        console.log("No flow or version id to test action");
        return;
      }
      setTestingAction(true);
      setTestingMode(TestingMode.ACTION);
      setPanelTab("testing");
      await api.testing.testAction(
        await createClient(),
        selectedAccount.account_id,
        db_flow_id,
        db_flow_version_id,
        action_id,
      );
    } catch (error) {
      console.error(error);
    } finally {
      setTestingAction(false);
    }
  };

  return (
    <WorkflowTestingContext.Provider
      value={{
        testingMode,
        testingAction,
        testStartedTime,
        testFinishedTime,
        workflowTestingSessionId,
        worklowTestingSessionTasks,
        testingWorkflow,
        testWorkflow,
        testAction,
        resetState,
      }}
    >
      {children}
    </WorkflowTestingContext.Provider>
  );
};
