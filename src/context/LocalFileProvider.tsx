import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import { useTauriContext } from "./TauriProvider";
import { watch, watchImmediate } from "tauri-plugin-fs-watch-api";
import {
  readDir,
  readTextFile,
  writeTextFile,
  FileEntry,
} from "@tauri-apps/api/fs";

//TODO: create demo flow that is loaded on first run

interface LocalFileContextInterface {
  flowPaths: FileEntry[];
  toml: string;
  writeToml: (toml: string) => void;
  currentFlow: string;
  setCurrentFlow: (flowName: string) => void;
}

export const LocalFileContext = createContext<LocalFileContextInterface>({
  flowPaths: [],
  toml: "",
  writeToml: () => {},
  currentFlow: "",
  setCurrentFlow: () => {},
});

export const useLocalFileContext = () => useContext(LocalFileContext);

export const LocalFileProvider = ({ children }: { children: ReactNode }) => {
  const { appDocuments, loading } = useTauriContext();
  const [flowPaths, setFlowPaths] = useState<FileEntry[]>([]);
  const [toml, setToml] = useState<string>("");
  const [currentFlow, setCurrentFlow] = useState<string>("");

  const setCurrentFlowLocal = async (flowName: string) => {
    console.log("setting current flow in localFileProvider -> ", flowName);
    setCurrentFlow(flowName);
    readToml();
  };

  const readToml = async () => {
    let content = await readTextFile(
      appDocuments + "/flows/" + currentFlow + "/flow.toml"
    );
    setToml(content);
  };

  const getLocalFiles = async () => {
    try {
      if (appDocuments !== undefined) {
        let entries = await readDir(appDocuments + "/flows", {
          recursive: true,
        });
        // filter out .DS_Store files
        entries = entries.filter((entry) => !entry.path.endsWith(".DS_Store"));

        console.log("entries", entries);
        setFlowPaths(entries);
      } else {
        console.log("appDocuments is undefined still");
      }
    } catch (error) {
      console.error(error);
    }
  };

  const writeToml = async (toml: string) => {
    console.log("writing toml to", currentFlow);

    await writeTextFile(
      appDocuments + "/flows/" + currentFlow + "/flow.toml",
      toml
    );
  };

  //get local files to show in UI when files change
  //read the exact toml file that is being editedf
  //TODO: make this less brute force
  useEffect(() => {
    // Your watch function
    if (!loading) {
      let stopWatching = () => {};
      console.log("Wathcing ", appDocuments, " for changes");
      const watchThisFile = async () => {
        stopWatching = await watchImmediate(
          appDocuments,
          (event) => {
            console.log("File changed: ", JSON.stringify(event, null, 3));
            console.log("toml file changed, sniffed in file watcher");
            readToml();
            getLocalFiles();
          },
          { recursive: true }
        );
      };

      watchThisFile();

      // Cleanup function
      return () => {
        stopWatching(); // Call the stopWatching function to kill the watch
      };
    }
  }, [loading]);

  useEffect(() => {
    if (!loading) {
      getLocalFiles();
    }
  }, [loading]);

  return (
    <LocalFileContext.Provider
      value={{
        flowPaths,
        toml,
        writeToml,
        currentFlow,
        setCurrentFlow: setCurrentFlowLocal,
      }}
    >
      {children}
    </LocalFileContext.Provider>
  );
};
