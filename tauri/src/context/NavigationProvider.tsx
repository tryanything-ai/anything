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
  nodeConfigPanel: boolean;
  setNodeConfigPanel: (option: boolean, node_id: string) => void;
  nodeId: string;
}

export const NavigationContext = createContext<NavigationContextInterface>({
  nodePanel: true,
  setNodePanel: () => {},
  tomlPanel: true,
  setTomlPanel: () => {},
  debugPanel: true,
  setDebugPanel: () => {},
  settingsPanel: true,
  setSettingsPanel: () => { },
  nodeConfigPanel: true,
  setNodeConfigPanel: () => { },
  nodeId: "",
});

export const useNavigationContext = () => useContext(NavigationContext);

//TODO: keyboard shortcuts
export const NavigationProvider = ({ children }: { children: ReactNode }) => {
  const [nodePanel, setNodePanel] = useState<boolean>(false);
  const [tomlPanel, setTomlPanel] = useState<boolean>(false);
  const [debugPanel, setDebugPanel] = useState<boolean>(false);
  const [settingsPanel, setSettingsPanel] = useState<boolean>(false);
  const [nodeConfigPanel, setNodeConfigPanel] = useState<boolean>(false);
  const [nodeId, setNodeId] = useState<string>("");

  const _setNodeConfigPanel = (option: boolean, node_id: string) => {
    setNodeConfigPanel(option);
    setNodeId(node_id);
  }

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
        nodeConfigPanel,
        setNodeConfigPanel: _setNodeConfigPanel,
        nodeId,
      }}
    >
      {children}
    </NavigationContext.Provider>
  );
};
