import { SupabaseClient } from '@supabase/supabase-js';
import { v4 as uuidv4 } from "uuid";

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL;

export const getCampaigns = async (supabase: SupabaseClient, account_id: string) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/campaigns`, {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/campaigns:', data);
      return data;
    }
  } catch (error) {
    console.error('Error fetching campaigns:', error);
    throw error;
  }
};

export const getCampaign = async (supabase: SupabaseClient, account_id: string, campaign_id: string) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/campaign/${campaign_id}`, {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/campaign/:id:', data);
      return data;
    }
  } catch (error) {
    console.error('Error fetching campaign:', error);
    throw error;
  }
};

export const createCampaign = async (
  supabase: SupabaseClient, 
  account_id: string, 
  campaignData: { 
    name: string; 
    description: string; 
    agent_id: string;
  }
) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const campaign_id = uuidv4();
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/campaign`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify({
          campaign_id,
          campaign_name: campaignData.name,
          description: campaignData.description,
          agent_id: campaignData.agent_id,
          status: 'draft',
        }),
      });
      const data = await response.json();
      console.log('Data from /api/campaign POST:', data);
      return data;
    }
  } catch (error) {
    console.error('Error creating campaign:', error);
    throw error;
  }
};

export const deleteCampaign = async (supabase: SupabaseClient, account_id: string, campaign_id: string) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/campaign/${campaign_id}`, {
        method: 'DELETE',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/campaign DELETE:', data);
      return data;
    }
  } catch (error) {
    console.error('Error deleting campaign:', error);
    throw error;
  }
};

export const getCampaignContacts = async (supabase: SupabaseClient, account_id: string, campaign_id: string) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/campaign/${campaign_id}/contacts`, {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/campaign/:id/contacts:', data);
      return data;
    }
  } catch (error) {
    console.error('Error fetching campaign contacts:', error);
    throw error;
  }
};

export const uploadCustomerList = async (
  supabase: SupabaseClient, 
  account_id: string, 
  campaign_id: string, 
  file: File
) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const formData = new FormData();
      formData.append('file', file);

      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/campaign/${campaign_id}/upload`, {
        method: 'POST',
        headers: {
          Authorization: `${session.access_token}`,
        },
        body: formData,
      });
      const data = await response.json();
      console.log('Data from /api/campaign/:id/upload POST:', data);
      return data;
    }
  } catch (error) {
    console.error('Error uploading customer list:', error);
    throw error;
  }
};

export const updateCampaignStatus = async (
  supabase: SupabaseClient, 
  account_id: string, 
  campaign_id: string, 
  status: string
) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/campaign/${campaign_id}/status`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify({ status }),
      });
      const data = await response.json();
      console.log('Data from /api/campaign/:id/status PUT:', data);
      return data;
    }
  } catch (error) {
    console.error('Error updating campaign status:', error);
    throw error;
  }
};
