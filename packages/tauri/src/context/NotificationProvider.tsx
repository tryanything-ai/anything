import React, {
    createContext,
    ReactNode,
    useContext,
    useEffect,
    useState,
  } from "react";
  
  interface NotificationsContextInterface {
    // theme: string;
    // setTheme: (theme: string) => void;
    // webFeaturesDisabled: boolean;
    // setWebFeaturesDisabled: (webFeaturesDisabled: boolean) => void;
  }
  
  export const NotificationsContext = createContext<NotificationsContextInterface>({
    // theme: localStorage.getItem("theme") || "dark",
    // setTheme: () => {},
    // webFeaturesDisabled: false,
    // setWebFeaturesDisabled: () => {},
  });
  
export const useNotificationsContext = () => useContext(NotificationsContext);

  export const NotificationsProvider = ({ children }: { children: ReactNode }) => {
  
    return (
      <NotificationsContext.Provider
        value={{ }}
      >
        {children}
      </NotificationsContext.Provider>
    );
  };
  