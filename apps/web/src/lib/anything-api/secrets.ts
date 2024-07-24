import { createClient } from "../supabase/client";

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export async function createSecret(secret_name: string, secret_value: string, secret_description: string) {
    try {
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Creating Secret');

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/secret`, {
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


export const getSecrets = async () => {
    try {
        // Get JWT from supabase to pass to the API
        // API conforms to RLS policies on behalf of users for external API
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Session:', session);

        if (session) {
            const response = await fetch('http://localhost:3001/secrets', {
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

export async function deleteSecret(secret_id: string) {
    try {
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Deleting Secret');

        if (session) {
            const response = await fetch(`http://localhost:3001/secret/${secret_id}`, {
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

export async function updateSecret(secret_id: string, secret_vault_id: string, secret_value: string, secret_description: string) {
    try {
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Updating Secret');

        console.log('Session:', session);

        if (session) {
            const response = await fetch(`http://localhost:3001/secret`, {
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