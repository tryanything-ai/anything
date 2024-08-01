import { Action, Workflow } from "@/types/workflows";
import { createClient } from "../supabase/client";
import { v4 as uuidv4 } from "uuid";

export type UpdateFlowArgs = {
  flow_name: string;
  active: boolean;
  version?: string;
};

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getFlows = async () => {
  try {
    // Get JWT from supabase to pass to the API
    // API conforms to RLS policies on behalf of users for external API
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Session:', session);

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/workflows`, {
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
      const response = await fetch(`${ANYTHING_API_URL}/workflow`, {
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

export async function updateFlow(flow_id: string, args: UpdateFlowArgs) {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Updating Workflow');

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/workflow/${flow_id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify(args),
      });

      const data = await response.json();
      console.log('Data from /api/workflows/id PUT:', data);
      return data;
    }

  } catch (error) {
    console.error('Error updating Workflow definition:', error);
  } finally {
  }
}

export async function updateFlowVersion(flow_id: string, flow_version_id: string, flow_definition: Workflow) {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Updating Workflow');

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/workflow/${flow_id}/version/${flow_version_id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify(flow_definition),
      });

      const data = await response.json();
      // console.log('Data from /api/workflows/id/version/id PUT:', data);
      return data;
    }

  } catch (error) {
    console.error('Error updating Workflow definition:', error);
  } finally {
  }
}

export async function deleteFlow(flowId: string) {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Deleting Workflow');

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/workflow/${flowId}`, {
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
      const response = await fetch(`${ANYTHING_API_URL}/workflow/${flowId}`, {
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
    console.error('Error getting Workflow:', error);
  } finally {
  }
};

export async function publishFlowVersion(flow_id: string, flow_version_id: string) {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Publishing Workflow');

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/workflow/${flow_id}/version/${flow_version_id}/publish`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        }, 
        body: JSON.stringify({"derp": true}),
      });

      const data = await response.json();
      console.log('Data from PUT /api/workflows/:id/version/:id/publish:', data);
      return data;
    }

  } catch (error) {
    console.error('Error publishing Workflow:', error);
  } 
}