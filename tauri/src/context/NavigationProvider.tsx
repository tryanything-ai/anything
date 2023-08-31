import { createContext, useState, useContext, ReactNode } from "react";

interface NavigationContextInterface {
  nodePanel: boolean;
  setNodePanel: (option: boolean) => void;
  tomlPanel: boolean;
  setTomlPanel: (option: boolean) => void;
  debugPanel: boolean;
  setDebugPanel: (option: boolean) => void;
  settingsPanel: boolean;
  setSettingsPanel: (option: boolean) => void;
}

export const NavigationContext = createContext<NavigationContextInterface>({
  nodePanel: true,
  setNodePanel: () => {},
  tomlPanel: true,
  setTomlPanel: () => {},
  debugPanel: true,
  setDebugPanel: () => {},
  settingsPanel: true,
  setSettingsPanel: () => {},
});

export const useNavigationContext = () => useContext(NavigationContext);

//TODO: keyboard shortcuts
export const NavigationProvider = ({ children }: { children: ReactNode }) => {
  const [nodePanel, setNodePanel] = useState<boolean>(false);
  const [tomlPanel, setTomlPanel] = useState<boolean>(false);
  const [debugPanel, setDebugPanel] = useState<boolean>(false);
  const [settingsPanel, setSettingsPanel] = useState<boolean>(false);

  return (
    <NavigationContext.Provider
      value={{
        nodePanel,
        setNodePanel,
        tomlPanel,
        setTomlPanel,
        debugPanel,
        setDebugPanel, 
        settingsPanel,
        setSettingsPanel,
      }}
    >
      {children}
    </NavigationContext.Provider>
  );
};
