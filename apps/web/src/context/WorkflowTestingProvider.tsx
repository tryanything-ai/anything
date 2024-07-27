"use client"

import {
    createContext,
    ReactNode,
    useState
} from "react";
import { useWorkflowVersionContext } from "./WorkflowVersionProvider";
import api from "@/lib/anything-api";
import { StartWorkflowTestResult, TaskRow, WorklfowTestSessionResult } from "@/lib/anything-api/testing";

export enum TestingMode {
    ACTION = "action",
    WORKFLOW = "workflow"
}

export interface WorkflowTestingContextInterface {
    testingMode: TestingMode;
    testingAction: boolean;
    workflowTestingSessionId?: string;
    worklowTestingSessionTasks: TaskRow[];
    testAction: (action_id: string) => Promise<void>;
    testingWorkflow: boolean;
    testWorkflow: () => Promise<void>;
}

export const WorkflowTestingContext = createContext<WorkflowTestingContextInterface>({
    testingMode: TestingMode.ACTION,
    testingAction: false,
    workflowTestingSessionId: "",
    worklowTestingSessionTasks: [],
    testAction: async () => { },
    testingWorkflow: false,
    testWorkflow: async () => { }
})

export const WorkflowTestingProvider = ({ children }: { children: ReactNode }) => {

    const { setPanelTab, db_flow_id, db_flow_version_id } = useWorkflowVersionContext();

    const [testingMode, setTestingMode] = useState<TestingMode>(TestingMode.ACTION);
    const [workflowTestingSessionId, setWorkflowTestingSessionId] = useState<string>("");
    const [worklowTestingSessionTasks, setWorkflowTestingSessionTasks] = useState<TaskRow[]>([]);
    const [testingAction, setTestingAction] = useState<boolean>(false);
    const [testingWorkflow, setTestingWorkflow] = useState<boolean>(false);

    const resetState = () => {
        setTestingMode(TestingMode.ACTION);
        setWorkflowTestingSessionId("");
        setWorkflowTestingSessionTasks([]);
        setTestingAction(false);
        setTestingWorkflow(false);
    }

    const pollForResults = async (flowId: string, versionId: string, workflow_session_id: string) => {
        let isComplete = false;

        while (!isComplete) {
            // Mock API call to check workflow status
            const result: WorklfowTestSessionResult = await api.testing.getTestingResults(flowId, versionId, workflow_session_id);

            if(result){

                console.log("Polling results:", result);
                //Make tasks available
                if(result.tasks){
                    console.log("Setting completed tasks:", result.tasks);
                    setWorkflowTestingSessionTasks(result.tasks); 
                } 

                if (result?.complete) {  

                    isComplete = true;
                    setTestingWorkflow(false);
                    // Handle completion (e.g., update state with results)
                    console.log("Workflow completed:", result.tasks);
                } else {
                    // Wait before polling again
                    await new Promise(resolve => setTimeout(resolve, 1000));
                }
            }
        }
    };

    const testWorkflow = async () => {
        try {

            resetState(); // Reset state before testing

            if (!db_flow_id || !db_flow_version_id) {
                console.log("No flow or version id to test workflow");
                return;
            }

            setTestingWorkflow(true);
            setTestingMode(TestingMode.WORKFLOW);
            setPanelTab("testing");

            let results: StartWorkflowTestResult = await api.testing.testWorkflow(db_flow_id, db_flow_version_id);

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
    }

    const testAction = async (action_id: string) => {
        try {
            if (!db_flow_id || !db_flow_version_id) {
                console.log("No flow or version id to test action");
                return;
            }
            setTestingAction(true);
            setTestingMode(TestingMode.ACTION);
            setPanelTab("testing");
            await api.testing.testAction(db_flow_id, db_flow_version_id, action_id);
        } catch (error) {
            console.error(error);
        } finally {
            setTestingAction(false);
        }
    }

    return (
        <WorkflowTestingContext.Provider
            value={{
                testingMode,
                testingAction,
                workflowTestingSessionId,
                worklowTestingSessionTasks,
                testingWorkflow,
                testWorkflow,
                testAction
            }}
        >
            {children}
        </WorkflowTestingContext.Provider>
    );
};