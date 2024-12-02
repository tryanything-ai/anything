import React from "react";

interface IconProps {
  icon: string; // Expected to be SVG content
  className?: string; // Optional className property
}

export function removeWidthHeight(svgString: string) {
  if (!svgString) {
    return "";
  }

  // console.log("svgString", svgString);

  let cleanedSvgString = svgString
    .replace(/\s*width="[^"]*"/, "")
    .replace(/\s*height="[^"]*"/, "");

  // console.log("cleanedSvgString", cleanedSvgString);

  // Ensure viewBox and preserveAspectRatio are properly set
  if (!cleanedSvgString.includes('preserveAspectRatio="')) {
    cleanedSvgString = cleanedSvgString.replace(
      "<svg",
      '<svg preserveAspectRatio="xMidYMid meet"',
    );
  }

  // Check if fill attribute already exists
  // if (!cleanedSvgString.includes('fill="')) {
  //   // Add fill attribute with currentValue
  //   cleanedSvgString = cleanedSvgString.replace(
  //     "<svg",
  //     '<svg fill="currentColor"'
  //   );
  // }

  // Check if fill attribute already exists
  if (!cleanedSvgString.includes('fill="')) {
    // Add fill attribute with currentValue
    cleanedSvgString = cleanedSvgString.replace(
      "<svg",
      '<svg fill="currentColor"',
    );
  }

  return cleanedSvgString;
}

export const SvgRenderer: React.FC<IconProps> = ({ icon, className }) => {
  // const combinedClasses = `${className ?? ""}`;
  const combinedClasses = `block w-full h-auto leading-none ${className ?? ""}`;
  const cleanIcon = icon;

  const cleanSizedIcon = removeWidthHeight(cleanIcon);

  // If it's an SVG content, render the SVG
  if (cleanSizedIcon.startsWith("<svg")) {
    return (
      <div
        className={combinedClasses}
        dangerouslySetInnerHTML={{ __html: cleanSizedIcon }}
      />
    );
  }

  // If it's not an SVG, render a fallback (e.g., a default icon or message)
  return (
    <span className="border rounded flex items-center justify-center text-gray-400">
      ?
    </span>
  );
};

export const BaseNodeIcon: React.FC<IconProps> = ({ icon, className }) => {
  return (
    <div className={`h-14 w-14 p-1 rounded-md bg-white bg-opacity-30`}>
      <SvgRenderer className={`${className} p-1 w-full h-full`} icon={icon} />
    </div>
  );
};

export const BaseSelectIcon: React.FC<IconProps> = ({ icon, className }) => {
  return (
    <div className={`h-9 w-9 p-1 rounded-md bg-white bg-opacity-30`}>
      <SvgRenderer className={`${className} w-full h-full`} icon={icon} />
    </div>
  );
};
