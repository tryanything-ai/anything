import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

interface NavigationContextInterface {
  sidePanel: boolean;
  setSidePanel: (option: boolean) => void;
}

export const NavigationContext = createContext<NavigationContextInterface>({
  sidePanel: true,
  setSidePanel: () => {},
});

export const useNavigationContext = () => useContext(NavigationContext);

//TODO: keyboard shortcuts
export const NavigationProvider = ({ children }: { children: ReactNode }) => {
  const [sidePanel, setSidePanel] = useState<boolean>(false);

  return (
    <NavigationContext.Provider value={{ sidePanel, setSidePanel }}>
      {children}
    </NavigationContext.Provider>
  );
};
