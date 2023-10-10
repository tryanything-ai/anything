import { env } from "@/env.mjs";

import { createClient } from "@supabase/supabase-js";
import { Database } from "@/types/supabase.types";
import { FakeProfiles } from "@/mocks/supabaseMock";

const supabaseUrl = env.NEXT_PUBLIC_SUPABASE_URL;
const supabaseAnonKey = env.NEXT_PUBLIC_SUPABASE_ANON_KEY;

export const supabase = createClient<Database>(supabaseUrl, supabaseAnonKey);

export const fetchTemplates = async () => {
    try {
        let { data: flow_templates, error } = await supabase
            .from("flow_templates")
            .select("*");

        if (error) throw error;

        return flow_templates;
        
    } catch (e) {
        console.log(e);
        return undefined; 
    }
}

const _fetchProfiles = async () => {
    try {
        let { data: profiles, error } = await supabase
            .from("profiles")
            .select("*")
            .eq("public", true);

        if (error) throw error;

        return profiles;
        
    } catch (e) {
        console.log(e);
        return undefined; 
    }
}

export const fetchProfiles = env.NEXT_PUBLIC_MOCK_ALL ? () => FakeProfiles : _fetchProfiles;

export type Profile = Database['public']['Tables']['profiles']['Row'];

const _fetchProfile = async (username: string): Promise<Profile | undefined> => {
    try {
        let { data: profile, error } = await supabase
            .from("profiles")
            .select("*")
            .eq("username", username)
            .single(); 

        if (error || !profile) throw error;

        return profile;
        
    } catch (e) {
        console.log(e);
        return undefined; 
    }
}

export const fetchProfile = env.NEXT_PUBLIC_MOCK_ALL ? (username: string) => FakeProfiles.find(profile => profile.username === username) : _fetchProfile;