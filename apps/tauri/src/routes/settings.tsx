import clsx from "clsx";
import React from "react";
import { Link } from "react-router-dom";

import { useSettingsContext } from "../context/SettingsProvider";
import themes from "../utils/themes";
import PageLayout from "../pageLayout";
import { useAuthenticaionContext } from "../context/AuthenticaionProvider";

export default function Settings() {
  const {
    setTheme,
    theme: currentTheme,
    setWebFeaturesDisabled,
    webFeaturesDisabled,
  } = useSettingsContext();
  const { signOut, session } = useAuthenticaionContext();

  return (
    <PageLayout>
      <div className="flex flex-col gap-4">
        <Link to="/settings/profile" className="btn btn-primary w-72">
          Edit Profile
        </Link>
        {session ? (
          <div onClick={signOut} className="btn btn-ghost w-72">
            Sign Out
          </div>
        ) : (
          <Link to="/login" className="btn btn-primary w-72">
            Sign In
          </Link>
        )}

        <div className="form-control w-72">
          <label className="cursor-pointer label">
            <span className="label-text  text-2xl">Web Features</span>
            <input
              type="checkbox"
              onChange={() => {
                setWebFeaturesDisabled(!webFeaturesDisabled);
              }}
              className="toggle toggle-primary"
              checked={!webFeaturesDisabled}
            />
          </label>
        </div>

        {/* <div>{newText}</div> */}
        {/* <div className="dropdown mt-2">
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
      </div> */}
      </div>
    </PageLayout>
  );
}
