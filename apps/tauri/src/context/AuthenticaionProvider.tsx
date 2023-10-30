import { Session } from "@supabase/supabase-js";
import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";

import { supabaseClient, Profile } from "utils";
import { useSettingsContext } from "./SettingsProvider";
import { useNavigate } from "react-router-dom";

interface AuthenticationContextInterface {
  signInWithEmail: (email: string, password: string) => void;
  signUpWithEmail: (email: string, password: string) => void;
  exchangeAccessTokenForSession: (access_token: string) => void;
  updateUser: () => void;
  getSession: () => void;
  createSession: (
    access_token: string,
    refresh_token: string
  ) => Promise<Session | null>;
  signOut: () => void;
  profile: Profile | null;
  session: Session | null;
}

export const AuthenticationContext =
  createContext<AuthenticationContextInterface>({
    signInWithEmail: () => {},
    signUpWithEmail: () => {},
    updateUser: () => {},
    createSession: () => null,
    exchangeAccessTokenForSession: () => {},
    getSession: () => {},
    signOut: () => {},
    profile: null,
    session: null,
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
  const navigate = useNavigate();

  const [profile, setProfile] = useState<Profile | null>(null);
  const [session, setSession] = useState<Session | null>(null);

  const createSessionFromUrl = async (access_token: string, refresh_token) => {
    if (!access_token) return null;
    if (!refresh_token) return null;

    const { data, error } = await supabaseClient.auth.setSession({
      access_token,
      refresh_token,
    });

    if (error) {
      console.log("Erorr setting session", JSON.stringify(error, null, 3));
      return null;
    }
    setSession(data.session);
    return data.session;
  };

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

  const exchangeAccessTokenForSession = async (code: string) => {
    let res = await supabaseClient.auth.exchangeCodeForSession(code);
    console.log("exchangeCodeForSession", JSON.stringify(res, null, 3));
  };

  const updateUser = async () => {
    if (webFeaturesDisabled) return null;
    console.log("updateUser called");
    let { data, error } = await supabaseClient.auth.updateUser({
      email: "carl@tryanything.xyz",
      password: "derpderp59!",
      // data: {
      // full_name: "John Smith",
      // age: 32,
      // is_beta: true,
      // },
    });
    if (error) console.log("updateUser error", error);
    console.log("updateUser", JSON.stringify(data, null, 3));
  };

  useEffect(() => {
    //update profile if sesssion exists
    if (session) {
      fetchProfile(session.user.id);
    }
  }, [session]);

  const getSession = async () => {
    supabaseClient.auth.getSession().then(({ data: { session } }) => {
      console.log("session found in AuthenticationProvider", session);
      setSession(session);
    });
  };

  useEffect(() => {
    // Hydrate Session
    getSession();

    console.log("subscribing to auth changes");

    //Subscribe to changes in auth state
    const {
      data: { subscription },
    } = supabaseClient.auth.onAuthStateChange((event, session) => {
      //use has hit a magic link to "update password" or forgot password
      // if (event === "PASSWORD_RECOVERY") {
      //   console.log("PASSWORD_RECOVERY");
      //   //   //send user to password update page
      //   //   navigate("/update-password");
      // }
      //user has updated password ( most likely )
      if (event === "USER_UPDATED") {
        console.log("USER_UPDATED");
        navigate("/");
      }
      console.log(
        "session changed in AuthenticationProvider",
        JSON.stringify(session, null, 3)
      );
      setSession(session);
    });

    return () => {
      console.log("unsubing from auth changes");
      subscription.unsubscribe();
    };
  }, []);

  // if (!session) {
  //   return (<Auth supabaseClient={supabase} appearance={{ theme: ThemeSupa }} />)
  // }
  // else {
  //   return (<div>Logged in!</div>)
  // }

  return (
    <AuthenticationContext.Provider
      value={{
        signInWithEmail,
        signUpWithEmail,
        signOut,
        profile,
        exchangeAccessTokenForSession,
        session,
        updateUser,
        getSession,
        createSession: createSessionFromUrl,
      }}
    >
      {children}
    </AuthenticationContext.Provider>
  );
};
