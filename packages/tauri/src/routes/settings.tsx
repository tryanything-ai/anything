import React from "react";
import themes from "../utils/themes";
import { useSettingsContext } from "../context/SettingsProvider";
import { Link } from "react-router-dom";
import clsx from "clsx";

export default function Settings() {
  const {
    setTheme,
    theme: currentTheme,
    setWebFeaturesDisabled,
    webFeaturesDisabled,
  } = useSettingsContext();

  const text = `This is a master shutoff for all features that have are web based.
  - Find Templates from Marketplace
   - Product Analytics
    - Crash Reporting Analytics`;

  const newText = text.split("\n").map((str, index) => (
    <React.Fragment key={index}>
      {str}
      <br />
    </React.Fragment>
  ));
  return (
    <div className="flex flex-col h-full w-full p-6">
      <Link to="/settings/profile" className="btn btn-primary m-1 ml-4">
        Edit Profile
      </Link>
      <div className="form-control w-52">
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
      <div>{newText}</div>
      <div className="dropdown mt-2">
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
