import { Action, Flow } from "@/types/flows";
import { createClient } from "../supabase/client";

export type UpdateFlowArgs = {
  flow_name: string;
  active: boolean;
  version?: string;
};

export const getFlows = async () => {
  //TODO: make this actually work
  try {
    // Get JWT from supabase to pass to the API
    // API conforms to RLS policies on behalf of users for external API
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Session:', session);

    if (session) {
      const response = await fetch('http://localhost:3001/items', {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/items:', data);
      return data;
    }
  } catch (error) {
    console.error('Error fetching items:', error);
  } finally {
  }
}

export const createFlow = async (flowName: string) => {
  // console.log(`Called createFlow with ${flowName}`);
  // let res = await anything.createFlow(flowName);
  // console.log(`Got back from createFlow ${JSON.stringify(res)}`);
  // return res;
};

export async function updateFlow(flowId: string, args: UpdateFlowArgs) {
  // return await anything.updateFlow(flowId, args);
}

export async function updateFlowVersion(flowId: string, flow: Flow) {
  // console.log(`updateFlowVersion called for flow_id: ${flowId}}`, flow);
  // return await anything.updateFlowVersion(flowId, flow.version, {
  //   version: flow.version,
  //   flow_definition: JSON.stringify(flow),
  //   published: false,
  //   description: flow.description,
  // });
}

export async function deleteFlow(flowId: string) {
  // return await anything.deleteFlow(flowId);
}

export const getFlow = async (flowId: string) => {
  // return await invoke("get_flow", { flowId });
};

export const getFlowByName = async (flowName: string) => {
  // return await anything.getFlowByName(flowName);
};

export const getFlowVersions = async (flowId: string) => {
  // return await invoke("get_flow_versions", { flowId });
};

export const executeFlow = async (flowId: string, flowVersionId: string, sessionId?: string, stage?: string) => {
  // console.log(`executeFlow called with flowId in tauri_api: ${flowId}, flowVersionId: ${flowVersionId}, sessionId: ${sessionId}, stage: ${stage}`);
  // return await anything.executeFlow(flowId, flowVersionId, sessionId, stage);
};

export const fetchSessionEvents = async (sessionId: string) => {
  // return await anything.fetchSessionEvents(sessionId);
};

export const getEvent = async (eventId: string) => {
  // return await anything.getEvent(eventId);
}

export const getActions = async () => {
  // let res = await anything.getActions<{ actions: Action[] }>();
  // return res.actions;
}

export const saveAction = async (action: Action) => {
  // let actionName = action.node_label;
  // console.log("saveAction Label in flows.ts: ", actionName)
  // return await anything.saveAction(action, actionName);
}

export const stopExecution = async () => {
  // return await anything.stop();
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
