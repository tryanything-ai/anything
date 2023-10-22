import { listen } from "@tauri-apps/api/event";

export type { UnlistenFn } from "@tauri-apps/api/event";

export type EventCallback = (payload: any) => void;
export const subscribeToEvent = (
  event_name: string,
  callBack: EventCallback
) => {
  console.log("subscribing to event: ", event_name);
  const unlistenPromise = listen(event_name, (event: any) => {
    console.log("event received: ", event);
    callBack(event.payload);
  });
  return unlistenPromise;
};
