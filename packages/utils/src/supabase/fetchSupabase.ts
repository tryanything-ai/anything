// @ts-ignore
import { PostgrestBuilder } from "@supabase/postgrest-js";
import * as SUPABASE from "./types/supabase.types";

export * from "./types/supabase.types";
import * as types from "./types/supabase.types";

import { supabaseClient } from "./client";

const templatesQuery = supabaseClient
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
    if (!username) throw new Error("username is undefined");
    const templatesQuery2 = supabaseClient
      .from("flow_templates")
      .select("*, flow_template_versions(*), tags(*), profiles!inner(*)")
      .eq("profiles.username", username);

    const { data, error }: SUPABASE.DbResult<typeof templatesQuery2> =
      await templatesQuery2;

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
    let { data: profiles, error } = await supabaseClient
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
    let { data: profile, error } = await supabaseClient
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

export const updateProfile = async (profile_id: string, updateData: any) => {
  try {
    updateData.updated_at = new Date().toISOString();

    const { data, error } = await supabaseClient
      .from("profiles")
      .update(updateData)
      .eq("id", profile_id)
      .select()
      .single();

    if (error) throw error;

    return data;
  } catch (e) {
    console.log(e);
    return undefined;
  }
};

export const uploadAvatar = async (
  profile_id: string,
  filePath: string,
  file: any
): Promise<types.Profile | unknown> => {
  try {
    const { error: uploadError, data: uploadData } =
      await supabaseClient.storage.from("avatars").upload(filePath, file);

    if (uploadError) {
      throw uploadError;
    }

    //get public url
    const { data: publicUrlData } = supabaseClient.storage
      .from("avatars")
      .getPublicUrl(uploadData.path);

    console.log("publicUrlData", publicUrlData);

    if (!publicUrlData) throw new Error("publicUrlData is undefined");
    //TODO: update profile with avatar url
    const { data, error } = await supabaseClient
      .from("profiles")
      .update({ avatar_url: publicUrlData.publicUrl })
      .eq("id", profile_id)
      .select()
      .single();

    if (error) throw error;

    return data;
  } catch (e) {
    console.log(e);
    return e;
  }
};
