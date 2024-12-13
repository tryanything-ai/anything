import { Action, Workflow } from "./types/workflows";
import { SupabaseClient } from '@supabase/supabase-js';
import { v4 as uuidv4 } from "uuid";

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getActionTemplatesForAccount = async (supabase: SupabaseClient, account_id: string) => {
    try {
        console.log('Finding action templates for account:', account_id);
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/actions`, {
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/actions:', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching actions:', error);
    }
}

export const publishActionTemplate = async (supabase: SupabaseClient, account_id: string, action: Action, publish_to_team: boolean, publish_to_marketplace: boolean, publish_to_marketplace_anonymously: boolean) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/marketplace/action/publish`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
                body: JSON.stringify({
                    publish_to_team,
                    publish_to_marketplace,
                    publish_to_marketplace_anonymously,
                    action_template_definition: action,
                }),
            });
            const data = await response.json();
            console.log('Data from /api/marketplace/action/publish:', data);
            return data;
        }
    } catch (error) {
        console.error('Error publishing action template:', error);
    }
}

export const getTriggerTemplatesForAccount = async (supabase: SupabaseClient, account_id: string) => {
    try {
        console.log('Finding action templates for account:', account_id);
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/triggers`, {
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
            });

            const data = await response.json();
            console.log('Data from /api/triggers:', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching actions:', error);
    }
}

export const getOtherActionTemplatesForAccount = async (supabase: SupabaseClient, account_id: string) => {
    try {
        console.log('Finding other templates for account:', account_id);
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/other`, {
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
            });

            const data = await response.json();
            console.log('Data from /api/other:', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching other actions:', error);
    }
}