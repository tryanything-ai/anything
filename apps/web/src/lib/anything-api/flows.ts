import { Workflow } from "@/types/workflows";
import { createClient } from "../supabase/client";
import { v4 as uuidv4 } from "uuid";

export type UpdateFlowArgs = {
  flow_name?: string;
  active?: boolean;
};

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getFlows = async (account_id: string) => {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Session:', session);

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/workflows`, {
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

export const getFlowVersionById = async (account_id: string, workflowId: string, versionId: string) => {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Session:', session);

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/workflow/${workflowId}/version/${versionId}`, {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/workflow/:id/version/:id:', data);
      return data;
    }
  } catch (error) {
    console.error('Error fetching /api/workflow/:id/version/:id::', error);
  } finally {
  }
}

export const createFlow = async (account_id: string) => {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Creating Workflow');

    if (session) {
      let flow_id = uuidv4();
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/workflow`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify({
          flow_id
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
};

export async function updateFlow(account_id: string, flow_id: string, args: UpdateFlowArgs) {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Updating Workflow:', flow_id, "with args: ", args);

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/workflow/${flow_id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify({...args}),
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

export async function updateFlowVersion(account_id: string, flow_id: string, flow_version_id: string, flow_definition: Workflow) {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Updating Workflow');

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/workflow/${flow_id}/version/${flow_version_id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify(flow_definition),
      });

      const data = await response.json();
      return data;
    }

  } catch (error) {
    console.error('Error updating Workflow definition:', error);
  } finally {
  }
}

export async function deleteFlow(account_id: string, flowId: string) {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Deleting Workflow');

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/workflow/${flowId}`, {
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

export const getFlow = async (account_id: string, flowId: string) => {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Fetching Workflow by ID');

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/workflow/${flowId}`, {
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

export async function publishFlowVersion(account_id: string, flow_id: string, flow_version_id: string) {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Publishing Workflow');

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/workflow/${flow_id}/version/${flow_version_id}/publish`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        }
      });

      const data = await response.json();
      console.log('Data from PUT /api/workflows/:id/version/:id/publish:', data);
      return data;
    }

  } catch (error) {
    console.error('Error publishing Workflow:', error);
  } 
}

export const getFlowVersionsForWorkflowId = async (account_id: string, workflowId: string) => {
  try {
    const supabase = createClient();
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Session:', session);

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/workflow/${workflowId}/versions`, {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/workflows/:workflowId/versions:', data);
      return data;
    }
  } catch (error) {
    console.error('Error fetching workflow versions:', error);
  } finally {
  }
}