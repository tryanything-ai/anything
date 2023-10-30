import { useEffect } from "react";
import { Auth } from "@supabase/auth-ui-react";
import { ThemeSupa, VIEWS } from "@supabase/auth-ui-shared";

import { supabaseClient } from "utils";
import PageLayout from "../pageLayout";
import { useLocation } from "react-router-dom";

import { useAuthenticaionContext } from "../context/AuthenticaionProvider";

export default function UpdatePassword() {
  const { updateUser, getSession } = useAuthenticaionContext();

  const location = useLocation();

  useEffect(() => {
    // Suapbase returns #access_token for update password email flow
    const currentHash = location.hash;
    console.log("currentHash", JSON.stringify(currentHash, null, 3));
    if (currentHash.startsWith("#access_token")) {
      console.log("currentHash has access token!", currentHash);

      console.log("Access_toekn route detected");
      //For Supabase auth links
      let access_token = currentHash.replace("#access_token=", "");
      console.log("access_token", access_token);
      // exchange(access_token);
    }
  }, [location]);

  useEffect(() => {
    getSession();
  }, []);

  return (
    <PageLayout>
      <button onClick={updateUser}>Update Password</button>
      <div className="flex flex-row h-full w-full justify-center items-center">
        <div className="w-96 text-white">
          <h1 className="text-4xl font-bold mb-4">Update Password</h1>
          <Auth
            supabaseClient={supabaseClient}
            providers={[]}
            // redirectTo="anything://" //will this go to home?
            view={VIEWS.UPDATE_PASSOWRD}
            showLinks={false}
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
