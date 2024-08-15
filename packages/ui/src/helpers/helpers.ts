// import { Profile } from "./types/supabase.types";

export const flowJsonFromBigFlow = (template: any) => {
  // TODO: this whole thing is kinda garbage and related to typescript problems with supabase queryes that are nested
  let flow_json: any;

  if (
    template.flow_template_versions &&
    Array.isArray(template.flow_template_versions) &&
    template.flow_template_versions.length > 0
  ) {
    flow_json = template.flow_template_versions[0].flow_template_json;
  } else {
    flow_json = null;
  }

  return flow_json;
};

export const getAProfileLink = (profile: any): string | undefined => {
  if (profile.twitter) return profile.twitter;
  if (profile.linkedin) return profile.linkedin;
  if (profile.github) return profile.github;
  if (profile.website) return profile.website;
  if (profile.instagram) return profile.instagram;
  if (profile.tiktok) return profile.tiktok;
  if (profile.youtube) return profile.youtube;
  return undefined;
};

export const formatUrl = (url: string): string => {
  // Remove the http or https and "www." from the beginning
  const formattedUrl = url.replace(/^(https?:\/\/)?(www\.)?/, "");

  // Remove trailing slash if it exists
  const cleanedUrl = formattedUrl.endsWith("/")
    ? formattedUrl.slice(0, -1)
    : formattedUrl;

  // If the string is longer than 30 characters, truncate and add ellipses
  if (cleanedUrl.length > 32) {
    return `${cleanedUrl.substring(0, 29)}...`;
  }
  return cleanedUrl;
};

export const hasLinks = (profile: any) => {
  return (
    profile.twitter ||
    profile.linkedin ||
    profile.github ||
    profile.website ||
    profile.instagram ||
    profile.tiktok ||
    profile.youtube
  );
};
