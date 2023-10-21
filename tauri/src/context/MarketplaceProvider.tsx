import { createContext, useContext, useEffect, ReactNode } from "react";
import { useSettingsContext } from "./SettingsProvider";
import { Flow } from "../utils/newNodes";
import { localDataDir } from "@tauri-apps/api/path";
import { supabase } from "../utils/initSupabase";
import api from "../tauri_api/api";
import { listen } from "@tauri-apps/api/event";

interface MarketplaceContextInterface {
  searchTemplates: (searchTerm: string) => void;
  fetchTemplates: () => Promise<Flow[]>;
  saveTemplate: (template: any) => void;
  updateTemplate: (template: any) => void;
  fetchTemplate: (
    author_username: string,
    template_name: string
  ) => Promise<any>;
}

export const MarketplaceContext = createContext<MarketplaceContextInterface>({
  searchTemplates: () => {},
  fetchTemplates: () => Promise.resolve([]),
  fetchTemplate: () => Promise.resolve({}),
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

    // let templates = flow_templates?.map((template) => {
    //   //TODO: this might be very naughty
    //   return template.flow_json as unknown as RustFlow;
    // });
    //TODO: rebuild with new types
    return [];
  };

  const fetchTemplate = async (
    author_username: string,
    template_name: string
  ) => {
    if (webFeaturesDisabled) return [];

    //Do supabase stuff.
    let { data, error } = await supabase
      .from("flow_templates")
      .select("*")
      .eq("author_username", author_username)
      .eq("template_name", template_name)
      .single();

    if (error) {
      console.log(error);
      return [];
    }

    return localDataDir;
  };

  const saveTemplate = (template: any) => {
    if (webFeaturesDisabled) return false;

    //Do supabase stuff.
  };

  const updateTemplate = (template: any) => {
    if (webFeaturesDisabled) return false;

    //Do supabase stuff.
  };
  const listenFunc = async () => {
    console.log("Listening in listenFunc");

    await listen("click", (event) => {
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
      console.log("ListenFunc Received", JSON.stringify(event, null, 3));
    });

    // return unlisten;
  };

  //Listen for deep link message to navigate to template
  useEffect(() => {
    listenFunc();
    console.log("Listening to Deep Link");
    let unlisten = api.subscribeToEvent("deeplink", (event: any) => {
      console.log("Deep Link Listener Fired", JSON.stringify(event, null, 3));
    });

    return () => {
      unlisten.then((unlisten) => unlisten());
      // unlisten2.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <MarketplaceContext.Provider
      value={{
        searchTemplates,
        fetchTemplates,
        saveTemplate,
        updateTemplate,
        fetchTemplate,
      }}
    >
      {children}
    </MarketplaceContext.Provider>
  );
};
