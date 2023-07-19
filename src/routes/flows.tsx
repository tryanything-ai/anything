import { Outlet, Link, useLocation } from "react-router-dom";
import { AiOutlineFileText, AiOutlineFork } from "react-icons/ai";
import clsx from "clsx";

export default function Flows() {
  const location = useLocation();

  const selectedClass = "text-primary";
  const defaultClass = "w-10 h-7";
  const linkClass = "hover:text-primary w-10 h-10";

  return (
    <div className="flex flex-row min-w-screen min-h-screen overflow-hidden">
      OuterShell
      <div className="w-screen h-screen">
        <Outlet />
      </div>
    </div>
  );
}
