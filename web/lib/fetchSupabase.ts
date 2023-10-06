import { env } from "@/env.mjs";

import { createClient } from "@supabase/supabase-js";
import { Database } from "@/types/supabase.types";

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