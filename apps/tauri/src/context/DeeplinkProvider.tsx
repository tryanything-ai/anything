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

  const listenFunc = async () => {
    console.log("Listening in listenFunc");

    await listen("click", (event) => {
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
      console.log("ListenFunc Received", JSON.stringify(event, null, 3));
    });

    // return unlisten;
  };

  //Listen for deep link message to navigate to template
  useEffect(() => {
    listenFunc();
    console.log("Listening to Deep Link");
    let unlisten = api.subscribeToEvent("deeplink", (event: any) => {
      console.log("Deep Link Listener Received", JSON.stringify(event, null, 3));
      const route = event.replace("anything://", "");
      navigate(route);
    });

    return () => {
      unlisten.then((unlisten) => unlisten());
    };
  }, []);
  return (
    <DeeplinkContext.Provider value={{}}>{children}</DeeplinkContext.Provider>
  );
};
