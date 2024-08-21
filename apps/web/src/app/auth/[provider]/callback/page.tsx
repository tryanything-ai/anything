"use client";

import { useEffect } from "react";
import { useParams, useRouter } from "next/navigation";
import api from "@/lib/anything-api"; // Adjust this import according to your API setup

const OAuthCallbackPage = () => {
  const router = useRouter();
  const { code, state, provider } = useParams<{
    code: string;
    state: string;
    provider: string;
  }>();

  useEffect(() => {
    const handleOAuthCallback = async () => {
      if (code && state) {
        console.log("Code:", code);
        console.log("State:", state);

        try {
          // Send the code and state to your server for further processing
          const response = await api.auth.handleCallbackForProvider({
            provider_name: provider,
            code,
            state,
          });

          if (response.error) {
            console.error("Error handling OAuth callback:", response.error);
            // Handle error appropriately, e.g., show a notification or redirect to an error page
          } else {
            // Redirect to the dashboard or another page after successful authentication
            router.push("/dashboard");
          }
        } catch (error) {
          console.error("Error handling OAuth callback:", error);
          // Handle error appropriately
        }
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
