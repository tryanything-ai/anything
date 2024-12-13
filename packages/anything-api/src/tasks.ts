import { Action, Workflow } from "./types/workflows";
import { SupabaseClient } from '@supabase/supabase-js';

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getTasks = async (supabase: SupabaseClient, account_id: string) => {
    try {
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

export const getTasksForWorkflow = async (supabase: SupabaseClient, account_id: string, workflow_id: string) => {
    try {
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
export const getTasksForSession = async (supabase: SupabaseClient, account_id: string, session_id: string) => {
    try {
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
