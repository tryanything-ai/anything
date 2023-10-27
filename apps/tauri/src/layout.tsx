import {
  VscHome,
  VscReferences,
  VscRepoForked,
  VscSettingsGear,
  VscTable,
  VscGlobe,
} from "react-icons/vsc";
import { Outlet } from "react-router-dom";
import NavLink from "./components/navlink";

export default function Layout() {
  return (
    <div className="flex flex-row w-screen h-screen text-slate-12 font-sans">
      <div className="w-14 flex flex-col gap-3 pt-3 pb-2 border-r border-slate-6">
        <NavLink link="/" icon={VscHome} />
        <NavLink link="/templates" icon={VscGlobe} />
        <NavLink link="/flows" icon={VscRepoForked} />
        <NavLink link="/vectors" icon={VscReferences} />
        <NavLink link="/tables" icon={VscTable} />
        <div className="flex-grow" />
        <NavLink link="/settings" icon={VscSettingsGear} />
      </div>
      <div className="w-full h-full flex flex-col">
        <Outlet />
      </div>
    </div>
  );
}
