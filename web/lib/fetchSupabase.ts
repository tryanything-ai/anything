import { createClient } from "@supabase/supabase-js";
import { Database } from "@/types/supabase.types";
import { FAKE_FLOW_VERSIONS, FAKE_PROFILES } from "@/mocks/supabaseMock";

import * as SUPABASE from "@/types/supabase.types";
export * from "@/types/supabase.types";

import { env } from "@/env.mjs";

const supabaseUrl = env.NEXT_PUBLIC_SUPABASE_URL;
const supabaseAnonKey = env.NEXT_PUBLIC_SUPABASE_ANON_KEY;

export const supabase = createClient<Database>(supabaseUrl, supabaseAnonKey);

//TODO: probably need to make a view for this
const templatesQuery = supabase
  .from("flow_templates")
  .select("*, flow_template_versions(*), tags(*), profiles(*)");

export type BigFlow = SUPABASE.DbResultOk<typeof templatesQuery>

export const fetchTemplates = async (): Promise<BigFlow | undefined> => {
  try {
    const { data, error }: SUPABASE.DbResult<typeof templatesQuery> =
      await templatesQuery;

    // console.log("data", JSON.stringify(data, null, 3));
    if (error || !data) throw error;

    return data;
  } catch (e) {
    console.log(e);
    return undefined;
  }
};

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

export const fetchProfiles =
  env.NEXT_PUBLIC_MOCK_ALL === "true" ? () => FAKE_PROFILES : _fetchProfiles;

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

export const fetchProfile =
  env.NEXT_PUBLIC_MOCK_ALL === "true"
    ? (username: string) =>
        FAKE_PROFILES.find((profile) => profile.username === username)
    : _fetchProfile;
