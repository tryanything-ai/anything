import React, { createContext, useState, useContext, ReactNode } from "react";

interface NavigationContextInterface {
  nodePanel: boolean;
  setNodePanel: (option: boolean) => void;
  tomlPanel: boolean;
  setTomlPanel: (option: boolean) => void;
  chatPanel: boolean;
  setChatPanel: (option: boolean) => void;
}

export const NavigationContext = createContext<NavigationContextInterface>({
  nodePanel: true,
  setNodePanel: () => {},
  tomlPanel: true,
  setTomlPanel: () => {},
  chatPanel: true,
  setChatPanel: () => {},
});

export const useNavigationContext = () => useContext(NavigationContext);

//TODO: keyboard shortcuts
export const NavigationProvider = ({ children }: { children: ReactNode }) => {
  const [nodePanel, setNodePanel] = useState<boolean>(false);
  const [tomlPanel, setTomlPanel] = useState<boolean>(false);
  const [chatPanel, setChatPanel] = useState<boolean>(false);

  return (
    <NavigationContext.Provider
      value={{
        nodePanel,
        setNodePanel,
        tomlPanel,
        setTomlPanel,
        chatPanel,
        setChatPanel,
      }}
    >
      {children}
    </NavigationContext.Provider>
  );
};
