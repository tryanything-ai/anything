import React from "react";
import * as VsIcons from "react-icons/vsc";
import { IconType } from "react-icons";

// Determines whether a string is a URL
const isURL = (str: string): boolean => {
  try {
    new URL(str);
    return true;
  } catch (e) {
    return false;
  }
};

interface Props {
  icon: string;
  className?: string; // Optional className property
  imgClassName?: string; // Optional image className property
}

const BaseNodeOrIcon: React.FC<Props> = ({ icon, className }) => {
  const iconName = icon;
  const IconComponent = VsIcons[iconName as keyof typeof VsIcons] as
    | IconType
    | undefined;

  const combinedClasses = `h-12 w-12 ${className ?? ""}`;

  // If it's a URL, render an image
  if (isURL(icon)) {
    return (
      <img className={combinedClasses} src={icon} alt="User provided icon" />
    );
  }

  // If it's a valid icon name, render the icon
  if (IconComponent) {
    return <IconComponent className={combinedClasses} />;
  }

  // If it's neither, render a fallback (e.g., a default icon or message)
  return <span className="bg-red-500 p-2 border rounded ">Invalid</span>;
};

const BaseNodeIcon: React.FC<Props> = ({ icon, className, imgClassName }) => {
  return (
    <div
      className={`flex justify-center items-center h-14 w-14 p-2 rounded-md bg-white bg-opacity-30 ${
        className ?? ""
      }`}
    >
      <BaseNodeOrIcon
        icon={icon}
        className={className}
        imgClassName={imgClassName}
      />
    </div>
  );
};

export default BaseNodeIcon;
