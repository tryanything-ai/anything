import { invoke } from "@tauri-apps/api/tauri";
import { Anything } from '../../../../core/crates/tauri-plugin-anything-tauri/webview-src/index'

import { EventInput } from "./types";

const anything = new Anything("anything");

export const getFlows = async () => {
  return await anything.getFlows();
};

export const createFlow = async ({
  flowName,
  flowId,
}: {
  flowName: string;
  flowId: string;
}) => {
  console.log(`Called createFlow with ${flowId}, ${flowName}`)
  return await anything.createFlow(flowName, flowId);
};

export const getChatFlows = async () => {
  return await invoke("get_chat_flows");
};

export const getFlow = async (flow_id: string) => {
  return await invoke("get_flow", { flow_id });
};

export const getFlowByName = async (flow_name: string) => {
  return await invoke("get_flow_by_name", { flowName: flow_name });
};

export const getFlowVersions = async (flow_id: string) => {
  return await invoke("get_flow_versions", {flow_id}); 
}

// export const getPublishedFlowVersion = async (flow_id: string) => {
//   return await invoke("get_published_flow_version", { flow_id });
// };

// export const getFlowCurrentUnpublishedVersion = async (flow_id: string) => {
//   return await invoke("get_flow_current_unpublished_version", { flow_id });
// };

export const getNodes = async () => {
  return await invoke("get_nodes");
};

export const getFlowNode = async ({
  flow_id,
  node_id,
}: {
  flow_id: string;
  node_id: string;
}) => {
  return await invoke("get_flow_node", { flow_id, node_id });
};



//TODO: probs bad need to pick this somewhere
export const saveToml = async ({
  toml,
  flow_id,
}: {
  toml: string;
  flow_id: string;
}) => {
  return await invoke("save_toml", { toml, flow_id });
};

export const createEvent = async (eventInput: EventInput) => {
  return await invoke("create_event", eventInput);
};

//Models
// export const startModel = async (args: any) => {
//   return await invoke("start", args);
// };

// export const downloadModel = async (args: any) => {
//   return await invoke("plugin:local_models|download_model", args);
// };

// export const callModel = async (args: any) => {
//   return await true;
//   //TODO: remove kinda a stub for testing running models
//   //  return await await invoke("plugin:local_models|call_model", args);
// };

// export const getPromptTemplates = async <T>(): Promise<T> => {
//   return await invoke("get_prompt_templates");
// };

// export const getModels = async () => {
//   return await invoke("get_models");
// };

// export const getArchitectures = async () => {
//   return await invoke("get_architectures");
// };

// export const getDownloadedModels = async () => {
//   return await invoke("get_downloaded_models");
// };

// export const loadSqlLite = async () => {
//   await invoke("plugin:sqlite|load");
// };

// export const executeSqlLite = async (args: any) => {
//   return await invoke("plugin:sqlite|execute", args);
// };

// export const selectSqlLite = async (args: any) => {
//   return await invoke("plugin:sqlite|select", args);
// };

// export const sendPrompt = async (args: any) => {
//   return await invoke("prompt", args);
// };

// export const getFlowsWitChats = async () => {
//   return await invoke("get_chat_flows");
// };
