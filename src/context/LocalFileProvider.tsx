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
  toml: string;
  setCurrentFlow: (flowName: string) => void;
}

export const LocalFileContext = createContext<LocalFileContextInterface>({
  flowPaths: [],
  toml: "",
  setCurrentFlow: () => {},
});

export const useLocalFileContext = () => useContext(LocalFileContext);

export const LocalFileProvider = ({ children }: { children: ReactNode }) => {
  const { appDocuments, loading } = useTauriContext();
  const [flowPaths, setFlowPaths] = useState<FileEntry[]>([]);
  const [toml, setToml] = useState<string>("");

  const setCurrentFlow = async (flowName: string) => {
    let content = await readTextFile(
      appDocuments + "/flows/" + flowName + "/flow.toml"
    );
    console.log("content", content);
    setToml(content);
  };

  const getLocalFiles = async () => {
    try {
      if (appDocuments !== undefined) {
        const entries = await readDir(appDocuments + "/flows", {
          recursive: true,
        });

        console.log("entries", entries);
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
    <LocalFileContext.Provider value={{ flowPaths, toml, setCurrentFlow }}>
      {children}
    </LocalFileContext.Provider>
  );
};
