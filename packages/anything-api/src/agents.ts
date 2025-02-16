import { SupabaseClient } from '@supabase/supabase-js';

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL;

export const createAgent = async (supabase: SupabaseClient, account_id: string, name: string) => {
  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/agent`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify({
          name
        }),
      });
      const data = await response.json();
      console.log('Data from /agent POST:', data);
      return data;
    }
  } catch (error) {
    console.error('Error creating Agent:', error);
    throw error;
  }
};

export const getAgents = async (supabase: SupabaseClient, account_id: string) => {
  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/agents`, {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /agents GET:', data);
      return data;
    }
  } catch (error) {
    console.error('Error fetching agents:', error);
    throw error;
  }
};

export const getAgent = async (supabase: SupabaseClient, account_id: string, agentId: string) => {
  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/agent/${agentId}`, {
        headers: {
          Authorization: `${session.access_token}`, 
        },
      });
      const data = await response.json();
      console.log('Data from /agent/:id GET:', data);
      return data;
    }
  } catch (error) {
    console.error('Error fetching agent:', error);
    throw error;
  }
};

export const updateAgent = async (supabase: SupabaseClient, account_id: string, agentId: string, updates: {
  name: string;
  greeting: string;
  system_prompt: string;
}) => {
  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/agent/${agentId}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify(updates),
      });
      const data = await response.json();
      console.log('Data from /agent/:id PUT:', data);
      return data;
    }
  } catch (error) {
    console.error('Error updating agent:', error);
    throw error;
  }
};

export const deleteAgent = async (supabase: SupabaseClient, account_id: string, agentId: string) => {
  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/agent/${agentId}`, {
        method: 'DELETE',
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /agent/:id DELETE:', data);
      return data;
    }
  } catch (error) {
    console.error('Error deleting agent:', error);
    throw error;
  }
};
