import { SupabaseClient } from '@supabase/supabase-js';

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getAccountBySlug = async (supabase: SupabaseClient, account_id: string, slug: string) => {
    try {
        console.log('getting account by slug in anything_api/accounts:', slug);
        const { data: { session } } = await supabase.auth.getSession();
 
        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/slug/${slug}`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/accounts/:slug', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching account by slug:', error);
    } 
}

export const getAccountInvitations = async (supabase: SupabaseClient, account_id: string) => {
    try {
        console.log('getting account invitations in anything_api/accounts:', account_id);
        const { data: { session } } = await supabase.auth.getSession();

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/invitations`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/accounts/invitations', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching account invitations:', error);
        throw error;
    }
}

export const getAccountMembers = async (supabase: SupabaseClient, account_id: string) => {
    try {
        console.log('getting account members in anything_api/accounts:', account_id);
        const { data: { session } } = await supabase.auth.getSession();

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/members`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/accounts/members', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching account members:', error);
        throw error;
    }
}