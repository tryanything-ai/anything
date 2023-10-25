import React from "react";

interface Props {
  icon: string; // Expected to be SVG content
  className?: string; // Optional className property
}

function removeWidthHeight(svgString: string) {
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

const BaseNodeOrIcon: React.FC<Props> = ({
  icon,
  className
}) => {
  const combinedClasses = `w-full h-full ${className ?? ""}`;
  let cleanIcon = icon;
  
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
  return <span className="bg-red-500 p-2 border rounded">Invalid</span>;
};

const BaseNodeIcon: React.FC<Props> = ({ icon, className }) => {
  return (
    <div
      className={` h-14 w-14 p-2 rounded-md bg-white bg-opacity-30 ${
        className ?? ""
      }`}
    >
      <BaseNodeOrIcon icon={icon} className={className} />
    </div>
  );
};

export default BaseNodeIcon;
