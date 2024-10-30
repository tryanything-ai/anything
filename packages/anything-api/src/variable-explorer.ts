import { createClient } from "./supabase/client";

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL;

export const getWorkflowVersionVariables = async (account_id: string, workflow_id: string, workflow_version_id: string, action_id: string) => {
    try {
        const supabase = createClient();
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