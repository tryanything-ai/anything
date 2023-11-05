import { Auth } from "@supabase/auth-ui-react";
import { ThemeSupa } from "@supabase/auth-ui-shared";

import { supabaseClient } from "utils";
import PageLayout from "../pageLayout";
import { useEffect } from "react";
import { useAuthenticationContext } from "../context/AuthenticaionProvider";
import { useLocation, useNavigate } from "react-router-dom";

export default function Login() {
  const { createSession, session } = useAuthenticationContext();
  const location = useLocation();
  const navigate = useNavigate();

  const extractTokens = (
    url: string
  ): { access_token: string; refresh_token: string; type: string } => {
    const urlParts = url.split("#");
    const params = new URLSearchParams(urlParts[1]);
    const access_token = params.get("access_token");
    const refresh_token = params.get("refresh_token");
    const expires_at = params.get("expires_at");
    const expires_in = params.get("expires_in");
    const token_type = params.get("token_type");
    const type = params.get("type");
    console.log("Access Token: ", access_token);
    console.log("Refresh Token: ", refresh_token);
    console.log("Expires At: ", expires_at);
    console.log("Expires In: ", expires_in);
    console.log("Token Type: ", token_type);
    console.log("Type: ", type);

    return { access_token, refresh_token, type };
  };

  const manageResetFlow = async () => {
    const currentHash = location.hash;
    let { access_token, refresh_token, type } = extractTokens(currentHash);

    //need to do this for stronger gurantees with deeplinking
    let session = await createSession(access_token, refresh_token);

    console.log("session", session);

    if (type === "signup") {
      navigate("/");
    } else if (type === "recovery") {
      navigate("/update-password");
    } else {
      console.log("Unsure how to handle this access_token");
    }
  };

  useEffect(() => {
    // Suapbase returns #access_token for update password email flow
    const currentHash = location.hash;

    //if we have a link with an access_token
    if (currentHash.includes("#access_token")) {
      manageResetFlow();
    }
  }, [location]);

  return (
    <PageLayout>
      <div className="flex flex-row h-full w-full justify-center items-center">
        <div className="w-96 text-white">
          <h1 className="text-4xl font-bold mb-4">Anything</h1>
          <Auth
            key={JSON.stringify(session)} //seems super naughty
            supabaseClient={supabaseClient}
            providers={[]}
            redirectTo="anything://login"
            appearance={{
              theme: ThemeSupa,
              variables: {
                default: {
                  colors: {
                    inputText: "white",
                    brand: "#FF00BF",
                    brandAccent: "#E600BD",
                  },
                },
              },
            }}
          />
        </div>
      </div>
    </PageLayout>
  );
}
