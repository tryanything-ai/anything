import { invoke } from "@tauri-apps/api";
import { UpdateFlow } from "tauri-plugin-anything-tauri/webview-src";
import { anything } from "./anything";

export async function updateFlow(flowId: string, updateFlow: UpdateFlow) {
    return await anything.updateFlow(flowId, updateFlow);
}

export async function createFlow(flowName: string) {
    return await anything.createFlow(flowName);
}