// @ts-ignore
import { PostgrestBuilder } from "@supabase/postgrest-js";
//above line needs to stay or all the types freak out in consuming packages and apps
import * as SUPABASE from "./types/supabase.types";

export * from "./types/supabase.types";
import * as types from "./types/supabase.types";
import slugify from "slugify";

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

export const fetchTemplateById = async (
  id: string
): Promise<BigFlow | undefined> => {
  try {
    // console.log("fetchTemplateById in supabase", id);
    const { data, error }: SUPABASE.DbResult<typeof templatesQuery> =
      await templatesQuery.eq("flow_template_id", id);
    // console.log("data in fetchTemplateById", JSON.stringify(data, null, 3));
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

const manageSlugUpdate = async (slug: string): Promise<string> => {
  let newSlug = slug;
  let slugExists = true;
  let count = 0;

  while (slugExists) {
    const { data: slug_data, error: slug_error } = await supabaseClient
      .from("flow_templates")
      .select("slug")
      .eq("slug", newSlug);
    count++;

    if (slug_error) throw slug_error;
    if (count > 10) throw new Error("Too many slug conflicts");

    if (slug_data && slug_data.length > 0) {
      console.log("Making New Slug");
      newSlug = slug + "-" + count;
    } else {
      slugExists = false;
    }
  }

  return newSlug;
};

export const saveFlowTemplate = async (
  flow_template_id: string,
  flow_template_version_id: string,
  flow_template_name: string,
  flow_template_description: string,
  flow_template_json: any,
  publisher_id: string,
  anything_flow_template_version: string
) => {
  try {
    //validate client side id's sent for consistancy.
    if (!flow_template_id || !flow_template_version_id)
      throw new Error(
        "flow_template_id or flow_template_version_id is undefined"
      );

    // make new slug if we have conflicts
    let slug = await manageSlugUpdate(
      slugify(flow_template_name, { lower: true })
    );

    // Save Template
    const { data, error } = await supabaseClient
      .from("flow_templates")
      .insert({
        anonymous_publish: false,
        flow_template_id,
        flow_template_name,
        flow_template_description,
        slug,
        public_template: true,
        publisher_id,
      })
      .select()
      .single();

    console.log("data", data);
    if (error) throw error;

    if (!data) throw new Error("data is undefined");

    let result = await saveFlowTemplateVersion(
      data.flow_template_id,
      flow_template_version_id,
      flow_template_name,
      flow_template_json,
      true,
      "0.0.0",
      "Initial Commit",
      publisher_id,
      anything_flow_template_version
    );

    if (!result) throw new Error("result is undefined");

    console.log("result", result);
    //for some consistency on front end.
    return await fetchTemplateById(data.flow_template_id);
  } catch (e) {
    console.log(e);
  }
};

export const saveFlowTemplateVersion = async (
  flow_template_id: string,
  flow_template_version_id: string,
  flow_template_version_name: string,
  flow_template_json: any,
  public_template: boolean,
  flow_template_version: string,
  commit_message: string,
  publisher_id: string,
  anything_flow_template_version: string
) => {
  try {
    // Save Template Version
    const { data, error } = await supabaseClient
      .from("flow_template_versions")
      .insert({
        flow_template_id,
        flow_template_version_id,
        flow_template_json,
        publisher_id,
        anything_flow_template_version,
        flow_template_version_name,
        flow_template_version,
        public_template,
        recommended_version: true,
        commit_message,
      })
      .select()
      .single();

    if (error) throw error;

    if (!data) throw new Error("data is undefined");
    return data;
  } catch (e) {
    console.log(e);
  }
};
