import { invoke } from '@tauri-apps/api/tauri'

export type UpdateFlowArgs = {
  flow_name: string;
  active: boolean;
  version?: string;
};

export type CreateFlowVersion = {
  flowId: string;
  flowVersion: string;
  description?: string;
  flowDefinition: any;
  published: boolean;
};

export class Anything {
  path: string;

  constructor(path: string) {
    this.path = path;
  }

  async stop() {
    return await invoke("plugin:anything|stop", {});
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
    return await invoke("plugin:anything|update_flow_version", {
      flowId,
      flowVersionId,
      updateFlow,
    });
  }

  async executeFlow<T>(flowId: string, flowVersionId: string, sessionId?: string, stage?: string): Promise<T> {
    return await invoke("plugin:anything|execute_flow", { flowId, flowVersionId, sessionId, stage });
  }

  async fetchSessionEvents<T>(sessionId: string): Promise<T> {
    return await invoke("plugin:anything|fetch_session_events", {
      sessionId
    });
  }

  async getEvent<T>(eventId: string): Promise<T> {
    return await invoke("plugin:anything|get_event", {
      eventId
    });
  }

  async getActions<T>(): Promise<T> {
    return await invoke("plugin:anything|get_actions", {});
  }

  async saveAction<T>(action: any, actionName: String): Promise<T> {
    return await invoke("plugin:anything|save_action", { action, actionName });
  }
}
