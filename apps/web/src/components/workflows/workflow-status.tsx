import React from "react";

import { Badge } from "@repo/ui/badge";

const WorkflowStatusComponent = ({
  active,
  draft,
}: {
  active: boolean;
  draft: boolean;
}) => {
  return (
    <div className="flex gap-2">
      {draft ? <DraftBadge /> : <WorfklowStatusBadge active={active} />}
    </div>
  );
};

export default WorkflowStatusComponent;

const statusStyles: Record<string, string> = {
  true: "bg-green-200 text-green-800 hover:bg-green-200",
  false: "bg-red-200 text-red-800 hover:bg-red-200",
};

export const WorfklowStatusBadge = ({ active }: { active: boolean }) => {
  return (
    <div
      className={`inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 ${active ? statusStyles["true"] : statusStyles["false"]}`}
    >
      {active ? "ON" : "OFF"}
    </div>
  );
};

export const DraftBadge = () => {
  return (
    <div
      className={`inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 h-10 px-4 py-2 bg-gray-200 text-gray-800 hover:bg-gray-200`}
    >
      DRAFT
    </div>
  );
};
