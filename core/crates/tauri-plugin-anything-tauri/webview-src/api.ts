import { CreateFlowVersion } from './type';
import { invoke } from "@tauri-apps/api/tauri";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { RenameFlowArgs } from "type";

export class Anything {
    path: string;

    constructor(path: string) {
        this.path = path;
    }

    async stop() {
        return await invoke("plugin:anything-tauri|stop", {
        })
    }

     async getFlows<T>(): Promise<T> {
        return await invoke("plugin:anything|get_flows", {
            path: this.path
        })
    }

    async getFlowByName<T>(flowName: string): Promise<T> {
        return await invoke("plugin:anything|get_flow_by_name", {
            flowName
        })
    }

    async createFlow<T>(flowName: string): Promise<T> {
        return await invoke("plugin:anything|create_flow", {flowName})
    }

    async CreateFlowVersion<T>(flowName: string, createFlowVersion: CreateFlowVersion): Promise<T> {
        return await invoke("plugin:anything|create_flow_version", {flowName, createFlowVersion})
    }
    
    async renameFlow<T>(flowId: string, updateFlow: RenameFlowArgs): Promise<T> {
        console.log("renameFlow called ", flowId, updateFlow);
        return await invoke("plugin:anything|update_flow", {flowId, updateFlow})
    }

    // async updateFlow<T>(flowId: string, updateFlow: UpdateFlow): Promise<T> {
    //     console.log("updateFlow called ", flowId, updateFlow);
    //     return await invoke("plugin:anything|update_flow", {flowId, updateFlow})
    // }

    async executeFlow<T>(flowName: string): Promise<T> {
        return await invoke("plugin:anything|execute_flow", {flowName})
    }

}