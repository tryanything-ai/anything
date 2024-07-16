import { Action, Workflow } from "@/types/workflows";
import { createClient } from "../supabase/client";
import { v4 as uuidv4 } from "uuid";

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

export const testWorkflow = async (workflow_id: string, workflow_version_id: string) => {
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
    } finally {
    }
}