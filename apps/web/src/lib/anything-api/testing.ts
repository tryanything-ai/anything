import { createClient } from "../supabase/client";

export interface TaskRow {
    task_id: string;
    account_id: string;
    task_status: string;
    flow_id: string;
    flow_version_id: string;
    action_label: string;
    trigger_id: string;
    trigger_session_id: string;
    trigger_session_status: string;
    flow_session_id: string;
    flow_session_status: string;
    node_id: string;
    action_type: string;
    plugin_id: string;
    stage: string;
    test_config?: Record<string, any>;
    config: Record<string, any>;
    context?: Record<string, any>;
    started_at?: string;
    ended_at?: string;
    debug_result?: Record<string, any>;
    result?: Record<string, any>;
    archived: boolean;
    updated_at?: string;
    created_at?: string;
    updated_by?: string;
    created_by?: string;
    processing_order: number;
}

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const testAction = async (workflow_id: string, workflow_version_id: string, action_id: string) => {
    try {
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/testing/workflow/${workflow_id}/version/${workflow_version_id}/action/${action_id}`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Testing action via /api/workflow/id/version/id/action/id', data);
            return data;
        }
    } catch (error) {
        console.error('Error testing action:', error);
    } finally {
    }
}

export type StartWorkflowTestResult = {
    flow_session_id: string;
    trigger_session_id: string;
} | undefined; 

export const testWorkflow = async (workflow_id: string, workflow_version_id: string): Promise<StartWorkflowTestResult> => {
    try {
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/testing/workflow/${workflow_id}/version/${workflow_version_id}`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Testing action via /api/workflow/id/version/id', data);
            return data;
        }
    } catch (error) {
        console.error('Error testing action:', error);
    } 
}

export type WorklfowTestSessionResult = {
    tasks: TaskRow[];
    complete: boolean;
} | undefined;


export const getTestingResults = async (workflow_id: string, workflow_version_id: string, workflow_session_id: string) => {
    try {
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        console.log('Getting Testing results:', workflow_id, workflow_version_id, workflow_session_id);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/testing/workflow/${workflow_id}/version/${workflow_version_id}/session/${workflow_session_id}`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Testing action via /api/workflow/id/version/id/session/id', data);
            return data;
        }
    } catch (error) {
        console.error('Error testing action:', error);
    }
}