import { useEffect, useState } from "react";
import { Outlet, Link, useLocation } from "react-router-dom";
import { useLocalFileContext } from "../context/LocalFileProvider";
import {
  VscLayoutSidebarRightOff,
  VscRepoForked,
  VscCode,
} from "react-icons/vsc";
import { useNavigationContext } from "../context/NavigationProvider";
import { useTomlFlowContext } from "../context/TomlFlowProvider";

export default function Header() {
  // const [editor, setEditor] = useState<string>("wysiwyg");

  const { setCurrentFlow, currentFlow } = useLocalFileContext();
  const { sidePanel, setSidePanel } = useNavigationContext();
  const { editor, setEditor } = useTomlFlowContext();
  const location = useLocation();

  useEffect(() => {
    let splitLocation = location.pathname.split("/");
    console.log("splitLocation", splitLocation);
    setEditor(splitLocation[3]);
    setCurrentFlow(decodeURIComponent(splitLocation[2]));
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
          <Link
            to={`/flows/${currentFlow}${editor === "drag" ? "/toml" : "/drag"}`}
          >
            {editor === "drag" ? (
              <VscCode className="mr-2 h-5 w-5" />
            ) : (
              <VscRepoForked className="mr-2 h-5 w-5" />
            )}
          </Link>
        </div>
      </div>
    </div>
  );
}