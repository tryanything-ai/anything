import { SupabaseClient } from '@supabase/supabase-js';

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export enum TimeUnit {
    Minute = "minute",
    Hour = "hour", 
    Day = "day",
    Week = "week",
    Month = "month"
}

export const getTasksChartForWorkflow = async (supabase: SupabaseClient, account_id: string, workflow_id: string, start_date: string, end_date: string, time_unit: TimeUnit, timezone: string) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/charts/${workflow_id}/tasks/${start_date}/${end_date}/${time_unit}/${timezone}`, {
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

export const getTasksChartForAccount = async (supabase: SupabaseClient, account_id: string, start_date: string, end_date: string, time_unit: TimeUnit, timezone: string) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/charts/tasks/${start_date}/${end_date}/${time_unit}/${timezone}`, {
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