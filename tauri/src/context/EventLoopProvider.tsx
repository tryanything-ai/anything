import { createContext, useContext, ReactNode } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

type EventCallback = (payload: any) => void;

interface EventLoopContextInterface {
  subscribeToEvent: (
    eventName: string,
    callback: EventCallback
  ) => Promise<UnlistenFn>;
}

export const EventLoopContext = createContext<EventLoopContextInterface>({
  subscribeToEvent: () => Promise.resolve(() => {}),
});

export const useEventLoopContext = () => useContext(EventLoopContext);

export const EventLoopProvider = ({ children }: { children: ReactNode }) => {
  //TODO: this pattern is kinda not great. maybe not even helpful. maybe remove context completely.
  const subscribeToEvent = (event_name: string, callBack: EventCallback) => {
    const unlistenPromise = listen(event_name, (event: any) => {
      // console.log(
      //   "Listened to event for " +
      //     event_name +
      //     " -> " +
      //     JSON.stringify(event.payload)
      // );
      callBack(event.payload);
    });

    return unlistenPromise;
  };

  return (
    <EventLoopContext.Provider value={{ subscribeToEvent }}>
      {children}
    </EventLoopContext.Provider>
  );
};
