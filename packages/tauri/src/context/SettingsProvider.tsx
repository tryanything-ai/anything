import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

interface SettingsContextInterface {
  theme: string;
  setTheme: (theme: string) => void;
  webFeaturesDisabled: boolean;
  setWebFeaturesDisabled: (webFeaturesDisabled: boolean) => void;
}

export const SettingsContext = createContext<SettingsContextInterface>({
  theme: localStorage.getItem("theme") || "dark",
  setTheme: () => {},
  webFeaturesDisabled: false,
  setWebFeaturesDisabled: () => {},
});

export const useSettingsContext = () => useContext(SettingsContext);

//TODO: its an antipattern to use local storage here. It should also be in Toml Somewhere
//we also want a way to these settings to effect functions in rust probably
export const SettingsProvider = ({ children }: { children: ReactNode }) => {
  const [theme, setTheme] = useState(localStorage.getItem("theme") || "dark");
  const [webFeaturesDisabled, setWebFeaturesDisabled] = useState(false);

  useEffect(() => {
    document.body.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }, [theme]);

  return (
    <SettingsContext.Provider
      value={{ theme, setTheme, webFeaturesDisabled, setWebFeaturesDisabled }}
    >
      {children}
    </SettingsContext.Provider>
  );
};
