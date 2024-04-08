import { invoke } from "@tauri-apps/api";
import { UpdateFlowArgs } from "../../../../core/crates/tauri-plugin-anything-tauri/webview-src";
import { anything } from "./anythingInit";
import { Flow } from "../utils/flowTypes";
export type { UpdateFlowArgs } from "../../../../core/crates/tauri-plugin-anything-tauri/webview-src";

export const getFlows = async () => {
  let res = await anything.getFlows();
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

export async function updateFlowVersion(flowId: string, flow: Flow) {
  console.log(`updateFlowVersion called for flow_id: ${flowId}}`, flow);
  return await anything.updateFlowVersion(flowId, flow.version, {
    version: flow.version,
    flow_definition: JSON.stringify(flow),
    published: false,
    description: flow.description,
  });
}

export async function deleteFlow(flowId: string) {
  return await anything.deleteFlow(flowId);
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

export const executeFlow = async (flowId: string, flowVersionId: string, sessionId?: string, stage?: string): Promise<string> => {
  // console.log(`executeFlow called with flowId in tauri_api: ${flowId}, flowVersionId: ${flowVersionId}, sessionId: ${sessionId}, stage: ${stage}`);
  return await anything.executeFlow(flowId, flowVersionId, sessionId, stage);
};

export const fetchSessionEvents = async (sessionId: string) => {
  return await anything.fetchSessionEvents(sessionId);
};

export const getEvent = async (eventId: string) => {
  return await anything.getEvent(eventId);
}


export const stopExecution = async () => {
  return await anything.stop();
};
// export const readToml = async (flow_id: string) => {
//   return "";
//   //TODO: debracated for now
//   // return await anything.readToml(flowId);
//   // return await invoke("read_toml", { flow_id });
// };

// export const writeToml = async (flowId: string, toml: string) => {
//   return true;
//   //TODO: debrected for now
//   // return await anything.writeTomle(flowId, toml);
// };
