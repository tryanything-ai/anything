import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import { useTauriContext } from "./TauriProvider";
import { watch } from "tauri-plugin-fs-watch-api";

interface LocalFileContextInterface {
  filePaths: string[];
}

export const LocalFileContext = createContext<LocalFileContextInterface>({
  filePaths: [],
});

export const useLocalFileContext = () => useContext(LocalFileContext);

export const LocalFileProvider = ({ children }: { children: ReactNode }) => {
  const { appDocuments, loading } = useTauriContext();
  const [filePaths, setFilePaths] = useState<string[]>([]);

  useEffect();
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

  return (
    <LocalFileContext.Provider value={{ filePaths }}>
      {children}
    </LocalFileContext.Provider>
  );
};
