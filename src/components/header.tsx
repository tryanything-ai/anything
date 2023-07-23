import { useEffect, useState } from "react";
import { Outlet, Link, useLocation } from "react-router-dom";
import { useLocalFileContext } from "../context/LocalFileProvider";
import {
  VscLayoutSidebarRightOff,
  VscRepoForked,
  VscCode,
} from "react-icons/vsc";
import { useNavigationContext } from "../context/NavigationProvider";

export default function Header() {
  const [editor, setEditor] = useState<string>("wysiwyg");

  const { setCurrentFlow, currentFlow } = useLocalFileContext();
  const { sidePanel, setSidePanel } = useNavigationContext();
  const location = useLocation();

  useEffect(() => {
    let splitLocation = location.pathname.split("/");
    console.log("splitLocation", splitLocation);
    setEditor(splitLocation[2]);
    setCurrentFlow(decodeURIComponent(splitLocation[1]));
  }, [location]);

  return (
    <div className="w-full z-10 bg-primary pl-2 text-white overflow-hidden">
      <div className="flex flex-row">
        <div className="">flows/{currentFlow}</div>
        <div className="flex-grow" />
        <button onClick={() => setSidePanel(!sidePanel)}>
          <VscLayoutSidebarRightOff className="mr-2 h-5 w-5" />
        </button>
        <div>
          <Link to={`/${currentFlow}${editor === "drag" ? "/toml" : "/drag"}`}>
            {editor === "drag" ? (
              <VscCode className="mr-2 h-5 w-5" />
            ) : (
              <VscRepoForked className="mr-2 h-5 w-5" />
            )}
          </Link>
        </div>
      </div>
      {/* <div className="-mt-6">
        <Outlet />
      </div> */}
    </div>
  );
}
