import { Auth } from "@supabase/auth-ui-react";
import { ThemeSupa } from "@supabase/auth-ui-shared";

import { supabaseClient } from "utils";
import PageLayout from "../pageLayout";

export default function Login() {
  return (
    <PageLayout>
      <div className="flex flex-row h-full w-full justify-center items-center">
        <div className="w-96 text-white">
          <Auth
            supabaseClient={supabaseClient}
            providers={[]}
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
