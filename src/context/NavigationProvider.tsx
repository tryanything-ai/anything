import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

interface NavigationContextInterface {}

export const NavigationContext = createContext<NavigationContextInterface>({});

export const useNavigationContext = () => useContext(NavigationContext);

//TODO: keyboard shortcuts
export const NavigationProvider = ({ children }: { children: ReactNode }) => {
  return (
    <NavigationContext.Provider value={{}}>
      {children}
    </NavigationContext.Provider>
  );
};
