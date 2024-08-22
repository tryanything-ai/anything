"use client";

import { useEffect } from "react";
import { useParams, useRouter, useSearchParams } from "next/navigation";
import api from "@/lib/anything-api"; // Adjust this import according to your API setup

const OAuthCallbackPage = () => {
  const router = useRouter();

  const searchParams = useSearchParams();

  // Convert the URLSearchParams object to a plain object
  const paramsObject = Object.fromEntries(searchParams.entries());

  const { code, state } = paramsObject;

  const { provider } = useParams<{
    provider: string;
  }>();

  useEffect(() => {
    const handleOAuthCallback = async () => {
      if (code && state) {
        console.log("Code:", code);
        console.log("State:", state);

        try {
          // Send the code and state to your server for further processing
          //TODO: in future we probably just push all params to the server and template them in the server
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
      } else {
        console.error("Missing code or state in the query parameters");
        // Handle error appropriately
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
        <p>Provider: {provider}</p>
      </div>
    </div>
  );
};

export default OAuthCallbackPage;
