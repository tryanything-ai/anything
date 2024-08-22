import { createClient } from "../supabase/client";

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getProvider = async (provider_name: string) => {
    try {

        console.log('getting provider_name in anything_api/auth:', provider_name);
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/auth/providers/${provider_name}`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/auth/:provider_name', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching provier by provider_name:', error);
    } 
}

export const getAuthAccounts = async () => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/auth/accounts`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/auth/accounts', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching auth accoutns', error);
    } 
}

export const getAuthAccountsForProvider = async (provider_name: string) => {
    try {
        console.log('getting auth accounts for provider: ', provider_name);
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/auth/accounts/${provider_name}`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/auth/accounts/:provider_name', data);
            return data;
        }

    } catch (error) {
        console.error('Error fetching auth accounts for provider:', error);
    } 
}

export const getProviders = async () => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/auth/providers`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/auth/providers', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching auth providers', error);
    } 
}

export const handleCallbackForProvider = async ({provider_name, code, state, }: {provider_name: string, code: any, state: any}) => {
    try {
       
        const supabase = createClient();
        const { data: { session }, error } = await supabase.auth.getSession();

        console.log('Session:', session);
        console.log('Error:', error);

        if (session) {
            console.log("calling /api/auth/:provider_name/callback");
            const response = await fetch(`${ANYTHING_API_URL}/auth/${provider_name}/callback`, {
                method: 'POST',
                headers: {
                  'Content-Type': 'application/json',
                  Authorization: `${session.access_token}`,
                },
                body: JSON.stringify({
                  code,
                  state
                }),
            });
            const data = await response.json();
            console.log('Data from /api/auth/:provider_name/callback', data);
            return data;
        } else {
            console.error('No session found in handleCallbackForProvider');
        }
    } catch (error) {
        console.error('Error handling callback', error);
    } 
}
