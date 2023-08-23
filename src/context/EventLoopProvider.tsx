import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import { listen, UnlistenFn } from "@tauri-apps/api/event";

type EventCallback = (payload: any) => void;

interface EventLoopContextInterface {
  subscribeToEvent: (eventName: string, callback: EventCallback) => void;
}

export const EventLoopContext = createContext<EventLoopContextInterface>({
  subscribeToEvent: () => {},
});

export const useEventLoopContext = () => useContext(EventLoopContext);

export const EventLoopProvider = ({ children }: { children: ReactNode }) => {
  const listeners: UnlistenFn[] = [];

  const subscribeToEvent = (event_name: string, callBack: EventCallback) => {
    const unlistenPromise = listen(event_name, (event: any) => {
      // console.log("EventLoopProvider: current_task event received");
      console.log(
        "Listened to event for " + event_name + " -> " + event.payload
      );
      // setCurrentTask(event.payload);
      callBack(event.payload);
    });

    // Resolve the promise and push the unlisten function to the listeners array
    unlistenPromise.then((unlisten) => {
      listeners.push(unlisten);
    });
  };

  useEffect(() => {
    // Clean up listeners when component unmounts
    return () => {
      listeners.forEach((unlisten) => unlisten());
    };
  }, []);

  return (
    <EventLoopContext.Provider value={{ subscribeToEvent }}>
      {children}
    </EventLoopContext.Provider>
  );
};
