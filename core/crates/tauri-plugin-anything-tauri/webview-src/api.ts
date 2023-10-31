import { invoke } from "@tauri-apps/api/tauri";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

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

    async createFlow<T>(flowName: string, flowId: string): Promise<T> {
        return await invoke("plugin:anything|create_flow", {
            path: this.path,
            flowName: flowName,
            flowId: flowId
        })
    }
}