import React from "react";

interface IconProps {
  icon: string; // Expected to be SVG content
  className?: string; // Optional className property
}

export function removeWidthHeight(svgString: string) {
  let cleanedSvgString = svgString
    .replace(/\s*width="[^"]*"/, "")
    .replace(/\s*height="[^"]*"/, "");

  // Check if fill attribute already exists
  if (!cleanedSvgString.includes('fill="')) {
    // Add fill attribute with currentValue
    cleanedSvgString = cleanedSvgString.replace(
      "<svg",
      '<svg fill="currentColor"'
    );
  }

  return cleanedSvgString;
}

export const SvgRenderer: React.FC<IconProps> = ({ icon, className }) => {
  const combinedClasses = `${className ?? ""}`;
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
  return <span className="bg-red-300 p-2 border rounded">🚫</span>;
};

export const BaseNodeIcon: React.FC<IconProps> = ({ icon, className }) => {
  return (
    <div
      className={`h-14 w-14 p-2 rounded-md bg-white bg-opacity-30 ${
        className ?? ""
      }`}
    >
      <SvgRenderer className={`${className} w-full h-full`} icon={icon} />
    </div>
  );
};
