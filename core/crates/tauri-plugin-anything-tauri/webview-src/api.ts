import { CreateFlowVersion } from "./type";
import { invoke } from "@tauri-apps/api/tauri";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { UpdateFlowArgs } from "type";

export class Anything {
  path: string;

  constructor(path: string) {
    this.path = path;
  }

  async stop() {
    return await invoke("plugin:anything-tauri|stop", {});
  }

  async getFlows<T>(): Promise<T> {
    return await invoke("plugin:anything|get_flows", {
      path: this.path,
    });
  }

  async getFlowByName<T>(flowName: string): Promise<T> {
    return await invoke("plugin:anything|get_flow_by_name", {
      flowName,
    });
  }

  async createFlow<T>(flowName: string): Promise<T> {
    return await invoke("plugin:anything|create_flow", { flowName });
  }

  async CreateFlowVersion<T>(
    flowName: string,
    createFlowVersion: CreateFlowVersion
  ): Promise<T> {
    return await invoke("plugin:anything|create_flow_version", {
      flowName,
      createFlowVersion,
    });
  }

  async updateFlow<T>(flowId: string, args: UpdateFlowArgs): Promise<T> {
    console.log("updateFlow called ", flowId, args);
    return await invoke("plugin:anything|update_flow", { flowId, args });
  }

  async deleteFlow<T>(flowId: string): Promise<T> {
    return await invoke("plugin:anything|delete_flow", { flowId });
  }

  async updateFlowVersion<T>(
    flowId: string,
    flowVersionId: string,
    updateFlow: any
  ): Promise<T> {
    console.log(
      "updateFlowVersion called in tauri plugin api ",
      flowId,
      flowVersionId,
      updateFlow
    );
    return await invoke("plugin:anything|update_flow_version", {
      flowId,
      flowVersionId,
      updateFlow,
    });
  }

  async executeFlow<T>(flowId: string, flowVersionId: string): Promise<T> {
    return await invoke("plugin:anything|execute_flow", { flowId, flowVersionId });
  }
}
