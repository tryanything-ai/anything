import { Outlet, Link, useLocation } from "react-router-dom";
import {
  VscHome,
  VscRepoForked,
  VscSettingsGear,
  VscTable,
  VscHubot,
  VscComment,
  VscReferences,
} from "react-icons/vsc";
import clsx from "clsx";

export default function Layout() {
  const location = useLocation();

  const selectedClass = "text-primary";
  const defaultClass = "w-10 h-7";
  const linkClass = "hover:text-primary w-10 h-10";

  return (
    <div className="flex flex-row min-w-screen min-h-screen overflow-hidden overscroll-none">
      <div className="w-14 flex flex-col gap-3 px-2 pt-3">
        <Link className={linkClass} to="/">
          <VscHome
            className={clsx(defaultClass, {
              [selectedClass]: location.pathname === "/",
            })}
          />
        </Link>
        <Link className={linkClass} to="/flows">
          <VscRepoForked
            className={clsx(defaultClass, {
              [selectedClass]: location.pathname.includes("/flows"),
            })}
          />
        </Link>
        {/* <Link className={linkClass} to="/chats">
          <VscComment
            className={clsx(defaultClass, {
              [selectedClass]: location.pathname.includes("/chats"),
            })}
          />
        </Link> */}
        {/* <Link className={linkClass} to="/models">
          <VscHubot
            className={clsx(defaultClass, {
              [selectedClass]: location.pathname.includes("/model"),
            })}
          />
        </Link> */}
        <Link className={linkClass} to="/vectors">
          <VscReferences
            className={clsx(defaultClass, {
              [selectedClass]: location.pathname.includes("/vector"),
            })}
          />
        </Link>
        <Link className={linkClass} to="/tables">
          <VscTable
            className={clsx(defaultClass, {
              [selectedClass]: location.pathname.includes("/table"),
            })}
          />
        </Link>
        <div className="flex-grow" />
        <Link className={linkClass} to="/settings">
          <VscSettingsGear
            className={clsx(defaultClass, {
              [selectedClass]: location.pathname === "/settings",
            })}
          />
        </Link>
      </div>
      <div className="w-screen h-screen">
        <Outlet />
      </div>
    </div>
  );
}
