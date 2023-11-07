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
import posthogClient from "posthog-js";

interface AuthenticationContextInterface {
  getSession: () => void;
  createSession: (
    access_token: string,
    refresh_token: string
  ) => Promise<Session | null>;
  signOut: () => void;
  fetchProfile: () => void;
  profile: Profile | null;
  session: Session | null;
}

export const AuthenticationContext =
  createContext<AuthenticationContextInterface>({
    createSession: () => null,
    getSession: () => {},
    signOut: () => {},
    fetchProfile: () => {},
    profile: null,
    session: null,
  });

export const useAuthenticationContext = () => useContext(AuthenticationContext);

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

  const fetchProfileFromUserId = async (user_id: string) => {
    if (webFeaturesDisabled) return null;
    try {
      let { data: profile, error } = await supabaseClient
        .from("profiles")
        .select("*")
        .eq("id", user_id)
        .single();

      if (error) throw error;
      if (profile) {
        setProfile(profile);
        return profile;
      } else {
        return undefined;
      }
    } catch (e) {
      console.log(e);
      return undefined;
    }
  };

  const fetchCurrentUserProfile = async () => {
    if (profile && profile.id) {
      await fetchProfileFromUserId(profile.id);
    } else if (session && session.user && session.user.id) {
      await fetchProfileFromUserId(session.user.id);
    } else {
      console.log("no profile or session user id found to fetchProfile");
    }
  };

  const signOut = async () => {
    // if (webFeaturesDisabled) return null;
    await supabaseClient.auth.signOut();
    posthogClient.reset();
    setSession(null);
    setProfile(null);
  };

  useEffect(() => {
    //update profile if sesssion exists
    fetchCurrentUserProfile();
  }, [session]);

  const getSession = async () => {
    supabaseClient.auth.getSession().then(({ data: { session } }) => {
      console.log("session found in AuthenticationProvider", session);
      setSession(session);
      posthogClient.identify(session?.user?.id, {
        email: session?.user?.email,
      });
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
      //user has updated password ( most likely )
      if (session?.user?.id) {
        posthogClient.identify(session?.user?.id, {
          email: session?.user?.email,
        });
      }
      if (event === "USER_UPDATED") {
        console.log("USER_UPDATED");
        navigate("/");
      }
      if (event === "SIGNED_IN") {
        console.log("SIGNED_IN");
        navigate("/");
      }

      console.log("event", JSON.stringify(event, null, 3));
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

  return (
    <AuthenticationContext.Provider
      value={{
        signOut,
        profile,
        fetchProfile: fetchCurrentUserProfile,
        session,
        getSession,
        createSession: createSessionFromUrl,
      }}
    >
      {children}
    </AuthenticationContext.Provider>
  );
};
