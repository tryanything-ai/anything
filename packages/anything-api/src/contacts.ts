import { SupabaseClient } from '@supabase/supabase-js';
import { v4 as uuidv4 } from "uuid";

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL;

export interface ContactInput {
  first_name?: string;
  last_name?: string;
  email?: string;
  phone?: string;
  company?: string;
  title?: string;
  address?: string;
  city?: string;
  state?: string;
  postal_code?: string;
  country?: string;
  status?: string;
  source?: string;
  notes?: string;
  tags?: string[];
  custom_fields?: Record<string, any>;
}

export const getContacts = async (supabase: SupabaseClient, account_id: string) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/contacts`, {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/contacts:', data);
      return data;
    }
  } catch (error) {
    console.error('Error fetching contacts:', error);
    throw error;
  }
};

export const getContact = async (supabase: SupabaseClient, account_id: string, contact_id: string) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/contact/${contact_id}`, {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/contact/:id:', data);
      return data;
    }
  } catch (error) {
    console.error('Error fetching contact:', error);
    throw error;
  }
};

export const createContact = async (
  supabase: SupabaseClient, 
  account_id: string, 
  contactData: ContactInput
) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/contact`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify(contactData),
      });
      const data = await response.json();
      console.log('Data from /api/contact POST:', data);
      return data;
    }
  } catch (error) {
    console.error('Error creating contact:', error);
    throw error;
  }
};

export const updateContact = async (
  supabase: SupabaseClient, 
  account_id: string, 
  contact_id: string,
  contactData: ContactInput
) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/contact/${contact_id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
        body: JSON.stringify(contactData),
      });
      const data = await response.json();
      console.log('Data from /api/contact/:id PUT:', data);
      return data;
    }
  } catch (error) {
    console.error('Error updating contact:', error);
    throw error;
  }
};

export const deleteContact = async (supabase: SupabaseClient, account_id: string, contact_id: string) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/contact/${contact_id}`, {
        method: 'DELETE',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/contact/:id DELETE:', data);
      return data;
    }
  } catch (error) {
    console.error('Error deleting contact:', error);
    throw error;
  }
};

export const searchContacts = async (
  supabase: SupabaseClient, 
  account_id: string, 
  searchTerm: string
) => {
  if (!ANYTHING_API_URL) {
    console.error('ANYTHING_API_URL is not defined');
    throw new Error('ANYTHING_API_URL is not defined');
  }

  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/contacts/search?term=${encodeURIComponent(searchTerm)}`, {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      console.log('Data from /api/contacts/search:', data);
      return data;
    }
  } catch (error) {
    console.error('Error searching contacts:', error);
    throw error;
  }
}; 