import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";

import api from "../tauri_api/api";
import { useTauriContext } from "./TauriProvider";
import { Flow } from "../utils/flowTypes";
import { UpdateFlowArgs } from "tauri-plugin-anything-tauri/webview-src";

interface FlowsContextInterface {
  flows: Flow[];
  createNewFlow: () => void;
  deleteFlow: (flowName: string) => void;
  updateFlow: (flowId: string, args: UpdateFlowArgs) => void;
  readNodeConfig: (nodeId: string, flow_name: string) => void;
  writeNodeConfig: (nodeId: string, flowName: string, data: any) => void;
}

export const FlowsContext = createContext<FlowsContextInterface>({
  flows: [],
  createNewFlow: () => {},
  deleteFlow: () => {},
  updateFlow: () => {},
  readNodeConfig: () => {},
  writeNodeConfig: () => {},
});

export const useFlowsContext = () => useContext(FlowsContext);

export const FlowsProvider = ({ children }: { children: ReactNode }) => {
  const { loading } = useTauriContext();
  const [flows, setFlows] = useState<Flow[]>([]);

  //BUG: there is a bug where when you add new flows the names colide because we write files as names.
  //To reproduce. Create 2 flows. they will be called "Flow 1 and Flow 2"
  //Delete Flow 1. Now Create a new Flow and it will try to create "Flow 2" again and fail.
  //TODO: more sophisticated way of determining new flow name
  const createNewFlow = async (): Promise<any> => {
    try {
      //TODO Move to DB to fix collision problem
      let flowName = "Flow" + " " + (flows.length + 1);
      console.log("Creating new Flow in FlowsProvider");
      await api.flows.createFlow(flowName);
    } catch (error) {
      console.log("error creating new flow in FlowsProvider", error);
      console.error(error);
    } finally {
      getFlows();
    }
  };

  const deleteFlow = async (flowName: string): Promise<any> => {
    //TODO: deal with situation where there are flow events in the db
    try {
      await api.flows.deleteFlow(flowName);
    } catch (error) {
      console.error(error);
    } finally {
      getFlows();
    }
  };

  const readToml = async (flowName: string): Promise<string> => {
    try {
      return await api.flows.readToml(flowName);
    } catch (error) {
      console.log("error reading toml in FlowProvider", error);
    }
  };

  const writeToml = async (
    flowName: string,
    toml: string
  ): Promise<boolean> => {
    try {
      return await api.flows.writeToml(flowName, toml);
    } catch (error) {
      console.log("error writing toml in FlowProvider", error);
    } finally {
      getFlows();
    }
  };

  //TODO: RUST_MIGRATION
  const readNodeConfig = async (
    nodeId: string,
    flowName: string
  ): Promise<boolean> => {
    try {
      return await api.flows.readNodeConfig(nodeId, flowName);
    } catch (error) {
      console.log("error reading node config in FlowProvider", error);
    }
  };

  const writeNodeConfig = async (
    nodeId: string,
    flowName: string,
    data: any
  ): Promise<boolean> => {
    try {
      return api.flows.writeNodeConfig(nodeId, flowName, data);
    } catch (error) {
      console.log("error writing node config in FlowProvider", error);
    }
  };

  const updateFlow = async (flowId: string, args: UpdateFlowArgs) => {
    //Update Flow
    let res = await api.flows.updateFlow(flowId, args);
    //Rehydrate Global State
    getFlows();
    //Return without waiting
    return res;
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

  const getFlows = async () => {
    console.log("Getting FLows from Tauri API?");
    let res: any = await api.flows.getFlows();
    setFlows(res.flows);
  };

  useEffect(() => {
    getFlows();
  }, []);

  return (
    <FlowsContext.Provider
      value={{
        flows,
        createNewFlow,
        deleteFlow,
        updateFlow,
        readNodeConfig,
        writeNodeConfig,
      }}
    >
      {children}
    </FlowsContext.Provider>
  );
};
