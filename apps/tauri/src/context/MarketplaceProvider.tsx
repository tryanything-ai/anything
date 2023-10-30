import {
  BigFlow,
  fetchProfile,
  fetchTemplateBySlug,
  fetchTemplates,
  Profile,
} from "utils";


import { createContext, ReactNode, useContext } from "react";

import { useSettingsContext } from "./SettingsProvider";

interface MarketplaceContextInterface {
  searchTemplates: (searchTerm: string) => void;
  fetchTemplates: () => Promise<BigFlow>;
  saveTemplate: (template: any) => void;
  updateTemplate: (template: any) => void;
  fetchTemplateBySlug: (slug: string) => Promise<BigFlow | undefined>;
  fetchProfile: (username: string) => Promise<Profile | undefined>;
}

export const MarketplaceContext = createContext<MarketplaceContextInterface>({
  searchTemplates: () => {},
  fetchTemplates: () => Promise.resolve([]),
  fetchTemplateBySlug: () => Promise.resolve(undefined),
  fetchProfile: () => Promise.resolve(undefined),
  saveTemplate: () => {},
  updateTemplate: () => {},
});

export const useMarketplaceContext = () => useContext(MarketplaceContext);

//We will break compatability of templates and will need to know what version of templates we are using.
//was used to create a template to manage compatability and conversion
const FLOW_TEMPLATES_VERSION = "0.0.1";

export const MarketplaceProvider = ({ children }: { children: ReactNode }) => {
  const { webFeaturesDisabled } = useSettingsContext();
  //fetch Supabaes Flow Templates
  //expose Supabase Search for Flows, Actions, Triggers, Templates, etc

  const searchTemplates = (searchTerm: string) => {
    if (webFeaturesDisabled) return [];
    return [];
    //Do supabase stuff.
  };

  const _fetchTemplates = async () => {
    if (webFeaturesDisabled) return [];

    //Do supabase stuff.

    let templateResponse = await fetchTemplates();

    // let templates = await fetchTemplates();
    // let { data: flow_templates, error } = await supabase
    //   .from("flow_templates")
    //   .select("*");

    if (!templateResponse) return [];
    else return templateResponse;

    // if (error) {
    //   console.log(error);
    //   return [];
    // }

    // let templates = flow_templates?.map((template) => {
    //   //TODO: this might be very naughty
    //   return template.flow_json as unknown as RustFlow;
    // });
    //TODO: rebuild with new types
    // return [];
  };

  const _fetchTemplateBySlug = async (slug: string) => {
    if (webFeaturesDisabled) return undefined;

    let templateResponse = await fetchTemplateBySlug(slug);

    if (!templateResponse) return undefined;
    else return templateResponse;
  };

  const _fetchProfile = async (username: string) => {
    if (webFeaturesDisabled) return undefined;

    //Do supabase stuff.
    let profile = await fetchProfile(username);
    if (!profile) return undefined;
    else return profile as Profile;
  };

  const saveTemplate = (template: any) => {
    if (webFeaturesDisabled) return false;

    //Do supabase stuff.
  };

  const updateTemplate = (template: any) => {
    if (webFeaturesDisabled) return false;

    //Do supabase stuff.
  };

  return (
    <MarketplaceContext.Provider
      value={{
        searchTemplates,
        fetchTemplates: _fetchTemplates,
        saveTemplate,
        updateTemplate,
        fetchTemplateBySlug: _fetchTemplateBySlug,
        fetchProfile: _fetchProfile,
      }}
    >
      {children}
    </MarketplaceContext.Provider>
  );
};