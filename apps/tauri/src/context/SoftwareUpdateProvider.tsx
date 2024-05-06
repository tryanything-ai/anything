import React, {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";

import {
  checkUpdate,
  installUpdate,
  onUpdaterEvent,
} from "@tauri-apps/api/updater";
import { relaunch } from "@tauri-apps/api/process";

//Docs for Tauri updater
//https://tauri.app/v1/guides/distribution/updater/#javascript

interface SettingsContextInterface {
  shouldUpdate: boolean;
  manifest: any;
  installUpdate: () => void;
}

export const SoftwareUpdateContext = createContext<SettingsContextInterface>({
  shouldUpdate: false,
  manifest: null,
  installUpdate: () => {},
});

export const useSoftwareUpdateContext = () => useContext(SoftwareUpdateContext);

//TODO: its an antipattern to use local storage here. It should also be in Toml Somewhere
//we also want a way to these settings to effect functions in rust probably
export const SoftwareUpdateProvider = ({
  children,
}: {
  children: ReactNode;
}) => {
  const [shouldUpdate, setShouldUpdate] = useState(false);
  const [manifest, setManifest] = useState<any>(null);

  const checkForUpdates = async () => {
    const { shouldUpdate, manifest } = await checkUpdate();
    setShouldUpdate(shouldUpdate);
    setManifest(manifest);
  };

  const startListening = async () => {
    const unlisten = await onUpdaterEvent(({ error, status }) => {
      // This will log all updater events, including status updates and errors.
      console.log("Updater event", error, status);
    });
    return unlisten;
  };

  const _installUpdate = async () => {
    try {
      const { shouldUpdate, manifest } = await checkUpdate();

      if (shouldUpdate) {
        // You could show a dialog asking the user if they want to install the update here.
        console.log(
          `Installing update ${manifest?.version}, ${manifest?.date}, ${manifest?.body}`
        );

        // Install the update. This will also restart the app on Windows!
        await installUpdate();

        // On macOS and Linux you will need to restart the app manually.
        // You could use this step to display another confirmation dialog.
        await relaunch();
      }
    } catch (error) {
      console.error(error);
    }
  };

  useEffect(() => {
    let unlistenPromise = startListening();
    checkForUpdates();

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <SoftwareUpdateContext.Provider
      value={{ shouldUpdate, installUpdate: _installUpdate, manifest }}
    >
      {children}
    </SoftwareUpdateContext.Provider>
  );
};
