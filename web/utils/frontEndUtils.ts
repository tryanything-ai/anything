import {
    Profile
} from "@/types/supabase.types";
export const formatUrl = (url: string): string => {
    // Remove the http or https and "www." from the beginning
    const formattedUrl = url.replace(/^(https?:\/\/)?(www\.)?/, "");
  
    // Remove trailing slash if it exists
    const cleanedUrl = formattedUrl.endsWith("/") ? formattedUrl.slice(0, -1) : formattedUrl;
  
    // If the string is longer than 30 characters, truncate and add ellipses
    if (cleanedUrl.length > 32) {
      return `${cleanedUrl.substring(0, 29)}...`;
    }
    return cleanedUrl;
};
  
export const hasLinks = (profile: Profile) => {
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