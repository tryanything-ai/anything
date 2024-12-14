import { Workflow } from "./types/workflows";
import { SupabaseClient } from '@supabase/supabase-js';
import { v4 as uuidv4 } from "uuid";

export type UpdateFlowArgs = {
  flow_name?: string;
  active?: boolean;
};

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getFlows = async (supabase: SupabaseClient, account_id: string) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }
  console.log('ANYTHING_API_URL:', ANYTHING_API_URL);

  console.log('account_id:', account_id);

  try {
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Session in @repo/anything-api:', session);

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

export const getFlowVersionById = async (supabase: SupabaseClient, account_id: string, workflowId: string, versionId: string) => {
  try {
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

export const createFlow = async (supabase: SupabaseClient, account_id: string, name: string, description: string) => {
  try {
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
          flow_id,
          name,
          description,
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

export const createFlowFromJson = async (supabase: SupabaseClient, account_id: string, name: string, flow_template: any) => {
  try {
    const { data: { session } } = await supabase.auth.getSession();

    console.log('Creating Workflow');

    if (session) {
      let flow_id = uuidv4();
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/workflow/json`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify({
          flow_id,
          name,
          flow_template,
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

export async function updateFlow(supabase: SupabaseClient, account_id: string, flow_id: string, args: UpdateFlowArgs) {
  try {
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

export async function updateFlowVersion(supabase: SupabaseClient, account_id: string, flow_id: string, flow_version_id: string, flow_definition: Workflow) {
  try {
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

export async function deleteFlow(supabase: SupabaseClient, account_id: string, flowId: string) {
  try {
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

export const getFlow = async (supabase: SupabaseClient, account_id: string, flowId: string) => {
  try {
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

export async function publishFlowVersion(supabase: SupabaseClient, account_id: string, flow_id: string, flow_version_id: string) {
  try {
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

export const getFlowVersionsForWorkflowId = async (supabase: SupabaseClient, account_id: string, workflowId: string) => {
  try {
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