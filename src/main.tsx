import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Layout from "./layout";
import ErrorPage from "./error-page";
import Home from "./routes/home";
import Settings from "./routes/settings";
import Flows from "./routes/flows";
import { TauriProvider } from "./context/TauriProvider";
import { SettingsProvider } from "./context/SettingsProvider";
import { LocalFileProvider } from "./context/LocalFileProvider";
import YamlEditor from "./routes/yamlEditor";
import FlowEditor from "./routes/flowEditor";
import "./styles.css";

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
        children: [
          {
            path: "/flows/:id",
            element: <FlowEditor />,
          },
          {
            path: "/flows/:id/yaml",
            element: <YamlEditor />,
          },
        ],
      },
      {
        path: "/settings",
        element: <Settings />,
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <TauriProvider>
      <LocalFileProvider>
        <SettingsProvider>
          <RouterProvider router={router} />
        </SettingsProvider>
      </LocalFileProvider>
    </TauriProvider>
  </React.StrictMode>
);
