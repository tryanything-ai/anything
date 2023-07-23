import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Layout from "./layout";
import ErrorPage from "./error-page";
import Home from "./routes/home";
import Settings from "./routes/settings";
import Flows from "./components/header";
import { TauriProvider } from "./context/TauriProvider";
import { SettingsProvider } from "./context/SettingsProvider";
import { LocalFileProvider } from "./context/LocalFileProvider";
import { TomlFlowProvider } from "./context/TomlFlowProvider";
//TODO: Move to managing the state ourselves
import { ReactFlowProvider } from "reactflow";
import TomlEditor from "./routes/tomlEditor";
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
        path: "/:id/drag",
        element: <FlowEditor />,
      },
      {
        path: "/:id/toml",
        element: <TomlEditor />,
      },
      // {
      //   path: "/flows",
      //   element: <Flows />,
      //   children: [
      //     {
      //       path: "/flows/:id",
      //       element: <FlowEditor />,
      //     },
      //     {
      //       path: "/flows/:id/toml",
      //       element: <TomlEditor />,
      //     },
      //   ],
      // },
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
        <TomlFlowProvider>
          <SettingsProvider>
            <ReactFlowProvider>
              <RouterProvider router={router} />
            </ReactFlowProvider>
          </SettingsProvider>
        </TomlFlowProvider>
      </LocalFileProvider>
    </TauriProvider>
  </React.StrictMode>
);
