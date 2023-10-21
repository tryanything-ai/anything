import { Auth } from "@supabase/auth-ui-react";
import { ThemeSupa } from "@supabase/auth-ui-shared";
import { supabase } from "../utils/initSupabase";

export default function Login() {
  return (
    <div className="flex flex-row h-full w-full justify-center items-center">
      <div className="w-96 text-white">
        <Auth
          supabaseClient={supabase}
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
  );
}
