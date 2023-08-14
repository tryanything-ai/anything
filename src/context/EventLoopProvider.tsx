import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

// import React, { createContext, useContext, useEffect } from 'react';
import { emit, listen, UnlistenFn } from '@tauri-apps/api/event';


interface EventLoopContextInterface {
  // listenToEvent: (eventName: string, callback: (payload: any) => void) => void;
  currentTask: any | null;
}

// TODO: This should be in Rust i beleive but I don't have time for that right now
export const EventLoopContext = createContext<EventLoopContextInterface>({
  // listenToEvent: () => {},
  currentTask: null,
});

export const useEventLoopContext = () => useContext(EventLoopContext);

export const EventLoopProvider = ({ children }: { children: ReactNode }) => {
  const [currentTask, setCurrentTask] = useState<any | null>(null);
  
  const listeners: UnlistenFn[] = [];

  const listenToEvent = () => {
    const unlistenPromise = listen('current_task', (event: any) => {
      console.log("EventLoopProvider: current_task event received"); 
      console.log(event.payload);
      setCurrentTask(event.payload);
    });

    // Resolve the promise and push the unlisten function to the listeners array
    unlistenPromise.then((unlisten) => {
      listeners.push(unlisten);
    });
  };

  useEffect(() => {
    listenToEvent();

    // Clean up listeners when component unmounts
    return () => {
      listeners.forEach((unlisten) => unlisten());
    };
  }, []);

  return (
    <EventLoopContext.Provider value={{currentTask  }}>
      {children}
    </EventLoopContext.Provider>
  );
};
