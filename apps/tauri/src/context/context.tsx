import { ReactNode } from "react";

import posthogClient from "posthog-js";
import { PostHogProvider } from "posthog-js/react";

// Contexts
import { AuthenticationProvider } from "../context/AuthenticaionProvider";
import { LocalFileProvider } from "../context/LocalFileProvider";
import { MarketplaceProvider } from "../context/MarketplaceProvider";
import { ModelProvider } from "../context/ModelsProvider";
import { NotificationsProvider } from "../context/NotificationProvider";
import { SettingsProvider } from "../context/SettingsProvider";
import { SqlProvider } from "../context/SqlProvider";
import { TauriProvider } from "../context/TauriProvider";
import { DeeplinkProvider } from "../context/DeeplinkProvider";

const VITE_PUBLIC_POSTHOG_KEY = import.meta.env.VITE_PUBLIC_POSTHOG_KEY;
const VITE_PUBLIC_POSTHOG_HOST = import.meta.env.VITE_PUBLIC_POSTHOG_HOST;

if (import.meta.env.mode === "production") {
  console.log("Initializing PostHog in production");
  posthogClient.init(VITE_PUBLIC_POSTHOG_KEY, {
    api_host: VITE_PUBLIC_POSTHOG_HOST,
    
  });
} 

const Context = ({ children }: { children: ReactNode }) => {
  return (
    <DeeplinkProvider>
      <SettingsProvider>
        <AuthenticationProvider>
          <MarketplaceProvider>
            <NotificationsProvider>
              <PostHogProvider client={posthogClient}>
                <TauriProvider>
                  <LocalFileProvider>
                    <ModelProvider>
                      <SqlProvider>{children}</SqlProvider>
                    </ModelProvider>
                  </LocalFileProvider>
                </TauriProvider>
              </PostHogProvider>
            </NotificationsProvider>
          </MarketplaceProvider>
        </AuthenticationProvider>
      </SettingsProvider>
    </DeeplinkProvider>
  );
};

export default Context;
