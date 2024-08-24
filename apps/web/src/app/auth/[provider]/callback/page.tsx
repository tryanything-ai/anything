"use client";

import { useEffect, useState } from "react";
import { useParams, useRouter, useSearchParams } from "next/navigation";
import api from "@/lib/anything-api"; // Adjust this import according to your API setup

const OAuthCallbackPage = () => {
  const router = useRouter();
  const searchParams = useSearchParams();

  // State to store the parameters
  const [params, setParams] = useState<{ code?: string; state?: string }>({});
  const { provider } = useParams<{ provider: string }>();

  useEffect(() => {
    // Convert the URLSearchParams object to a plain object
    const paramsObject = Object.fromEntries(searchParams.entries());
    const { code, state } = paramsObject;

    if (code && state) {
      setParams({ code, state });

      // Remove query parameters from the URL
      router.replace(window.location.pathname);
    } else {
      console.error("Missing code or state in the query parameters");
      // Handle error appropriately
    }
  }, []);

  useEffect(() => {
    const handleOAuthCallback = async () => {
      const { code, state } = params;

      if (code && state) {
        console.log("Code:", code);
        console.log("State:", state);

        try {
          console.log("Calling Auth API for:", provider);
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
            console.log("Successfully handled OAuth callback:", response);
            // Redirect to the dashboard or another page after successful authentication
            router.push("/dashboard");
          }
        } catch (error) {
          console.error("Error handling OAuth callback:", error);
          // Handle error appropriately
        }
      }
    };

    if (params.code && params.state) {
      handleOAuthCallback();
    }
  }, [params]);

  return (
    <div>
      <h1>Processing OAuth Callback...</h1>
      <p>Please wait while we complete the authentication process.</p>
      <div>
        <p>Code: {params.code}</p>
        <p>State: {params.state}</p>
        <p>Provider: {provider}</p>
      </div>
    </div>
  );
};

export default OAuthCallbackPage;