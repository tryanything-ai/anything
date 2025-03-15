import { SupabaseClient } from '@supabase/supabase-js';

const ANYTHING_API_URL = process.env.NEXT_PUBLIC_ANYTHING_API_URL;

export async function uploadFile(
  supabase: SupabaseClient, 
  account_id: string, 
  file: File,
  access: string,
  onProgress?: (progress: number) => void
): Promise<any> {
  return new Promise(async (resolve, reject) => {
    try {
      const { data: { session } } = await supabase.auth.getSession();

      if (!session) {
        throw new Error('No session found');
      }

      const formData = new FormData();
      formData.append('file', file);

      const xhr = new XMLHttpRequest();

      // Add progress event listener
      xhr.upload.addEventListener('progress', (event) => {
        if (event.lengthComputable && onProgress) {
          const percentComplete = (event.loaded / event.total) * 100;
          onProgress(percentComplete);
        }
      });

      // Add load event listener
      xhr.addEventListener('load', () => {
        if (xhr.status >= 200 && xhr.status < 300) {
          resolve(JSON.parse(xhr.response));
        } else {
          reject(new Error(`Upload failed with status ${xhr.status}`));
        }
      });

      // Add error event listener
      xhr.addEventListener('error', () => {
        reject(new Error('Upload failed'));
      });

      // Open and send the request
      xhr.open('POST', `${ANYTHING_API_URL}/account/${account_id}/file/upload/${access}`);
      xhr.setRequestHeader('Authorization', session.access_token);
      xhr.send(formData);

    } catch (error) {
      console.error('Error uploading file:', error);
      reject(error);
    }
  });
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
