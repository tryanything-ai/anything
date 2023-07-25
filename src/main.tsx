import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Layout from "./layout";
import ErrorPage from "./error-page";
import Home from "./routes/home";
import Settings from "./routes/settings";
import Tables from "./routes/tables";
import { TauriProvider } from "./context/TauriProvider";
import { SettingsProvider } from "./context/SettingsProvider";
import { LocalFileProvider } from "./context/LocalFileProvider";
// import { TomlFlowProvider } from "./context/TomlFlowProvider";
// import { FlowProvider } from "./context/FlowProvider";

// import { ReactFlowProvider } from "reactflow";
// import TomlEditor from "./routes/tomlEditor";
import FlowEditor from "./routes/flowEditor";
import TableData from "./routes/tableData";
import Flows from "./routes/flows";
import "./styles.css";
import { SqlProvider } from "./context/SqlProvider";

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
        path: "flows/:flow_name",
        element: <FlowEditor />,
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
        <SqlProvider>
          {/* <FlowProvider> */}
          {/* <TomlFlowProvider> */}
          <SettingsProvider>
            <RouterProvider router={router} />
          </SettingsProvider>
          {/* </TomlFlowProvider> */}
          {/* </FlowProvider> */}
        </SqlProvider>
      </LocalFileProvider>
    </TauriProvider>
  </React.StrictMode>
);
