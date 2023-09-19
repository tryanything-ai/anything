import { invoke } from "@tauri-apps/api/tauri";

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

export const executeSqlLite = async (args: any) => {
  return await invoke("plugin:sqlite|execute", args);
};

export const selectSqlLite = async (args: any) => {
  return await invoke("plugin:sqlite|select", args);
};

// export const sendPrompt = async (args: any) => {
//   return await invoke("prompt", args);
// };

// export const getFlowsWitChats = async () => {
//   return await invoke("get_chat_flows");
// };
