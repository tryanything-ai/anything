import { createClient } from "../supabase/client";

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL

export const getAccountStatus = async (account_id: string) => {
    try {
      const supabase = createClient();
      const { data: { session } } = await supabase.auth.getSession();
  
      console.log('Session:', session);
  
      if (session) {
        const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/billing/status`, {
          headers: {
            Authorization: `${session.access_token}`,
          },
        });
        const data = await response.json();
        console.log('Data from /api/account/:account_id/status:', data);
        return data;
      }
    } catch (error) {
      console.error('Error fetching account status:', error);
    } finally {
    }
  }

export const getCheckoutLink = async (account_id: string, return_url: string) => {
    try {
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();
    
        console.log('Session:', session);
    
        if (session) {
          const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/billing/checkout`, {
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              Authorization: `${session.access_token}`,
            },
            body: JSON.stringify({ return_url }),
          });
          const data = await response.json();
          console.log('Data from /api/account/:account_id/billing/checkout:', data);
          return data;
        }
      } catch (error) {
        console.error('Error fetching checkout link:', error);
      } finally {
      }
}

export const getBillingPortalLink = async (account_id: string, returnUrl: string) => {
    try {
        const supabase = createClient();
        const { data: { session } } = await supabase.auth.getSession();
    
        console.log('Session:', session);
    
        if (session) {
          const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/billing/portal`, {
            method: 'POST',
            headers: {
              Authorization: `${session.access_token}`,
            },
            body: JSON.stringify({ returnUrl }),
          });
          const data = await response.json();
          console.log('Data from /api/account/:account_id/billing/portal:', data);
          return data;
        }
      } catch (error) {
        console.error('Error fetching billing portal link:', error);
      } finally {
      }
}