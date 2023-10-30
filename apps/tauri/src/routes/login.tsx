import { Auth } from "@supabase/auth-ui-react";
import { ThemeSupa, VIEWS } from "@supabase/auth-ui-shared";

import { supabaseClient } from "utils";
import PageLayout from "../pageLayout";
import { useParams } from "react-router-dom";
import { useEffect } from "react";
import { useAuthenticaionContext } from "../context/AuthenticaionProvider";
import { useLocation, useSearchParams } from "react-router-dom";

export default function Login() {
  // const { exchangeAccessTokenForSession } = useAuthenticaionContext();
  const { createSession, session } = useAuthenticaionContext();
  const location = useLocation();
  // const [searchParams, setSearchParams] = useSearchParams();

  //catch params and change UI
  // const { access_token } = useParams<{
  //   access_token: string;
  // }>();

  // const exchange = async (access_token: string) => {
  //   try {
  //     // await exchangeAccessTokenForSession(access_token);
  //   } catch (e) {
  //     console.log(e);
  //   }
  // };

  // useEffect(() => {
  //   if (access_token) {
  //     console.log("access_token in login componenet", access_token);
  //     //set session in supabase
  //     exchange(access_token);
  //     // authRef.current?.dispatch({
  //     //   type: "UPDATE_PASSWORD",
  //     //   access_token,
  //     // });
  //   } else {
  //     console.log("no access_token in login component");
  //   }
  // }, [access_token]);

  const extractTokens = (
    url: string
  ): { access_token: string; refresh_token: string } => {
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

    return { access_token, refresh_token };
  };

  const doIt = async () => {
    const currentHash = location.hash;
    let { access_token, refresh_token } = extractTokens(currentHash);

    let session = await createSession(access_token, refresh_token);
    console.log("session", session);
  };

  useEffect(() => {
    // Suapbase returns #access_token for update password email flow
    const currentHash = location.hash;
    // const currentSearch = location.search;
    // const searchParams = new URLSearchParams(location.pathname);
    // console.log(
    //   "searchParams from UrlSearchParams",
    //   JSON.stringify(searchParams, null, 3)
    // );
    // console.log("CurrentSearch", currentSearch);
    // console.log("CurrentLocation", location);
    // console.log("currentHash", JSON.stringify(currentHash, null, 3));
    // // Get access token from query param
    // const accessToken = searchParams.get("access_token");

    // // Get refresh token from query param
    // const refreshToken = searchParams.get("refresh_token");
    // console.log("accessToken", accessToken);
    // console.log("refreshToken", refreshToken);

    // auth should automatically set update_password view i think

    //if we have a link with an access_token
    if (currentHash.includes("#access_token")) {
      doIt();
    }
    //   console.log("currentHash has access token!", currentHash);

    //   console.log(
    //     "Current Search Params",
    //     JSON.stringify(searchParams, null, 3)
    //   );
    //   // const page = searchParams.get("page"); // Get page param
    //   // const pageSize = searchParams.get("pageSize"); // G

    //   if (currentHash.includes("type=recovery")) {
    //     console.log("currentHash has recovery token!", currentHash);
    //     //Navigate to update password route
    //   }
    // }
    //     let update_route = "/update-password" + route;
    //     console.log("Navigating to update-password route -> " + update_route);
    //     navigate(update_route);
    //   }

    // if (currentHash.startsWith("#access_token")) {
    //   console.log("currentHash has access token!", currentHash);

    //   console.log("Access_toekn route detected");
    //   //For Supabase auth links
    //   let access_token = currentHash.replace("#access_token=", "");
    //   console.log("access_token", access_token);
    //   // exchange(access_token);
    // }
  }, [location]);

  return (
    <PageLayout>
      <div className="flex flex-row h-full w-full justify-center items-center">
        <div className="w-96 text-white">
          {/* <h1 className="text-4xl font-bold mb-4">Anything</h1> */}
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
