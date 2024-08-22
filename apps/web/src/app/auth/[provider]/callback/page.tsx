import { NextResponse } from 'next/server';
import api from '@/lib/anything-api'; // Adjust this import according to your API setup

export async function GET(request: Request) {
  const url = new URL(request.url);
  const code = url.searchParams.get('code');
  const state = url.searchParams.get('state');
  const provider = url.pathname.split('/')[2]; // Extract provider from the URL path

  if (!code || !state) {
    console.error('Missing code or state in the query parameters');
    return NextResponse.redirect('/error'); // Redirect to an error page if parameters are missing
  }

  try {
    // Use the api instance to handle the OAuth callback
    const response = await api.auth.handleCallbackForProvider({
      provider_name: provider!,
      code,
      state,
    });

    if (response.error) {
      console.error('Error handling OAuth callback:', response.error);
      return NextResponse.redirect('/error'); // Redirect to an error page if API returns an error
    } else {
      console.log('Successfully handled OAuth callback:', response);

      // Set a cookie with the session token or user info
      const res = NextResponse.redirect('/dashboard');
      res.cookies.set('auth_token', response.token); // Example of setting a cookie

      return res;
    }
  } catch (error) {
    console.error('Error handling OAuth callback:', error);
    return NextResponse.redirect('/error'); // Redirect to an error page on exception
  }
}
