import { Action, Workflow } from "@/types/workflows";
import { createClient } from "../supabase/client";
import { v4 as uuidv4 } from "uuid";

export type UpdateFlowArgs = {
  flow_name: string;
  active: boolean;
  version?: string;
};

export const getFlows = async () => {
  try {
    // Get JWT from supabase to pass to the API
    // API conforms to RLS policies on behalf of users for external API
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Session:', session);

    if (session) {
      const response = await fetch('http://localhost:3001/workflows', {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/workflows:', data);
      return data;
    }
  } catch (error) {
    console.error('Error fetching workflows:', error);
  } finally {
  }
}

export const createFlow = async (flowName: string) => {
  try {
    // Get JWT from supabase to pass to the API
    // API conforms to RLS policies on behalf of users for external API
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Creating Workflow');

    if (session) {
      let flow_id = uuidv4();
      const response = await fetch(`http://localhost:3001/workflow`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify({
          flow_id,
          flow_name: flowName
        }),
      });
      const data = await response.json();
      console.log('Data from /api/workflows POST:', data);
      return data;
    }
  } catch (error) {
    console.error('Error creating Workflow:', error);
  } finally {
  }
  // console.log(`Called createFlow with ${flowName}`);
  // let res = await anything.createFlow(flowName);
  // console.log(`Got back from createFlow ${JSON.stringify(res)}`);
  // return res;
};

export async function updateFlow(flowId: string, args: UpdateFlowArgs) {
  // return await anything.updateFlow(flowId, args);
}

export async function updateFlowVersion(flowId: string, flow: Workflow) {
  // console.log(`updateFlowVersion called for flow_id: ${flowId}}`, flow);
  // return await anything.updateFlowVersion(flowId, flow.version, {
  //   version: flow.version,
  //   flow_definition: JSON.stringify(flow),
  //   published: false,
  //   description: flow.description,
  // });
}

export async function deleteFlow(flowId: string) {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Deleting Workflow');

    if (session) {
      const response = await fetch(`http://localhost:3001/workflow/${flowId}`, {
        method: 'DELETE',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        }
      });

      const data = await response.json();
      console.log('Data from /api/workflows DELETE:', data);
      return data;
    }

  } catch (error) {
    console.error('Error deleting Workflow:', error);
  } finally {
  }
}

export const getFlow = async (flowId: string) => {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Fetching Workflow by ID');

    if (session) {
      const response = await fetch(`http://localhost:3001/workflow/${flowId}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        }
      });

      const data = await response.json();
      console.log('Data from /api/workflows GET:', data);
      return data;
    }

  } catch (error) {
    console.error('Error deleting Workflow:', error);
  } finally {
  }
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
