"use client";

import { useEffect } from "react";
// import { useRouter } from "next/router";
import { useParams } from "next/navigation";
// import { createClient } from "@/lib/supabase/client";

const OAuthCallbackPage = () => {
  const { code, state } = useParams<{ code: string; state: string }>();
  // const { code, state } = router.query;

  useEffect(() => {
    const handleOAuthCallback = async () => {
      if (code) {
        console.log("code:", code);
        console.log("state:", state);
        // const supabase = createClient();
        // const { error } = await supabase.auth.exchangeCodeForSession(code as string);

        // if (error) {
        //   console.error('Error exchanging code for session:', error);
        //   // Handle error appropriately, e.g., show a notification or redirect to an error page
        // } else {
        //   // Redirect to the dashboard or another page after successful authentication
        //   router.push('/dashboard');
        // }
      }
    };

    handleOAuthCallback();
  }, [code, state]);

  return (
    <div>
      <h1>Processing OAuth Callback...</h1>
      <p>Please wait while we complete the authentication process.</p>
      <div>
        <p>Code: {code}</p>
        <p>State: {state}</p>
      </div>
    </div>
  );
};

export default OAuthCallbackPage;
