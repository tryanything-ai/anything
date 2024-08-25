import { createBrowserClient } from "@supabase/ssr";

// @ts-ignore
import { SupabaseClient } from "@supabase/supabase-js";

export const createClient = (): SupabaseClient =>
  createBrowserClient(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!,
    { 
      auth: {
      autoRefreshToken: false,
      persistSession: false,
      detectSessionInUrl: false,
      flowType: "implicit", 
      debug: true,
    }
  }
  );