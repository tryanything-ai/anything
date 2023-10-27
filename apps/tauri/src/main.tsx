import "./styles.css";
import "ui/styles.css";
// import "unfonts.css";

import posthogClient from "posthog-js";
import { PostHogProvider } from "posthog-js/react";
import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import { AuthenticationProvider } from "./context/AuthenticaionProvider";
import { LocalFileProvider } from "./context/LocalFileProvider";
import { MarketplaceProvider } from "./context/MarketplaceProvider";
import { ModelProvider } from "./context/ModelsProvider";
import { NotificationsProvider } from "./context/NotificationProvider";
import { SettingsProvider } from "./context/SettingsProvider";
import { SqlProvider } from "./context/SqlProvider";
// Contexts
import { TauriProvider } from "./context/TauriProvider";
import ErrorPage from "./error-page";
import Layout from "./layout";
import ChatInterface from "./routes/chatInterface";
import Chats from "./routes/chats";
import FlowEditor from "./routes/flowEditor";
import Flows from "./routes/flows";
// Routes
import Home from "./routes/home";
import Login from "./routes/login";
import Models from "./routes/models";
import Profile from "./routes/profile";
import Settings from "./routes/settings";
import TableData from "./routes/tableData";
import Tables from "./routes/tables";
import Template from "./routes/template";
import Templates from "./routes/templates";
import Vectors from "./routes/vectors";

const VITE_PUBLIC_POSTHOG_KEY = import.meta.env.VITE_PUBLIC_POSTHOG_KEY;
const VITE_PUBLIC_POSTHOG_HOST = import.meta.env.VITE_PUBLIC_POSTHOG_HOST;

if (import.meta.env.mode === "production") {
  console.log("Initializing PostHog in production");
  posthogClient.init(VITE_PUBLIC_POSTHOG_KEY, {
    api_host: VITE_PUBLIC_POSTHOG_HOST,
  });
} else {
  // console.log("Initializing PostHog in development");
  // console.log("import.meta.env", import.meta.env);
}

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    errorElement: <ErrorPage />,
    children: [
      {
        index: true,
        element: <Home />,
      },
      {
        path: "/flows",
        element: <Flows />,
      },
      {
        path: "/templates",
        element: <Templates />,
      },
      {
        path: "/templates/:slug",
        element: <Template />,
      },
      {
        path: "/models",
        element: <Models />,
      },
      {
        path: "/vectors",
        element: <Vectors />,
      },
      {
        path: "flows/:flow_name",
        element: <FlowEditor />,
      },
      {
        path: "/chats",
        element: <Chats />,
      },
      {
        path: "/chats/:flow_id",
        element: <ChatInterface />,
      },
      {
        path: "/tables",
        element: <Tables />,
      },
      {
        path: "/tables/:table",
        element: <TableData />,
      },
      {
        path: "/login",
        element: <Login />,
      },
      {
        path: "/settings",
        element: <Settings />,
      },
      {
        path: "/settings/profile",
        element: <Profile />,
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <SettingsProvider>
      <AuthenticationProvider>
        <MarketplaceProvider>
          <NotificationsProvider>
            <PostHogProvider client={posthogClient}>
              <TauriProvider>
                <LocalFileProvider>
                  <ModelProvider>
                    <SqlProvider>
                      <RouterProvider router={router} />
                    </SqlProvider>
                  </ModelProvider>
                </LocalFileProvider>
              </TauriProvider>
            </PostHogProvider>
          </NotificationsProvider>
        </MarketplaceProvider>
      </AuthenticationProvider>
    </SettingsProvider>
  </React.StrictMode>
);
