import { invoke } from "@tauri-apps/api";
import { UpdateFlowArgs } from "tauri-plugin-anything-tauri/webview-src";
import { anything } from "./anything";

export const getFlows = async () => {
  let res = await anything.getFlows();
  console.log(`Got back from getFlows ${JSON.stringify(res)}`);
  return res;
};

export const createFlow = async (flowName: string) => {
  console.log(`Called createFlow with ${flowName}`);
  let res = await anything.createFlow(flowName);
  console.log(`Got back from createFlow ${JSON.stringify(res)}`);
  return res;
};

export async function updateFlow(flowId: string, args: UpdateFlowArgs) {
  return await anything.updateFlow(flowId, args);
}

export async function deleteFlow(flowId: string) {
  return true; //TODO:
  // return await anything.deleteFlow(flowId);
}

export const getFlow = async (flowId: string) => {
  return await invoke("get_flow", { flowId });
};

export const getFlowByName = async <T>(flowName: string): Promise<T> => {
  return await anything.getFlowByName(flowName);
};

export const getFlowVersions = async (flowId: string) => {
  return await invoke("get_flow_versions", { flowId });
};

export const readToml = async (flow_id: string) => {
  return "";
  //TODO:
  // return await anything.readToml(flowId);
  // return await invoke("read_toml", { flow_id });
};

export const writeToml = async (flowId: string, toml: string) => {
  return true;
  //TODO:
  // return await anything.writeTomle(flowId, toml);
};

export const readNodeConfig = async (flowId: string, nodeId: string) => {
  return true;
  //TODO:
  // return await anything.readNodeConfig(flowId, nodeId);
};

export const writeNodeConfig = async (
  flowId: string,
  nodeId: string,
  config: string
) => {
  return true;
  //TODO:
  // return await anything.wrtieNodeConfig(flowId, nodeId, config);
};
