import { SupabaseClient } from '@supabase/supabase-js';

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL;

export const getWorkflowVersionResults = async (
    supabase: SupabaseClient,
    account_id: string,
    workflow_id: string,
    workflow_version_id: string,
    action_id: string
) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        console.log('[VARIABLES API] Getting Results for Variable Explorer:', workflow_id, workflow_version_id, action_id);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/testing/workflow/${workflow_id}/version/${workflow_version_id}/action/${action_id}/results`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Getting Workfow Version Variables /api/account/id/workflow/id/version/id/session/id', data);
            return data;
        }
    } catch (error) {
        console.error('Error testing action:', error);
    }
}

export const getWorkflowVersionPluginInputs = async (
    supabase: SupabaseClient,
    account_id: string,
    workflow_id: string,
    workflow_version_id: string,
    action_id: string
) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        console.log('[VARIABLES API] Getting Results for Variable Explorer:', workflow_id, workflow_version_id, action_id);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/testing/workflow/${workflow_id}/version/${workflow_version_id}/action/${action_id}/variables`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Getting Workfow Version Variables /api/account/id/workflow/id/version/id/session/id', data);
            return data;
        }
    } catch (error) {
        console.error('Error testing action:', error);
    }
}

export const getSystemVariables = async (
    supabase: SupabaseClient,
    account_id: string
) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        console.log('[VARIABLES API] Getting System Variables');

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/testing/system_variables`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Getting System Variables /api/account/id/system_variables', data);
            return data;
        }
    } catch (error) {
        console.error('Error testing action:', error);
    }
}