import { parse, stringify } from "iarna-toml-esm";
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import { v4 as uuidv4 } from "uuid";

import api from "../tauri_api/api";
import { Rust_Flow } from "../tauri_api/types";
import { useTauriContext } from "./TauriProvider";

interface LocalFileContextInterface {
  flows: Rust_Flow[];
  createNewFlow: () => void;
  deleteFlow: (flowName: string) => void;
  renameFlowFiles: (flowName: string, newFlowName: string) => void;
  readNodeConfig: (nodeId: string, flow_name: string) => void;
  writeNodeConfig: (nodeId: string, flowName: string, data: any) => void;
}

export const LocalFileContext = createContext<LocalFileContextInterface>({
  flows: [],
  createNewFlow: () => {},
  deleteFlow: () => {},
  renameFlowFiles: () => {},
  readNodeConfig: () => {},
  writeNodeConfig: () => {},
});

export const useLocalFileContext = () => useContext(LocalFileContext);

export const LocalFileProvider = ({ children }: { children: ReactNode }) => {
  const { loading } = useTauriContext();
  // const [flowPaths, setFlowPaths] = useState<FileEntry[]>([]);
  const [flows, setFlows] = useState<Rust_Flow[]>([]);

  // const getLocalFiles = async () => {
  //   try {
  //     if (appDocuments !== undefined) {
  //       let entries = await api.fs.readDir(appDocuments + "/flows", {
  //         recursive: true,
  //       });
  //       // filter out .DS_Store files
  //       entries = entries.filter((entry) => !entry.path.endsWith(".DS_Store"));

  //       // console.log("entries", entries);
  //       //TODO: check for properly formed file structure
  //       setFlowPaths(entries);
  //     } else {
  //       console.log("appDocuments is undefined still");
  //     }
  //   } catch (error) {
  //     console.error(error);
  //   }
  // };

  //BUG: there is a bug where when you add new flows the names colide because we write files as names.
  //TODO: more sophisticated way of determining new flow name
  const createNewFlow = async () => {
    try {
      let flow_name = "Flow" + " " + (flows.length + 1);
      console.log("new flow name", flow_name);
      let flow_id = uuidv4();

      await api.createFlow({ flow_name, flow_id });
      //   // Basic TOML structure for the flow.toml file
      //   const flowTomlContent = `
      // [flow]
      // name = "${flowName}"
      // id =  "${flowId}"
      // version = "0.0.1"
      // author = "Your Name <your.email@example.com>"
      // description = "Description of your flow"
      // `;
      //   // Basic TOML structure for the settings.toml file (modify as needed)
      //   const settingsTomlContent = `
      // [settings]
      // some_key = "some_value"
      // `;
      //   if (appDocuments !== undefined) {
      //     await api.fs.createDir(appDocuments + "/flows/" + flowName, {
      //       recursive: true,
      //     });
      //     await api.fs.writeTextFile(
      //       appDocuments + "/flows/" + flowName + "/flow.toml",
      //       flowTomlContent
      //     );
      //     await api.fs.writeTextFile(
      //       settingsTomlContent
      //     );
      //     // get local files for ui again
      //     await getLocalFiles();
      // }
    } catch (error) {
      console.error(error);
    } finally {
      getFlows();
    }
  };

  const deleteFlow = async (flowName: string) => {
    //TODO: deal with situation where there are flow events in the db
    try {
      // if (appDocuments !== undefined) {
      //   await api.fs.removeDir(appDocuments + "/flows/" + flowName, {
      //     recursive: true,
      //   });
      //   // get local files for ui again
      //   await getLocalFiles();
      // }
    } catch (error) {
      console.error(error);
    }
  };

  // const allPathsExist = async (paths: string[]) => {
  //   const results = await Promise.all(paths.map((path) => api.fs.exists(path)));
  //   return results.every((result) => result);
  // };

  const readToml = async (flow_name: string) => {
    try {
      // if (!appDocuments || !flow_name) {
      //   throw new Error("appDocuments or flow_name is undefined");
      // }
      // console.log("reading toml in FlowProvider");
      // return await api.fs.readTextFile(
      //   appDocuments + "/flows/" + flow_name + "/flow.toml"
      // );
      return "";
    } catch (error) {
      console.log("error reading toml in FlowProvider", error);
    }
  };

  //TODO: RUST_MIGRATION
  const readNodeConfig = async (nodeId: string, flowName: string) => {
    try {
      if (!flowName) {
        throw new Error("appDocuments or flowName is undefined");
      }
      console.log("reading node config in FlowProvider");
      let new_toml = await readToml(flowName);
      if (!new_toml) throw new Error("new_toml is undefined");
      let parsedToml = parse(new_toml);
      if (!parsedToml.nodes) throw new Error("parsedToml.nodes is undefined");

      if (!parsedToml.nodes) {
        parsedToml.nodes = [];
      }
      let nodes = parsedToml.nodes as any;
      if (nodes.length === 0) throw new Error("nodes is empty");
      let node = nodes.find((node: any) => node.id === nodeId);
      if (!node) throw new Error("node is undefined");
      return node;
    } catch (error) {
      console.log("error reading node config in FlowProvider", error);
    }
  };

  const writeNodeConfig = async (
    nodeId: string,
    flowName: string,
    data: any
  ) => {
    try {
      if (!flowName) {
        throw new Error("appDocuments or flowName is undefined");
      }
      console.log("writing node config in FlowProvider");
      let new_toml = await readToml(flowName);

      if (!new_toml) throw new Error("new_toml is undefined");
      let parsedToml = parse(new_toml);
      if (!parsedToml.nodes) throw new Error("parsedToml.nodes is undefined");
      let nodes = parsedToml.nodes as any;
      if (nodes.length === 0) throw new Error("nodes is empty");
      let node = nodes.find((node: any) => node.id === nodeId);
      if (!node) throw new Error("node is undefined");
      node.data = data;
      // let newToml = stringify(parsedToml);
      // await api.fs.writeTextFile(
      //   appDocuments + "/flows/" + flowName + "/flow.toml",
      //   newToml
      // );
    } catch (error) {
      console.log("error writing node config in FlowProvider", error);
    }
  };

  const renameFlowFiles = async (flowName: string, newFlowName: string) => {
    // console.log("renameFlowFiles", flowName, newFlowName);
    try {
      //   // if(true) throw Error("Not implemented yet");
      //   if (appDocuments === undefined) throw Error("AppDocuments Undefiend");
      //   if (flowName === newFlowName) throw Error("Flow names are the same");
      //   let allExist = await allPathsExist([
      //     appDocuments + "/flows/" + flowName + "/flow.toml",
      //     appDocuments + "/flows/" + flowName + "/settings.toml",
      //   ]);
      //   if (!allExist) throw Error("Flow files do not all exist");
      //   //make new dir first
      //   await api.fs.createDir(appDocuments + "/flows/" + newFlowName, {
      //     recursive: true,
      //   });
      //   //copy files over
      //   await api.fs.copyFile(
      //     appDocuments + "/flows/" + flowName + "/flow.toml",
      //     appDocuments + "/flows/" + newFlowName + "/flow.toml"
      //   );
      //   await api.fs.copyFile(
      //     appDocuments + "/flows/" + flowName + "/settings.toml",
      //     appDocuments + "/flows/" + newFlowName + "/settings.toml"
      //   );
      //   //delete old dir
      //   await api.fs.removeDir(appDocuments + "/flows/" + flowName, {
      //     recursive: true,
      //   });
      //   //TODO: update file frontmatter
      //   await getLocalFiles();
    } catch (error) {
      console.error("Error renaming flow" + error);
    }
  };
  //get local files to show in UI when files change
  //read the exact toml file that is being editedf
  //TODO: make this less brute force
  useEffect(() => {
    // Your watch function
    if (!loading) {
      // let stopWatching = () => {};
      // console.log("Watching ", appDocuments, " for changes");
      // const watchThisFile = async () => {
      //   stopWatching = await api.watch.watchImmediate(
      //     appDocuments,
      //     (event) => {
      //       console.log("File changed: ", JSON.stringify(event, null, 3));
      //       // console.log("toml file changed, sniffed in file watcher");
      //       // readToml(); //TODO: do this in a less chatty way
      //       getLocalFiles();
      //     }
      //     // { recursive: true }
      //   );
      // };
      // watchThisFile();
      // // Cleanup function
      // return () => {
      //   stopWatching(); // Call the stopWatching function to kill the watch
      // };
    }
  }, [loading]);

  //RUST_MIGRATION
  const getFlows = async () => {
    console.log("Getting FLows from Tauri API?");
    let res: any = await api.getFlows();
    setFlows(res);

    console.log("res from new rust stub", res);
  };

  useEffect(() => {
    getFlows();
  }, []);

  // useEffect(() => {
  // if (!loading) {
  //   getLocalFiles();
  // }
  // }, [loading]);

  return (
    <LocalFileContext.Provider
      value={{
        flows,
        // flowPaths,
        createNewFlow,
        deleteFlow,
        renameFlowFiles,
        readNodeConfig,
        writeNodeConfig,
      }}
    >
      {children}
    </LocalFileContext.Provider>
  );
};
