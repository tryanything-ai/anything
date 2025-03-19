"use client";

import { createContext, ReactNode, useContext, useState } from "react";
import { useWorkflowVersion } from "./WorkflowVersionProvider";
import api from "@repo/anything-api";
import {
  StartWorkflowTestResult,
  TaskRow,
  WorklfowTestSessionResult,
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

  const resetState = () => {
    setTestingMode(TestingMode.ACTION);
    setWorkflowTestingSessionId("");
    setWorkflowTestingSessionTasks([]);
    setTestingAction(false);
    setTestingWorkflow(false);
    setTestStartedTime("");
    setTestFinishedTime("");
  };

  const pollForResults = async (
    flowId: string,
    versionId: string,
    workflow_session_id: string,
  ) => {
    let isComplete = false;
    let attempts = 0;
    const MAX_ATTEMPTS = 60; // Stop after 60 seconds
    let finalTasksSet = false; // Add flag to track if we've set final tasks

    // Add initial delay to allow backend to process
    await new Promise((resolve) => setTimeout(resolve, 500));

    while (!isComplete && attempts < MAX_ATTEMPTS) {
      try {
        if (!flowId || !versionId || !workflow_session_id || !selectedAccount) {
          console.error("Missing required polling parameters");
          return;
        }

        // Don't make additional requests if we've already marked as complete
        if (finalTasksSet) {
          console.log("Polling stopped - final tasks already set");
          return;
        }

        const result = await api.testing.getTestingResults(
          await createClient(),
          selectedAccount.account_id,
          flowId,
          versionId,
          workflow_session_id,
        );

        if (!result) {
          console.error("No result returned from polling");
          attempts++;
          await new Promise((resolve) => setTimeout(resolve, 1000));
          continue;
        }

        if (result.complete) {
          isComplete = true;
          // Set final tasks state only once
          if (!finalTasksSet && result.tasks) {
            console.log("Setting final tasks state");
            setWorkflowTestingSessionTasks(result.tasks);
            finalTasksSet = true;
          }
          setTestingWorkflow(false);
          setTestFinishedTime(new Date().toISOString());
          return;
        }

        // Only update tasks if we haven't reached completion
        if (!finalTasksSet && result.tasks) {
          console.log("Updating in-progress tasks");
          setWorkflowTestingSessionTasks(result.tasks);
        }

        attempts++;
        await new Promise((resolve) => setTimeout(resolve, 1000));
      } catch (error) {
        console.error("Error polling for results:", error);
        attempts++;
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }
    }

    if (!isComplete) {
      console.error("Polling timed out after 60 seconds");
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
      // Start polling for results
      pollForResults(db_flow_id, db_flow_version_id,  flow_session_id);
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
