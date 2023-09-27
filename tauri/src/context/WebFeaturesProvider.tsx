import { createContext, useContext, ReactNode } from "react";
import { useSettingsContext } from "./SettingsProvider";
import { createClient } from "@supabase/supabase-js";
import { Database, Json } from "../types/supabase.types";
import { RustFlow } from "../utils/flowConversion";

const supabaseUrl = import.meta.env.VITE_SUPABASE_URL;
const supabaseAnonKey = import.meta.env.VITE_SUPABASE_ANON_KEY;

//TODO: Perhaps move Supabase stuff to a serverless function so oss contributors don't need keys
export const supabase = createClient<Database>(supabaseUrl, supabaseAnonKey);

interface WebFeaturesContextInterface {
  searchTemplates: (searchTerm: string) => void;
  fetchTemplates: () => Promise<RustFlow[]>;
  saveTemplate: (template: any) => void;
  updaetTemplate: (template: any) => void;
}

export const WebFeaturesContext = createContext<WebFeaturesContextInterface>({
  searchTemplates: () => {},
  fetchTemplates: () => Promise.resolve([]),
  saveTemplate: () => {},
  updaetTemplate: () => {},
});

export const useWebFeaturesContext = () => useContext(WebFeaturesContext);

//We will break compatability of templates and will need to know what version of templates we are using.
//was used to create a template to manage compatability and conversion
const FLOW_TEMPLATES_VERSION = "0.0.1";

export const WebFeaturesProvider = ({ children }: { children: ReactNode }) => {
  const { webFeaturesDisabled } = useSettingsContext();
  //fetch Supabaes Flow Templates
  //expose Supabase Search for Flows, Actions, Triggers, Templates, etc

  const searchTemplates = (searchTerm: string) => {
    if (webFeaturesDisabled) return [];

    //Do supabase stuff.
  };

  const fetchTemplates = async () => {
    if (webFeaturesDisabled) return [];

    //Do supabase stuff.

    let { data: flow_templates, error } = await supabase
      .from("flow_templates")
      .select("*");

    if (error) {
      console.log(error);
      return [];
    }

    let templates = flow_templates?.map((template) => {
      //TODO: this might be very naughty
      return template.flow_json as unknown as RustFlow;
    });

    return templates || [];
  };

  const saveTemplate = (template: any) => {
    if (webFeaturesDisabled) return false;

    //Do supabase stuff.
  };

  const updaetTemplate = (template: any) => {
    if (webFeaturesDisabled) return false;

    //Do supabase stuff.
  };

  return (
    <WebFeaturesContext.Provider
      value={{ searchTemplates, fetchTemplates, saveTemplate, updaetTemplate }}
    >
      {children}
    </WebFeaturesContext.Provider>
  );
};
