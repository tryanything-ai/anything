import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";
// import {
//   BrowserRouter as Router,
//   Switch,
//   Route,
//   useHistory,
// } from "react-router-dom";

interface NavigationContextInterface {}

export const NavigationContext = createContext<NavigationContextInterface>({});

export const useNavigationContext = () => useContext(NavigationContext);

export const NavigationProvider = ({ children }: { children: ReactNode }) => {
  //   const history = useHistory();

  //   window.addEventListener("tauri:back", () => {
  //     history.goBack();
  //   });

  //   window.addEventListener("tauri:forward", () => {
  //     history.goForward();
  //   });

  return (
    <NavigationContext.Provider value={{}}>
      {children}
    </NavigationContext.Provider>
  );
};
