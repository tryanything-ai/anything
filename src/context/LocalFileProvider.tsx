import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import { useTauriContext } from "./TauriProvider";
import { watch } from "tauri-plugin-fs-watch-api";
import { readDir, readTextFile, FileEntry } from "@tauri-apps/api/fs";

//TODO: create demo flow that is loaded on first run

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
        const entries = await readDir(appDocuments + "/flows", {
          recursive: true,
        });

        setFlowPaths(entries);
        //open the first one and set the filePath I think
        // setFilePath(entries[0].path);
        //TODO: hydrate the state of theh file tree into a react component
      } else {
        console.log("appDocuments is undefined still");
      }
    } catch (error) {
      console.error(error);
    }
  };
  useEffect(() => {
    // Your watch function
    if (!loading) {
      let stopWatching = () => {};
      console.log("Wathcing ", appDocuments, " for changes");
      const watchThisFile = async () => {
        stopWatching = await watch(
          appDocuments,
          (event) => {
            const { kind, path } = event;
            //TODO: filter out .DS_Store files from mac.
            // Handle event logic here
            console.log("File changed: ", JSON.stringify(event, null, 3));
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
    <LocalFileContext.Provider value={{ flowPaths }}>
      {children}
    </LocalFileContext.Provider>
  );
};
