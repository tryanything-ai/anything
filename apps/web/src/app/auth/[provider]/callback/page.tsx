import { redirect } from "next/navigation";
import api from "@/lib/anything-api"; // Adjust this import according to your API setup

const OAuthCallbackPage = async ({ searchParams, params }: any) => {
  const { provider } = params;
  const { code, state } = searchParams;

  if (!code || !state) {
    console.error("Missing code or state in the query parameters");
    // Handle error appropriately, e.g., redirect to an error page
    return (
      <div>
        <h1>Error</h1>
        <p>Missing code or state in the query parameters.</p>
      </div>
    );
  }

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
      return (
        <div>
          <h1>Error</h1>
          <p>Error handling OAuth callback: {response.error.message}</p>
        </div>
      );
    } else {
      console.log("Successfully handled OAuth callback:", response);
      // Redirect to the dashboard or another page after successful authentication
      redirect("/dashboard");
    }
  } catch (error: any) {
    console.error("Error handling OAuth callback:", error);
    // Handle error appropriately
    return (
      <div>
        <h1>Error</h1>
        <p>Error handling OAuth callback: {error.message}</p>
      </div>
    );
  }

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
