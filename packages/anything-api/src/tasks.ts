import { Action, Workflow } from "./types/workflows";
import { createClient } from "./supabase/client";


const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getTasks = async (account_id: string) => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/tasks`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/tasks:', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching tasks:', error);
    } finally {
    }
}

export const getTasksForWorkflow = async (account_id: string, workflow_id: string) => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/tasks/${workflow_id}`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/tasks/workflow_id:', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching tasks for workflow_id:', error);
    } 
}

//A single run of a whole workflow is called a "session" for now i think
export const getTasksForSession = async (account_id: string, session_id: string) => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/session/${session_id}/tasks`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/tasks:', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching tasks:', error);
    } finally {
    }
}
