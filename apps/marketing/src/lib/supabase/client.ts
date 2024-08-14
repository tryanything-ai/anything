import { createClient } from "@supabase/supabase-js";


let supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL!; 
let supabaseAnonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!; 

export const supabaseClient = createClient(
  supabaseUrl,
  supabaseAnonKey
);
