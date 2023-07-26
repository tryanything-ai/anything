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

interface LocalFileContextInterface {
  flowPaths: FileEntry[];
}

export const LocalFileContext = createContext<LocalFileContextInterface>({
  flowPaths: [],
});

export const useLocalFileContext = () => useContext(LocalFileContext);

export const LocalFileProvider = ({ children }: { children: ReactNode }) => {
  const { appDocuments, loading } = useTauriContext();
  const [flowPaths, setFlowPaths] = useState<FileEntry[]>([]);

  const getLocalFiles = async () => {
    try {
      if (appDocuments !== undefined) {
        let entries = await readDir(appDocuments + "/flows", {
          recursive: true,
        });
        // filter out .DS_Store files
        entries = entries.filter((entry) => !entry.path.endsWith(".DS_Store"));

        console.log("entries", entries);
        //TODO: check for properly formed file structure
        setFlowPaths(entries);
      } else {
        console.log("appDocuments is undefined still");
      }
    } catch (error) {
      console.error(error);
    }
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
            // console.log("toml file changed, sniffed in file watcher");
            // readToml(); //TODO: do this in a less chatty way
            getLocalFiles();
          }
          // { recursive: true }
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
      }}
    >
      {children}
    </LocalFileContext.Provider>
  );
};
