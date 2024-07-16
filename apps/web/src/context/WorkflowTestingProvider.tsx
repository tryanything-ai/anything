"use client"

import {
    createContext,
    ReactNode,
    useState
} from "react";
import { useWorkflowVersionContext } from "./WorkflowVersionProvider";
import api from "@/lib/anything-api";

export enum TestingMode {
    ACTION = "action",
    WORKFLOW = "workflow"
}

export interface WorkflowTestingContextInterface {
    testingMode: TestingMode;
    testingAction: boolean;
    testAction: (action_id: string) => Promise<void>;
    testingWorkflow: boolean;
    testWorkflow: (workflow_id: string) => Promise<void>;
}

export const WorkflowTestingContext = createContext<WorkflowTestingContextInterface>({
    testingMode: TestingMode.ACTION,
    testingAction: false,
    testAction: async () => { },
    testingWorkflow: false,
    testWorkflow: async () => { }
})

export const WorkflowTestingProvider = ({ children }: { children: ReactNode }) => {

    const { setPanelTab, db_flow_id, db_flow_version_id } = useWorkflowVersionContext();

    const [testingMode, setTestingMode] = useState<TestingMode>(TestingMode.ACTION);
    const [testingAction, setTestingAction] = useState<boolean>(false);
    const [testingWorkflow, setTestingWorkflow] = useState<boolean>(false);

    const testWorkflow = async (workflow_id: string) => {
        try {
            if (!db_flow_id || !db_flow_version_id) {
                console.log("No flow or version id ot test action");
                return;
            }

            setTestingWorkflow(true);
            setTestingMode(TestingMode.WORKFLOW);
            setPanelTab("testing");
            await api.testing.testWorkflow(db_flow_id, db_flow_version_id);
        } catch (error) {
            console.error(error);
        } finally {
            setTestingWorkflow(false);
        }
    }

    const testAction = async (action_id: string) => {
        try {
            if (!db_flow_id || !db_flow_version_id) {
                console.log("No flow or version id ot test action");
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
                testingWorkflow,
                testWorkflow,
                testAction
            }}
        >
            {children}
        </WorkflowTestingContext.Provider>
    );
};
