import { SupabaseClient } from '@supabase/supabase-js';

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
    action_id: string;
    type: string;
    plugin_name: string;
    plugin_version: string;
    stage: string;
    test_config?: Record<string, any>;
    config: Record<string, any>;
    context?: Record<string, any>;
    started_at?: string;
    ended_at?: string;
    debug_result?: Record<string, any>;
    result?: Record<string, any>;
    error?: Record<string, any>;
    archived: boolean;
    updated_at?: string;
    created_at?: string;
    updated_by?: string;
    created_by?: string;
    processing_order: number;
}

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const testAction = async (
    supabase: SupabaseClient,
    account_id: string,
    workflow_id: string,
    workflow_version_id: string,
    action_id: string
) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/testing/workflow/${workflow_id}/version/${workflow_version_id}/action/${action_id}`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Testing action via /api/account/id/workflow/id/version/id/action/id', data);
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

export const testWorkflow = async (
    supabase: SupabaseClient,
    account_id: string,
    workflow_id: string,
    workflow_version_id: string,
    trigger_session_id: string,
    flow_session_id: string
): Promise<StartWorkflowTestResult> => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/testing/workflow/${workflow_id}/version/${workflow_version_id}`, {
                headers: {
                    Authorization: `${session.access_token}`,
                    'Content-Type': 'application/json',
                },
                method: 'POST',
                body: JSON.stringify({
                    trigger_session_id,
                    flow_session_id
                })
            });
            const data = await response.json();
            console.log('Testing action via /api/account/id/workflow/id/version/id', data);
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

// WebSocket message types
export interface WorkflowTestingUpdate {
    type: 'workflow_update' | 'connection_established' | 'session_state';
    update_type?: 'task_created' | 'task_updated' | 'task_completed' | 'task_failed' | 'workflow_completed' | 'workflow_failed';
    flow_session_id: string;
    data?: any;
    tasks?: TaskRow[];
    complete?: boolean;
}

// WebSocket connection for workflow testing
export const createWorkflowTestingWebSocket = (
    supabase: SupabaseClient,
    account_id: string,
    flow_session_id: string,
    onMessage: (update: WorkflowTestingUpdate) => void,
    onError?: (error: Event) => void,
    onClose?: (event: CloseEvent) => void
): Promise<WebSocket | null> => {
    return new Promise(async (resolve, reject) => {
        try {
            const { data: { session } } = await supabase.auth.getSession();
            
            if (!session) {
                reject(new Error('No session available'));
                return;
            }

            // Convert HTTP URL to WebSocket URL
            const wsUrl = ANYTHING_API_URL?.replace('http://', 'ws://').replace('https://', 'wss://');
            const url = `${wsUrl}/account/${account_id}/testing/workflow/session/${flow_session_id}/ws?token=${encodeURIComponent(session.access_token)}`;
            
            console.log('[WEBSOCKET] Connecting to workflow testing WebSocket');
            console.log('[WEBSOCKET] Token length:', session.access_token.length);
            
            const ws = new WebSocket(url);
            
            ws.onopen = (event) => {
                console.log('[WEBSOCKET] Connected to workflow testing WebSocket');
                resolve(ws);
            };
            
            ws.onmessage = (event) => {
                try {
                    const update: WorkflowTestingUpdate = JSON.parse(event.data);
                    console.log('[WEBSOCKET] Received update:', update);
                    onMessage(update);
                } catch (error) {
                    console.error('[WEBSOCKET] Error parsing message:', error);
                }
            };
            
            ws.onerror = (error) => {
                console.error('[WEBSOCKET] WebSocket error:', error);
                if (onError) onError(error);
                reject(error);
            };
            
            ws.onclose = (event) => {
                console.log('[WEBSOCKET] WebSocket closed:', event.code, event.reason);
                if (onClose) onClose(event);
            };
            
        } catch (error) {
            console.error('[WEBSOCKET] Error creating WebSocket:', error);
            reject(error);
        }
    });
};




export const getTestingResults = async (
    supabase: SupabaseClient,
    account_id: string,
    workflow_id: string,
    workflow_version_id: string,
    workflow_session_id: string
) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        console.log('Getting Testing results:', workflow_id, workflow_version_id, workflow_session_id);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/testing/workflow/${workflow_id}/version/${workflow_version_id}/session/${workflow_session_id}`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Testing action via /api/account/id/workflow/id/version/id/session/id', data);
            return data;
        }
    } catch (error) {
        console.error('Error testing action:', error);
    }
}