"use client";

import { createContext, ReactNode, useContext, useState } from "react";
import { useWorkflowVersion } from "./WorkflowVersionProvider";
import api from "@repo/anything-api";
import {
  StartWorkflowTestResult,
  TaskRow,
  WorklfowTestSessionResult,
} from "@repo/anything-api/testing";
import { useAccounts } from "./AccountsContext";

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

    while (!isComplete) {
      // Mock API call to check workflow status
      if (!flowId || !versionId || !workflow_session_id || !selectedAccount) {
        console.log("No flow or version id to poll for results");
        return;
      }

      const result: WorklfowTestSessionResult =
        await api.testing.getTestingResults(
          selectedAccount.account_id,
          flowId,
          versionId,
          workflow_session_id,
        );

      if (result) {
        console.log("Polling results:", result);
        //Make tasks available
        if (result.tasks) {
          console.log("Setting completed tasks:", result.tasks);
          setWorkflowTestingSessionTasks(result.tasks);
        }

        if (result?.complete) {
          isComplete = true;
          setTestingWorkflow(false);
          setTestFinishedTime(new Date().toISOString());
          // Handle completion (e.g., update state with results)
          console.log("Workflow completed:", result.tasks);
        } else {
          // Wait before polling again
          await new Promise((resolve) => setTimeout(resolve, 1000));
        }
      }
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

      let results: StartWorkflowTestResult = await api.testing.testWorkflow(
        selectedAccount.account_id,
        db_flow_id,
        db_flow_version_id,
      );

      if (!results) {
        console.log("No results returned from testing workflow");
        return;
      } else {
        console.log("Testing workflow results:", results);
        setWorkflowTestingSessionId(results.flow_session_id);
      }
      // Start polling for results
      pollForResults(db_flow_id, db_flow_version_id, results.flow_session_id);
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
