import { createContext, useContext, ReactNode } from "react";

import { useSettingsContext } from "./SettingsProvider";

interface WebFeaturesContextInterface {
  searchTemplates: (searchTerm: string) => void;
}

export const WebFeaturesContext = createContext<WebFeaturesContextInterface>({
  searchTemplates: (searchTerm) => {},
});

export const useWebFeaturesContext = () => useContext(WebFeaturesContext);

export const WebFeaturesProvider = ({ children }: { children: ReactNode }) => {
  const { webFeaturesDisabled } = useSettingsContext();
  //fetch Supabaes Flow Templates
  //fetch Supabase Action Templates
  //fetch Supabase Trigger Templates

  //expose Supabase Search for Flows, Actions, Triggers, Templates, etc

  const searchTemplates = (searchTerm: string) => {
    if (webFeaturesDisabled) return [];

    //Do supabase stuff.
  };

  return (
    <WebFeaturesContext.Provider value={{ searchTemplates }}>
      {children}
    </WebFeaturesContext.Provider>
  );
};
