import { SupabaseClient } from '@supabase/supabase-js';
import { v4 as uuidv4 } from "uuid";

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL;

export const getCampaigns = async (supabase: SupabaseClient, account_id: string) => {
  console.log('[CAMPAIGN] Getting campaigns for account', account_id);
  if (!ANYTHING_API_URL) {
    console.error('[CAMPAIGN] ANYTHING_API_URL is not defined');
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
      console.log('[CAMPAIGN] Successfully fetched campaigns:', data);
      return data;
    }
  } catch (error) {
    console.error('[CAMPAIGN] Error fetching campaigns:', error);
    throw error;
  }
};

export const getCampaign = async (supabase: SupabaseClient, account_id: string, campaign_id: string) => {
  console.log('[CAMPAIGN] Getting campaign details', { account_id, campaign_id });
  if (!ANYTHING_API_URL) {
    console.error('[CAMPAIGN] ANYTHING_API_URL is not defined');
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
      console.log('[CAMPAIGN] Successfully fetched campaign details:', data);
      return data;
    }
  } catch (error) {
    console.error('[CAMPAIGN] Error fetching campaign:', error);
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
  console.log('[CAMPAIGN] Creating new campaign', { account_id, campaignData });
  if (!ANYTHING_API_URL) {
    console.error('[CAMPAIGN] ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/campaign`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify({
          name: campaignData.name,
          description: campaignData.description,
          agent_id: campaignData.agent_id,
        }),
      });
      const data = await response.json();
      console.log('[CAMPAIGN] Successfully created campaign:', data);
      return data;
    }
  } catch (error) {
    console.error('[CAMPAIGN] Error creating campaign:', error);
    throw error;
  }
};

export const deleteCampaign = async (supabase: SupabaseClient, account_id: string, campaign_id: string) => {
  console.log('[CAMPAIGN] Deleting campaign', { account_id, campaign_id });
  if (!ANYTHING_API_URL) {
    console.error('[CAMPAIGN] ANYTHING_API_URL is not defined');
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
      console.log('[CAMPAIGN] Successfully deleted campaign:', data);
      return data;
    }
  } catch (error) {
    console.error('[CAMPAIGN] Error deleting campaign:', error);
    throw error;
  }
};

export const getCampaignContacts = async (supabase: SupabaseClient, account_id: string, campaign_id: string) => {
  console.log('[CAMPAIGN] Getting campaign contacts', { account_id, campaign_id });
  if (!ANYTHING_API_URL) {
    console.error('[CAMPAIGN] ANYTHING_API_URL is not defined');
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
      console.log('[CAMPAIGN] Successfully fetched campaign contacts:', data);
      return data;
    }
  } catch (error) {
    console.error('[CAMPAIGN] Error fetching campaign contacts:', error);
    throw error;
  }
};


export const updateCampaignStatus = async (
  supabase: SupabaseClient, 
  account_id: string, 
  campaign_id: string, 
  status: string
) => {
  console.log('[CAMPAIGN] Updating campaign status', { account_id, campaign_id, status });
  if (!ANYTHING_API_URL) {
    console.error('[CAMPAIGN] ANYTHING_API_URL is not defined');
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
      console.log('[CAMPAIGN] Successfully updated campaign status:', data);
      return data;
    }
  } catch (error) {
    console.error('[CAMPAIGN] Error updating campaign status:', error);
    throw error;
  }
};

/**
 * Creates contacts and adds them to a campaign with deduplication by phone number.
 * 
 * This function sends contacts to the server where they are processed one by one (no batching):
 * 1. For each contact, the server checks if a contact with the same phone number already exists
 * 2. If the contact exists, it uses that contact's ID
 * 3. If the contact doesn't exist, it creates a new contact
 * 4. It then adds the contact to the campaign
 * 
 * @param supabase - Supabase client
 * @param account_id - Account ID
 * @param campaign_id - Campaign ID
 * @param contacts - Array of contacts to create and add to the campaign
 * @returns Object with counts of created, existing, and added contacts
 */
export const addContactsToCampaignWithDeduplication = async (
  supabase: SupabaseClient,
  account_id: string,
  campaign_id: string,
  contacts: Array<{
    name: string;
    phone_number: string;
    email?: string;
    additional_data?: any;
  }>
) => {
  console.log('[CAMPAIGN] Creating and adding contacts to campaign', { 
    account_id, 
    campaign_id, 
    contactCount: contacts.length 
  });
  
  if (!ANYTHING_API_URL) {
    console.error('[CAMPAIGN] ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      console.log('[CAMPAIGN] Sending contacts to be processed individually (no batching)');
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/campaign/${campaign_id}/contacts/create`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify({ contacts }),
      });
      
      const data = await response.json();
      console.log('[CAMPAIGN] Successfully processed contacts:', data);
      return data;
    }
  } catch (error) {
    console.error('[CAMPAIGN] Error creating and adding contacts:', error);
    throw error;
  }
};

export interface UpdateCampaignInput {
  name?: string;
  description?: string;
  agent_id?: string;
  schedule_days_of_week?: string[];
  schedule_start_time?: string;
  schedule_end_time?: string;
  timezone?: string;
}

/**
 * Updates a campaign with the provided data
 * 
 * @param supabase - Supabase client
 * @param account_id - Account ID
 * @param campaign_id - Campaign ID
 * @param campaignData - Data to update
 * @returns Updated campaign data
 */
export const updateCampaign = async (
  supabase: SupabaseClient,
  account_id: string,
  campaign_id: string,
  campaignData: UpdateCampaignInput
) => {
  console.log('[CAMPAIGN] Updating campaign', { 
    account_id, 
    campaign_id, 
    campaignData 
  });
  
  if (!ANYTHING_API_URL) {
    console.error('[CAMPAIGN] ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/campaign/${campaign_id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify(campaignData),
      });
      
      const data = await response.json();
      console.log('[CAMPAIGN] Successfully updated campaign:', data);
      return data;
    }
  } catch (error) {
    console.error('[CAMPAIGN] Error updating campaign:', error);
    throw error;
  }
};

