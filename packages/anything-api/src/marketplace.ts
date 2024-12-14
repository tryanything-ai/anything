import { SupabaseClient } from '@supabase/supabase-js';

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getActionTemplatesForMarketplace = async () => {
    try {
        const url = `${ANYTHING_API_URL}/marketplace/actions`;
        console.log(`[MARKETPLACE.TS] Fetching from: ${url}`);
        const response = await fetch(url);
        const data = await response.json();
        console.log('[MARKETPLACE.TS] Data from /api/marketplace/actions:', data);
        return data;
    } catch (error) {
        console.error('[MARKETPLACE.TS] Error fetching actions:', error);
    }
}

export const getWorkflowTemplatesForMarketplace = async () => {
    try {
        const url = `${ANYTHING_API_URL}/marketplace/workflows`;
        console.log(`[MARKETPLACE.TS] Fetching from: ${url}`);
        const response = await fetch(url);
        const data = await response.json();
        console.log('[MARKETPLACE.TS] Data from /api/marketplace/workflows:', data);
        return data;
    } catch (error) {
        console.error('[MARKETPLACE.TS] Error fetching workflows:', error);
    }
}

export const getWorkflowTemplateBySlugForMarketplace = async (slug: string) => {
    try {
        const url = `${ANYTHING_API_URL}/marketplace/workflow/${slug}`;
        console.log(`[MARKETPLACE.TS] Fetching from: ${url}`);
        const response = await fetch(url);
        const data = await response.json();
        console.log(`[MARKETPLACE.TS] Data from /api/marketplace/workflows/${slug}:`, data);
        return data;
    } catch (error) {
        console.error('[MARKETPLACE.TS] Error fetching workflow by slug:', error);
    }
}

export const publishFlowTemplateToMarketplace = async (supabase: SupabaseClient, account_id: string, workflow_id: string, workflow_version_id: string, publish_to_marketplace_anonymously: boolean) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('[MARKETPLACE.TS] Session:', session);

        const headers: HeadersInit = {
            'Content-Type': 'application/json',
        };

        if (session) {
            headers['Authorization'] = `${session.access_token}`;
        }

        const url = `${ANYTHING_API_URL}/account/${account_id}/marketplace/workflow/${workflow_id}/version/${workflow_version_id}/publish`;
        console.log(`[MARKETPLACE.TS] Posting to: ${url}`);
        const response = await fetch(url, {
            method: 'POST',
            headers: headers,
            body: JSON.stringify({
                publish_to_marketplace_anonymously
            }),
        });
        const data = await response.json();
        console.log('[MARKETPLACE.TS] Data from /api/marketplace/workflow/:id/verion/:id/publish:', data);
        return data;
    } catch (error) {
        console.error('[MARKETPLACE.TS] Error publishing workflow template:', error);
    }
}

export const cloneWorkflowTemplate = async (supabase: SupabaseClient, account_id: string, template_id: string) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('[MARKETPLACE.TS] Session:', session);

        const headers: HeadersInit = {
            'Content-Type': 'application/json',
        };

        if (session) {
            headers['Authorization'] = `${session.access_token}`;
        }

        const url = `${ANYTHING_API_URL}/account/${account_id}/marketplace/workflow/${template_id}/clone`;
        console.log(`[MARKETPLACE.TS] Fetching from: ${url}`);
        const response = await fetch(url, {
            method: 'GET',
            headers: headers,
        });
        const data = await response.json();
        console.log(`[MARKETPLACE.TS] Data from /api/marketplace/workflow/${template_id}/clone:`, data);
        return data;
    } catch (error) {
        console.error('[MARKETPLACE.TS] Error cloning workflow template:', error);
    }
}
