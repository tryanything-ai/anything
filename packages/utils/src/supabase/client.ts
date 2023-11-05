import { createClient } from "@supabase/supabase-js";

import { Database } from "./types/supabase.types";

export * from "./types/supabase.types";

//For tauri because we need import.meta for vite build and cant find a good solution
const loadEnv = (VAR_NAME: string): string => {
  if (typeof process !== "undefined" && process.env) {
    console.log("using process.env");
    // Node.js environment
    return process.env[VAR_NAME] || "";
  } else {
    console.log("using import.meta");
    return import.meta.env[VAR_NAME] || "";
  }
};

let supabaseUrl = loadEnv("NEXT_PUBLIC_SUPABASE_URL");
let supabaseAnonKey = loadEnv("NEXT_PUBLIC_SUPABASE_ANON_KEY");

export const supabaseClient = createClient<Database>(
  supabaseUrl,
  supabaseAnonKey
);
