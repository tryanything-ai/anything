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
}

const BaseNodeOrIcon: React.FC<Props> = ({ icon }) => {
  const iconName = icon;
  const IconComponent = VsIcons[iconName as keyof typeof VsIcons] as
    | IconType
    | undefined;

  // If it's a URL, render an image
  if (isURL(icon)) {
    return (
      <img
        className="h-12 w-12"
        src={icon}
        alt="User provided icon"
        width="12"
        height="12"
      />
    );
  }

  // If it's a valid icon name, render the icon
  if (IconComponent) {
    return <IconComponent className="h-12 w-12" />;
  }

  // If it's neither, render a fallback (e.g., a default icon or message)
  return <span className="bg-red-500 p-2 border rounded ">Invalid</span>;
};

const BaseNodeIcon: React.FC<Props> = ({ icon }) => {
  return (
    <div className="flex justify-center items-center h-14 w-14 p-2 rounded-md bg-secondary text-secondary-content">
      <BaseNodeOrIcon icon={icon} />
    </div>
  );
};

export default BaseNodeIcon;
