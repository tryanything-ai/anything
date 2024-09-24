import { Action, Workflow } from "./types/workflows";
import { createClient } from "./supabase/client";
import { v4 as uuidv4 } from "uuid";

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export enum TimeUnit {
    Minute = "minute",
    Hour = "hour",
    Day = "day",
    Week = "week",
    Month = "month"
}

export const getTasksChart = async (account_id: string, workflow_id: string, start_date: string, end_date: string, time_unit: TimeUnit) => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/charts/${workflow_id}/tasks/${start_date}/${end_date}/${time_unit}`, {
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
