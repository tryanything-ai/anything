import { Auth } from "@supabase/auth-ui-react";
import { ThemeSupa, VIEWS } from "@supabase/auth-ui-shared";

import { supabaseClient } from "utils";
import PageLayout from "../pageLayout";
import { useParams } from "react-router-dom";

export default function Login() {
  //catch params and change UI
  const { slug } = useParams<{
    slug: string;
  }>();

  return (
    <PageLayout>
      <div className="flex flex-row h-full w-full justify-center items-center">
        <div className="w-96 text-white">
          {/* <h1 className="text-4xl font-bold mb-4">Anything</h1> */}
          <Auth
            supabaseClient={supabaseClient}
            providers={[]}
            redirectTo="anything://loginPage"
            // view={}
            appearance={{
              theme: ThemeSupa,
              variables: {
                default: {
                  colors: {
                    inputText: "white",
                    brand: "#FF00BF",
                    brandAccent: "#E600BD",
                    //   brandAccent: "darkred",
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
