import { useEffect, useState } from "react";
import { Outlet, Link, useLocation } from "react-router-dom";
import { useLocalFileContext } from "../context/LocalFileProvider";

export default function Flows() {
  const [editor, setEditor] = useState<string>("wysiwyg");
  const { setCurrentFlow, currentFlow } = useLocalFileContext();
  const location = useLocation();

  useEffect(() => {
    let splitLocation = location.pathname.split("/");
    if (splitLocation.length === 4) {
      setEditor("toml");
    } else {
      setEditor("wysiwyg");
    }
    setCurrentFlow(decodeURIComponent(splitLocation[2]));
  }, [location]);

  return (
    <div className="flex flex-col min-w-screen min-h-screen overflow-hidden">
      <div className="flex flex-row z-10">
        <div className="">flows/{currentFlow}</div>
        <div className="flex-grow" />
        <div>
          <Link
            to={`/flows/${currentFlow}${editor === "wysiwyg" ? "/toml" : ""}`}
            className="mr-2"
          >
            {editor === "wysiwyg" ? "toml" : "wysiwyg"}
          </Link>
        </div>
      </div>
      <div className="-mt-6">
        <Outlet />
      </div>
    </div>
  );
}
