import { SupabaseClient } from '@supabase/supabase-js';

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export async function createSecret(supabase: SupabaseClient, account_id: string, secret_name: string, secret_value: string, secret_description: string) {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Creating Secret');

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/secret`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
                body: JSON.stringify({
                    secret_name,
                    secret_value,
                    secret_description,
                }),
            });

            const data = await response.json();
            console.log('Data from /api/secret POST:', data);
            return data;
        }

    } catch (error) {
        console.error('Error creating Secret:', error);
    } finally {
    }
}


export const getSecrets = async (supabase: SupabaseClient, account_id: string) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/secrets`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/secrets:', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching secrets:', error);
    } finally {
    }
}

export const getAnythingApiKeys = async (supabase: SupabaseClient, account_id: string) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/keys`, {
                headers: {
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /api/keys:', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching api keys:', error);
    } finally {
    }
}

export async function createAnythingApiKey(supabase: SupabaseClient, account_id: string, secret_name: string, secret_description: string) {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Creating Api Key');

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/key`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
                body: JSON.stringify({
                    secret_name,
                    secret_description,
                }),
            });

            const data = await response.json();
            console.log('Data from /api/key POST:', data);
            return data;
        }

    } catch (error) {
        console.error('Error creating api key:', error);
    } finally {
    }
}


export async function deleteAnythingApiKey(supabase: SupabaseClient, account_id: string, secret_id: string) {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Deleting Api Key');

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/key/${secret_id}`, {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                }
            });

            const data = await response.json();
            console.log('Data from /api/key DELETE:', data);
            return data;
        }

    } catch (error) {
        console.error('Error deleting api key:', error);
    } finally {
    }
}


export async function deleteSecret(supabase: SupabaseClient, account_id: string, secret_id: string) {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Deleting Secret');

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/secret/${secret_id}`, {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                }
            });

            const data = await response.json();
            console.log('Data from /api/secret DELETE:', data);
            return data;
        }

    } catch (error) {
        console.error('Error deleting Secret:', error);
    } finally {
    }
}

export async function updateSecret(supabase: SupabaseClient, account_id: string, secret_id: string, secret_vault_id: string, secret_value: string, secret_description: string) {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Updating Secret');

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/secret`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
                body: JSON.stringify({
                    secret_id,
                    secret_vault_id,
                    secret_value,
                    secret_description,
                }),
            });

            const data = await response.json();
            console.log('Data from /api/secret PUT:', data);
            return data;
        }

    } catch (error) {
        console.error('Error updating Secret:', error);
    } finally {
    }
}