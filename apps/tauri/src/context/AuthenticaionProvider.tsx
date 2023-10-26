import { Session } from '@supabase/supabase-js'
import { createContext, ReactNode,useContext, useEffect, useState } from "react";

import { supabaseClient, Profile } from "utils";
import { useSettingsContext } from "./SettingsProvider";

interface AuthenticationContextInterface {
  signInWithEmail: (email: string, password: string) => void;
  signUpWithEmail: (email: string, password: string) => void;
  signOut: () => void;
  profile: Profile | null;
}

export const AuthenticationContext =
  createContext<AuthenticationContextInterface>({
    signInWithEmail: () => {},
    signUpWithEmail: () => {},
    signOut: () => {},
    profile: null,
  });

export const useAuthenticaionContext = () => useContext(AuthenticationContext);

//This is basically strictly for template sharing, and sharing in genreal
//this is not for integration management which should be done closer to rust
export const AuthenticationProvider = ({
  children,
}: {
  children: ReactNode;
}) => {
  const { webFeaturesDisabled } = useSettingsContext();

  const [profile, setProfile] = useState<Profile | null>(null);
  const [session, setSession] = useState<Session | null>(null)

  const signUpWithEmail = async (email: string, password: string) => {
    if (webFeaturesDisabled) return null;
    if (!email || !password) return console.log("no email or password");
    const { data, error } = await supabaseClient.auth.signUp({
      email,
      password,
      options: {
        //TODO: deeplink?
        emailRedirectTo: `${location.origin}/auth/callback`,
      },
    });

    if (error) {
      console.log(error);
      return;
    }

    console.log("Signup data", JSON.stringify(data, null, 3));

    if (data && data.user) {
      //hydrate profile
      await fetchProfile(data.user.id);
    }
  };

  const signInWithEmail = async (email: string, password: string) => {
    if (webFeaturesDisabled) return null;
    if (!email || !password) return console.log("no email or password");
    const { data, error } = await supabaseClient.auth.signInWithPassword({
      email,
      password,
    });

    if (error) {
      console.log(error);
      return;
    }
    console.log("Login data", JSON.stringify(data, null, 3));
    //hydrate profile
    await fetchProfile(data.user.id);
  };

  const fetchProfile = async (user_id: string) => {
    if (webFeaturesDisabled) return null;
    try {
      let { data: profile, error } = await supabaseClient
        .from("profiles")
        .select("*")
        .eq("id", user_id);

      if (error) throw error;
      if (profile) {
        setProfile(profile[0]);
        return profile[0];
      } else {
        return undefined;
      }
    } catch (e) {
      console.log(e);
      return undefined;
    }
  };

  const signOut = async () => {
    if (webFeaturesDisabled) return null;
    await supabaseClient.auth.signOut();
    setProfile(null);
  };

  
  useEffect(() => {
    // Hydrate Session
    supabaseClient.auth.getSession().then(({ data: { session } }) => {
      console.log("session found in AuthenticationProvider", session)
      setSession(session)
    }); 

    //Subscribe to changes in auth state
    const {
      data: { subscription },
    } = supabaseClient.auth.onAuthStateChange((_event, session) => {
      console.log("session changed in AuthenticationProvider", session)
      setSession(session)
    })

    return () => subscription.unsubscribe()
  }, [])

  // if (!session) {
  //   return (<Auth supabaseClient={supabase} appearance={{ theme: ThemeSupa }} />)
  // }
  // else {
  //   return (<div>Logged in!</div>)
  // }

  return (
    <AuthenticationContext.Provider
      value={{ signInWithEmail, signUpWithEmail, signOut, profile }}
    >
      {children}
    </AuthenticationContext.Provider>
  );
};
