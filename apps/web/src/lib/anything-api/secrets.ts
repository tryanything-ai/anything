import { Action, Workflow } from "@/types/workflows";
import { createClient } from "../supabase/client";
import { v4 as uuidv4 } from "uuid";

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