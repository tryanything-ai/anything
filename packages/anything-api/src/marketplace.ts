import { createClient } from "./supabase/client";

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getActionTemplatesForMarketplace = async () => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        const headers: HeadersInit = {
            'Content-Type': 'application/json',
        };

        if (session) {
            headers['Authorization'] = `${session.access_token}`;
        }

        const response = await fetch(`${ANYTHING_API_URL}/marketplace/actions`, {
            headers: headers,
        });
        const data = await response.json();
        console.log('Data from /api/marketplace/actions:', data);
        return data;
    } catch (error) {
        console.error('Error fetching actions:', error);
    }
}

export const getWorkflowTemplatesForMarketplace = async () => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        const headers: HeadersInit = {
            'Content-Type': 'application/json',
        };

        if (session) {
            headers['Authorization'] = `${session.access_token}`;
        }

        const response = await fetch(`${ANYTHING_API_URL}/marketplace/workflows`, {
            headers: headers,
        });
        const data = await response.json();
        console.log('Data from /api/marketplace/workflows:', data);
        return data;
    } catch (error) {
        console.error('Error fetching workflows:', error);
    }
}

export const getWorkflowTemplateBySlugForMarketplace = async (slug: string) => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        const headers: HeadersInit = {
            'Content-Type': 'application/json',
        };

        if (session) {
            headers['Authorization'] = `${session.access_token}`;
        }

        const response = await fetch(`${ANYTHING_API_URL}/marketplace/workflow/${slug}`, {
            headers: headers,
        });
        const data = await response.json();
        console.log(`Data from /api/marketplace/workflows/${slug}:`, data);
        return data;
    } catch (error) {
        console.error('Error fetching workflow by slug:', error);
    }
}

export const publishFlowTemplateToMarketplace = async (account_id: string, workflow_id: string, workflow_version_id: string,  publish_to_marketplace_anonymously: boolean) => {
    try {
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        const headers: HeadersInit = {
            'Content-Type': 'application/json',
        };

        if (session) {
            headers['Authorization'] = `${session.access_token}`;
        }

        const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/marketplace/workflow/${workflow_id}/version/${workflow_version_id}/publish`, {
            method: 'POST',
            headers: headers,
            body: JSON.stringify({
                publish_to_marketplace_anonymously
            }),
        });
        const data = await response.json();
        console.log('Data from /api/marketplace/workflow/:id/verion/:id/publish:', data);
        return data;
    } catch (error) {
        console.error('Error publishing workflow template:', error);
    }
}