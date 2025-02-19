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


export const addToolToAgent = async (supabase: SupabaseClient, account_id: string, agentId: string, workflow_id: string) => {
    try {
      const { data: { session } } = await supabase.auth.getSession();
  
      if (session) {
        const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/agent/${agentId}/tool`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            Authorization: `${session.access_token}`,
          },
          body: JSON.stringify({
            workflow_id, 
          }),
        });
        const data = await response.json();
        console.log('Data from /agent POST:', data);
        return data;
      }
    } catch (error) {
      console.error('Error adding tool to agent:', error);
      throw error;
        }
    };


export const removeToolFromAgent = async (supabase: SupabaseClient, account_id: string, agentId: string, workflow_id: string) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        if (session) {  
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/agent/${agentId}/tool/${workflow_id}`, {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /agent/:id/tool/:toolId DELETE:', data);
            return data;
        }
    } catch (error) {
        console.error('Error removing tool from agent:', error);
        throw error;
    }
};


export const getAgentTools = async (supabase: SupabaseClient, account_id: string, agentId: string) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/agent/${agentId}/tools`, {
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /agent/:id/tools GET:', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching agent tools:', error);
        throw error;
    }
};

export const searchPhoneNumbers = async (supabase: SupabaseClient, account_id: string, country: string, area_code: string) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/phone_numbers/${country}/${area_code}`, {
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
            }); 
            const data = await response.json();

            console.log('Data from /phone_numbers/:country/:area_code GET:', data);
            return data;
        }
    } catch (error) {
        console.error('Error searching phone numbers:', error);
        throw error;
    }
};

export const buyPhoneNumber = async (supabase: SupabaseClient, account_id: string, phoneNumber: string) => {
    try {
          const { data: { session } } = await supabase.auth.getSession();

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/phone_number`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
                body: JSON.stringify({
                    phone_number: phoneNumber,
                }),   
            });
            const data = await response.json();
            console.log('Data from /phone_number POST:', data);
            return data;
        }
    } catch (error) {
        console.error('Error buying phone number:', error);
        throw error;
    }
};


export const addPhoneNumberToAgent = async (supabase: SupabaseClient, account_id: string, agentId: string, phoneNumberId: string) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/agent/${agentId}/phone_number`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
                body: JSON.stringify({
                    phone_number_id: phoneNumberId,   
                }),
            });
            const data = await response.json();
            console.log('Data from /agent/:id/phone_number POST:', data);
            return data;
        }
    } catch (error) {
        console.error('Error adding phone number to agent:', error);
        throw error;
    }
};

export const removePhoneNumberFromAgent = async (supabase: SupabaseClient, account_id: string, agentId: string, phoneNumberId: string) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        if (session) {  
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/agent/${agentId}/phone_number/${phoneNumberId}`, {
                method: 'DELETE',
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
            });
            const data = await response.json();
            console.log('Data from /agent/:id/phone_number/:phoneNumberId DELETE:', data);
            return data;
        }
    } catch (error) {
        console.error('Error removing phone number from agent:', error);
        throw error;
    }
};


export const getAccountPhoneNumbers = async (supabase: SupabaseClient, account_id: string) => {
    try {
        const { data: { session } } = await supabase.auth.getSession();

        if (session) {
            const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/phone_numbers`, {
                headers: {
                    'Content-Type': 'application/json',
                    Authorization: `${session.access_token}`,
                },
            }); 
            const data = await response.json();
            console.log('Data from /phone_numbers GET:', data);
            return data;
        }
    } catch (error) {
        console.error('Error fetching account phone numbers:', error);
        throw error;
    }
};

