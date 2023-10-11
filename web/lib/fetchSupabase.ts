import { createClient } from "@supabase/supabase-js";
import { Database } from "@/types/supabase.types";
import { FAKE_FLOW_VERSIONS, FAKE_PROFILES } from "@/mocks/supabaseMock";

import * as SUPABASE from  "@/types/supabase.types";
export * from "@/types/supabase.types";


import { env } from "@/env.mjs";

const supabaseUrl = env.NEXT_PUBLIC_SUPABASE_URL;
const supabaseAnonKey = env.NEXT_PUBLIC_SUPABASE_ANON_KEY;

export const supabase = createClient<Database>(supabaseUrl, supabaseAnonKey);

const templatesQuery = supabase
  .from("flow_template_versions")
  .select("*, flow_template_tags:flow_template_id(*)");

export const _fetchTemplates = async (): Promise<SUPABASE.FlowTemplateVersion[] | undefined> => {
  try {
    const { data, error }: SUPABASE.DbResult<typeof templatesQuery> =
      await templatesQuery;
    
    console.log("data", data); 
    if (error || !data) throw error;

    return data;
  } catch (e) {
    console.log(e);
    return undefined;
  }
};

export const fetchTemplates = env.NEXT_PUBLIC_MOCK_ALL
  ? () => FAKE_FLOW_VERSIONS
  : _fetchTemplates;

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
};

export const fetchProfiles = env.NEXT_PUBLIC_MOCK_ALL
  ? () => FAKE_PROFILES
  : _fetchProfiles;

const _fetchProfile = async (
  username: string
): Promise<SUPABASE.Profile | undefined> => {
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
};

export const fetchProfile = env.NEXT_PUBLIC_MOCK_ALL
  ? (username: string) =>
      FAKE_PROFILES.find((profile) => profile.username === username)
  : _fetchProfile;
