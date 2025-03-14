import { SupabaseClient } from '@supabase/supabase-js';

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL;

export async function uploadFile(
  supabase: SupabaseClient, 
  account_id: string, 
  file: File,
  onProgress?: (progress: number) => void
) {
  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const formData = new FormData();
      formData.append('file', file);

      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/file`, {
        method: 'POST',
        headers: {
          Authorization: `${session.access_token}`,
        },
        body: formData,
      });

      const data = await response.json();
      return data;
    }
  } catch (error) {
    console.error('Error uploading file:', error);
    throw error;
  }
}

export async function getFiles(supabase: SupabaseClient, account_id: string) {
  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(`${ANYTHING_API_URL}/account/${account_id}/files`, {
        headers: {
          Authorization: `${session.access_token}`,
        },
      });
      const data = await response.json();
      return data;
    }
  } catch (error) {
    console.error('Error fetching files:', error);
    throw error;
  }
}

export async function getFileDownloadUrl(
  supabase: SupabaseClient,
  account_id: string,
  file_id: string
) {
  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(
        `${ANYTHING_API_URL}/account/${account_id}/file/${file_id}/download`,
        {
          headers: {
            Authorization: `${session.access_token}`,
          },
        }
      );
      const data = await response.json();
      return data.downloadUrl;
    }
  } catch (error) {
    console.error('Error getting file download URL:', error);
    throw error;
  }
}

export async function deleteFile(
  supabase: SupabaseClient,
  account_id: string,
  file_id: string
) {
  try {
    const { data: { session } } = await supabase.auth.getSession();

    if (session) {
      const response = await fetch(
        `${ANYTHING_API_URL}/account/${account_id}/file/${file_id}`,
        {
          method: 'DELETE',
          headers: {
            Authorization: `${session.access_token}`,
          },
        }
      );
      const data = await response.json();
      return data;
    }
  } catch (error) {
    console.error('Error deleting file:', error);
    throw error;
  }
}
