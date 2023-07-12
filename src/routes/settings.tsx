import { Outlet } from "react-router-dom";
import themes from "../utils/themes";
import { useSettingsContext } from "../context/SettingsProvider";
import clsx from "clsx";

export default function Settings() {
  const { setTheme, theme: currentTheme } = useSettingsContext();

  return (
    <div className="flex flex-row h-full w-full p-6">
      <div className="dropdown">
        <label tabIndex={0} className="btn m-1">
          Choose Theme
        </label>
        <ul
          tabIndex={0}
          className="dropdown-content z-[1] p-2 shadow bg-base-100 rounded-box max-h-60 w-52 overflow-y-scroll"
        >
          {themes.map((theme) => {
            return (
              <li
                key={theme}
                className={clsx("hover:bg-primary-focus w-ful p-2 rounded-md", {
                  "bg-secondary-focus": theme === currentTheme,
                })}
                onClick={() => {
                  setTheme(theme);
                }}
              >
                {theme}
              </li>
            );
          })}
        </ul>
      </div>
    </div>
  );
}
