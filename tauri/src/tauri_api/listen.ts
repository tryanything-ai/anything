import { listen } from "@tauri-apps/api/event";
export type { UnlistenFn } from "@tauri-apps/api/event";

export type EventCallback = (payload: any) => void;
export const subscribeToEvent = (
  event_name: string,
  callBack: EventCallback
) => {
  const unlistenPromise = listen(event_name, (event: any) => {
    callBack(event.payload);
  });
  return unlistenPromise;
};
