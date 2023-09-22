import { invoke } from "@tauri-apps/api/tauri";

export const getFlows = async () => {
  console.log("Invoking Get FLows");
  return await invoke("get_flows");
};

export const getChatFlows = async () => {
  return await invoke("get_chat_flows");
};

export const getFlow = async (flow_id: string) => {
  return await invoke("get_flow", { flow_id });
};

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

export const createFlow = async ({
  flow_name,
  flow_id,
}: {
  flow_name: string;
  flow_id: string;
}) => {
  return await invoke("create_flow", { flow_name, flow_id });
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
