import {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import { useTauriContext } from "./TauriProvider";
import { watchImmediate } from "tauri-plugin-fs-watch-api";
import {
  readDir,
  writeTextFile,
  FileEntry,
  createDir,
  removeDir
} from "@tauri-apps/api/fs";

interface LocalFileContextInterface {
  flowPaths: FileEntry[];
  createNewFlow: () => void;
  deleteFlow: (flowName: string) => void;
}

export const LocalFileContext = createContext<LocalFileContextInterface>({
  flowPaths: [],
  createNewFlow: () => { },
  deleteFlow: () => { },
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

  const createNewFlow = async () => {
    try {

      let flowName = "Flow" + " " + flowPaths.length; 
      //TODO: use from template of basic flow
      if (appDocuments !== undefined) {
        await createDir(appDocuments + "/flows/" + flowName, { recursive: true });

        await writeTextFile(
          appDocuments + "/flows/" + flowName + "/flow.toml",
          ""
        );
        await writeTextFile(
          appDocuments + "/flows/" + flowName + "/settings.toml",
          ""
        );

        // get local files for ui again
        await getLocalFiles();
      }
    } catch (error) {
      console.error(error);
    }
  };

  const deleteFlow = async (flowName: string) => {
    //TODO: deal with situation where there are flow events in the db
    try {
      if (appDocuments !== undefined) {
        await removeDir(appDocuments + "/flows/" + flowName, { recursive: true });
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
        createNewFlow,
        deleteFlow
      }}
    >
      {children}
    </LocalFileContext.Provider>
  );
};
