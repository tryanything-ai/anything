import "./styles.css";
import "ui/styles.css";

import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

// Routes
import Home from "./routes/home";
import Login from "./routes/login";
import Models from "./routes/models";
import EditProfile from "./routes/editProfile";
import Settings from "./routes/settings";
import TableData from "./routes/tableData";
import Tables from "./routes/tables";
import Template from "./routes/template";
import Templates from "./routes/templates";
import Vectors from "./routes/vectors";
import ErrorPage from "./error-page";
import Layout from "./layout";
import ChatInterface from "./routes/chatInterface";
import Chats from "./routes/chats";
import FlowEditor from "./routes/flowEditor";
import Flows from "./routes/flows";
import UpdatePassword from "./routes/updatePassword";
import Profile from "./routes/profile"; 

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
        path: "/:username",
        element: <Profile />,
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
        path: "/update-password",
        element: <UpdatePassword />,
      },
      {
        path: "/settings",
        element: <Settings />,
      },
      {
        path: "/settings/profile",
        element: <EditProfile />,
      },
    ],
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    {/* <Context> */}
    <RouterProvider router={router} />
    {/* </Context> */}
  </React.StrictMode>
);
