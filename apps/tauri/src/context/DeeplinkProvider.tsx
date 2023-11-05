import React, {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";

import api from "../tauri_api/api";
import { listen } from "@tauri-apps/api/event";
import { useNavigate } from "react-router-dom";

interface DeeplinkContextInterface {}

export const DeeplinkContext = createContext<DeeplinkContextInterface>({});

export const useDeeplinkContext = () => useContext(DeeplinkContext);

//TODO: its an antipattern to use local storage here. It should also be in Toml Somewhere
//we also want a way to these settings to effect functions in rust probably
export const DeeplinkProvider = ({ children }: { children: ReactNode }) => {
  const navigate = useNavigate();

  //Listen for deep link message to navigate to template
  useEffect(() => {
    console.log("Listening to Deep Link");
    let unlisten = api.subscribeToEvent("deeplink", (event: any) => {
      console.log(
        "Deep Link Listener Received",
        JSON.stringify(event, null, 3)
      );
      console.log("Link Received", event);

        let route = event.replace("anything://", "");
        navigate(route); 

      // manually catching update password flow
    //   if (route.includes("#access_token") && route.includes("type=recovery")) {
    //     let update_route = "/update-password" + route;
    //     console.log("Navigating to update-password route -> " + update_route);
    //     navigate(update_route);
    //   } else {
    //     console.log("Navigating to route", route);
    //     navigate(route);
    //   }
    });

    return () => {
      unlisten.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <DeeplinkContext.Provider value={{}}>{children}</DeeplinkContext.Provider>
  );
};
