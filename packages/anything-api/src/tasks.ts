import { SupabaseClient } from '@supabase/supabase-js';

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export interface PaginationParams {
    page?: number;
    page_size?: number;
    search?: string;
}

export interface PaginatedResponse<T> {
    data: T[];
    pagination: {
        page: number;
        page_size: number;
        total: number;
    };
}

export const getTasks = async (
    supabase: SupabaseClient, 
    account_id: string,
    pagination?: PaginationParams
): Promise<PaginatedResponse<any>> => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        if (!session) {
            throw new Error('No session found');
        }

        const queryParams = new URLSearchParams();
        if (pagination?.page) {
            queryParams.append('page', pagination.page.toString());
        }
        if (pagination?.page_size) {
            queryParams.append('page_size', pagination.page_size.toString());
        }
        if (pagination?.search) {
            queryParams.append('search', pagination.search);
        }

        const queryString = queryParams.toString();
        const url = `${ANYTHING_API_URL}/account/${account_id}/tasks${queryString ? `?${queryString}` : ''}`;

        const response = await fetch(url, {
            headers: {
                Authorization: `${session.access_token}`,
            },
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        return data;
    } catch (error) {
        console.error('Error fetching tasks:', error);
        throw error;
    }
}

export const getTasksForWorkflow = async (
    supabase: SupabaseClient, 
    account_id: string, 
    workflow_id: string,
    pagination?: PaginationParams
): Promise<PaginatedResponse<any>> => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        if (!session) {
            throw new Error('No session found');
        }

        const queryParams = new URLSearchParams();
        if (pagination?.page) {
            queryParams.append('page', pagination.page.toString());
        }
        if (pagination?.page_size) {
            queryParams.append('page_size', pagination.page_size.toString());
        }
        if (pagination?.search) {
            queryParams.append('search', pagination.search);
        }

        const queryString = queryParams.toString();
        const url = `${ANYTHING_API_URL}/account/${account_id}/tasks/${workflow_id}${queryString ? `?${queryString}` : ''}`;

        const response = await fetch(url, {
            headers: {
                Authorization: `${session.access_token}`,
            },
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        const data = await response.json();
        return data;
    } catch (error) {
        console.error('Error fetching tasks for workflow_id:', error);
        throw error;
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
