import { Action, Workflow } from "@/types/workflows";
import { createClient } from "../supabase/client";
import { v4 as uuidv4 } from "uuid";

export async function createSecret(secret_name: string, secret_value: string) {
    try {
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();

        console.log('Creating Secret');

        if (session) {
            const response = await fetch(`http://localhost:3001/secret`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
                body: JSON.stringify({
                    secret_name,
                    secret_value,
                    secret_description: 'test'
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