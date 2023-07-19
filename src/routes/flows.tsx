import { useEffect, useState } from "react";
import { Outlet, Link, useLocation } from "react-router-dom";
import { AiOutlineFileText, AiOutlineFork } from "react-icons/ai";
import clsx from "clsx";
import { useLocalFileContext } from "../context/LocalFileProvider";

export default function Flows() {
  const { setCurrentFlow } = useLocalFileContext();
  const location = useLocation();
  const selectedClass = "text-primary";
  const defaultClass = "w-10 h-7";
  const linkClass = "hover:text-primary w-10 h-10";

  //TODO: load state from files
  useEffect(() => {
    console.log(
      "flowName / fileName / decodedPath",
      decodeURIComponent(location.pathname.split("/")[2])
    );
    setCurrentFlow(decodeURIComponent(location.pathname.split("/")[2]));
  }, [location]);

  return (
    <div className="flex flex-row min-w-screen min-h-screen overflow-hidden">
      <div className="w-screen h-screen">
        <Outlet />
      </div>
    </div>
  );
}
