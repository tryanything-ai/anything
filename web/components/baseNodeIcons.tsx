import React from "react";

interface Props {
  icon: string; // Expected to be SVG content
  className?: string; // Optional className property
}

const BaseNodeOrIcon: React.FC<Props> = ({ icon, className }) => {
  const combinedClasses = `w-full h-full ${className ?? ""}`;

  // If it's an SVG content, render the SVG
  if (icon.startsWith("<svg")) {
    return (
      <div
        className={combinedClasses}
        // style={{ width: "100%", height: "100%" }}
        dangerouslySetInnerHTML={{ __html: icon }}
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
