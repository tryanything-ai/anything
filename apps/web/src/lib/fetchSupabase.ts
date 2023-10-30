import { createClient } from "@supabase/supabase-js";

// import { env } from "@/env.mjs";
// import { FAKE_FLOW_VERSIONS, FAKE_PROFILES } from "@/mocks/supabaseMock";
import { Database } from "@/types/supabase.types";
import * as SUPABASE from "@/types/supabase.types";

export * from "@/types/supabase.types";

console.log("process.env.NEXT_PUBLIC_SUPABASE_URL", process.env.NEXT_PUBLIC_SUPABASE_URL);

const supabaseUrl = process.env.NEXT_PUBLIC_SUPABASE_URL
  ? process.env.NEXT_PUBLIC_SUPABASE_URL
  : "";
const supabaseAnonKey = process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY
  ? process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY
  : "";

export const supabase = createClient<Database>(supabaseUrl, supabaseAnonKey);

const templatesQuery = supabase
  .from("flow_templates")
  .select("*, flow_template_versions(*), tags(*), profiles(*)");

export type BigFlow = SUPABASE.DbResultOk<typeof templatesQuery>;

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

export const fetchTemplateBySlug = async (
  slug: string
): Promise<BigFlow | undefined> => {
  try {
    const { data, error }: SUPABASE.DbResult<typeof templatesQuery> =
      await templatesQuery.eq("slug", slug);

    // console.log("data in fetchTemplateBySlug", JSON.stringify(data, null, 3));
    if (error || !data) throw error;

    return data;
  } catch (e) {
    console.log(e);
    return undefined;
  }
};

export const fetchProfileTemplates = async (
  username: string
): Promise<BigFlow | undefined> => {
  try {
    const { data, error }: SUPABASE.DbResult<typeof templatesQuery> =
      await templatesQuery.eq("profiles.username", username);

    // console.log("data", JSON.stringify(data, null, 3));
    if (error || !data) throw error;

    return data;
  } catch (e) {
    console.log(e);
    return undefined;
  }
};

export const fetchProfiles = async () => {
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

export const fetchProfile = async (
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
