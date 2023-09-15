import { invoke } from "@tauri-apps/api/tauri";

//Models
const startModel = async (args: any) => {
  return await invoke("start", args);
};

const downloadModel = async (args: any) => {
  return await invoke("plugin:local_models|download_model", args);
};

const callModel = async (args: any) => {
  return await true;
  //TODO: remove kinda a stub for testing running models
  //  return await await invoke("plugin:local_models|call_model", args);
};

const getPromptTemplates = async <T>(): Promise<T> => {
  return await invoke("get_prompt_templates");
};

const getModels = async () => {
  return await invoke("get_models")
}

const getArchitectures = async () => {
  return await invoke("get_architectures"); 
}

const getDownloadedModels = async () => {
  return await invoke("get_downloaded_models")
}

const loadSqlLite = async () => {
  await invoke("plugin:sqlite|load");
};

const executeSqlLite = async (args: any) => {
  return await invoke("plugin:sqlite|execute", args);
}

const selectSqlLite = async (args: any) => {
  return await invoke("plugin:sqlite|select", args);
}

const sendPrompt = async (args: any) => {
  return await invoke("prompt", args);
}

const getFlowsWitChats = async () => {
  return await invoke("get_chat_flows")
}


const api = {
  startModel,
  downloadModel,
  callModel,
  getModels,
  getArchitectures,
  getDownloadedModels, 
  getPromptTemplates,
  loadSqlLite, 
  executeSqlLite, 
  selectSqlLite, 
  sendPrompt, 
  getFlowsWitChats
};

export default api;
