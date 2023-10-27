import {
  VscHome,
  VscReferences,
  VscRepoForked,
  VscSettingsGear,
  VscTable,
} from "react-icons/vsc";
import { Outlet } from "react-router-dom";
import NavLink from "./components/navlink";

export default function Layout() {
  return (
    <div className="flex flex-row min-w-screen min-h-screen overflow-hidden overscroll-none text-slate-12 font-sans">
      <div className="w-14 flex flex-col gap-3 pt-3 pb-2 border-r border-slate-6">
        <NavLink link="/" icon={VscHome} />
        <NavLink link="/flows" icon={VscRepoForked} />
        <NavLink link="/vectors" icon={VscReferences} />
        <NavLink link="/tables" icon={VscTable} />
        <div className="flex-grow" />
        <NavLink link="/settings" icon={VscSettingsGear} />
      </div>
      <div className="w-screen h-screen">
        <Outlet />
      </div>
    </div>
  );
}
