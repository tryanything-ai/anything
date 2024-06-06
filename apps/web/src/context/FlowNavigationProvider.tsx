import { createContext, ReactNode, useContext, useState } from "react";

interface FLowNavigationContextInterface {
  nodePanel: boolean;
  setNodePanel: (option: boolean) => void;
  tomlPanel: boolean;
  setTomlPanel: (option: boolean) => void;
  debugPanel: boolean;
  setDebugPanel: (option: boolean) => void;
  settingsPanel: boolean;
  setSettingsPanel: (option: boolean) => void;
  sharingPanel: boolean;
  setSharingPanel: (option: boolean) => void;
  nodeConfigPanel: boolean;
  setNodeConfigPanel: (option: boolean, node_id: string) => void;
  nodeId: string;
  closeAllPanelsOpenOne: (panelName: string, arg?: any) => void;
}

export const FlowNavigationContext =
  createContext<FLowNavigationContextInterface>({
    nodePanel: true,
    setNodePanel: () => {},
    tomlPanel: true,
    setTomlPanel: () => {},
    debugPanel: true,
    setDebugPanel: () => {},
    settingsPanel: true,
    setSettingsPanel: () => {},
    sharingPanel: true,
    setSharingPanel: () => {},
    nodeConfigPanel: true,
    setNodeConfigPanel: () => {},
    nodeId: "",
    closeAllPanelsOpenOne: () => {},
  });

export const useFlowNavigationContext = () => useContext(FlowNavigationContext);

//TODO: keyboard shortcuts
export const FlowNavigationProvider = ({
  children,
}: {
  children: ReactNode;
}) => {
  const [nodePanel, setNodePanel] = useState<boolean>(true);
  const [tomlPanel, setTomlPanel] = useState<boolean>(false);
  const [debugPanel, setDebugPanel] = useState<boolean>(true);
  const [settingsPanel, setSettingsPanel] = useState<boolean>(false);
  const [sharingPanel, setSharingPanel] = useState<boolean>(false);
  const [nodeConfigPanel, setNodeConfigPanel] = useState<boolean>(false);
  const [nodeId, setNodeId] = useState<string>("");

  const _setNodeConfigPanel = (option: boolean, node_id: string) => {
    setNodeConfigPanel(option);
    setNodeId(node_id);
  };

  const closeAllPanelsOpenOne = (panelName: string, arg?: any) => {
    setNodePanel(false);
    setTomlPanel(false);
    setDebugPanel(false);
    setSettingsPanel(false);
    setNodeConfigPanel(false);
    setSharingPanel(false);

    switch (panelName) {
      case "node":
        setNodePanel(true);
        break;
      case "toml":
        setTomlPanel(true);
        break;
      case "nodeConfig":
        _setNodeConfigPanel(true, arg);
        break;
      case "debug":
        setDebugPanel(true);
        break;
      case "settings":
        setSettingsPanel(true);
        break;
      case "sharing":
        setSharingPanel(true);
        break;
      default:
        break;
    }
  };

  return (
    <FlowNavigationContext.Provider
      value={{
        nodePanel,
        setNodePanel,
        tomlPanel,
        setTomlPanel,
        debugPanel,
        setDebugPanel,
        settingsPanel,
        setSettingsPanel,
        sharingPanel,
        setSharingPanel,
        nodeConfigPanel,
        setNodeConfigPanel: _setNodeConfigPanel,
        nodeId,
        closeAllPanelsOpenOne,
      }}
    >
      {children}
    </FlowNavigationContext.Provider>
  );
};
