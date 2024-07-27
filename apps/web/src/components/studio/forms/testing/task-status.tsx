// TaskStatus.tsx
import React from "react";

import { Badge } from "@/components/ui/badge";
import { RefreshCw, Clock } from "lucide-react";
import { formatTimeDifference } from "@/lib/utils";

const statusStyles: Record<string, string> = {
  pending: "bg-gray-200 text-gray-800 hover:bg-gray-200",
  waiting: "bg-yellow-200 text-yellow-800, hover:bg-yellow-200",
  running: "bg-blue-200 text-blue-800 hover:bg-blue-200",
  completed: "bg-green-200 text-green-800 hover:bg-green-200",
  failed: "bg-red-200 text-red-800 hover:bg-red-200",
  canceled: "bg-purple-200 text-purple-800 hover:bg-purple-200",
};

const TaskStatusComponent = ({
  status,
  started_at,
  ended_at,
}: {
  status: string;
  started_at?: string;
  ended_at?: string;
}) => {
  return (
    <div className="flex gap-2">
      <Badge
        className={`inline-flex items-center px-3 py-1 rounded-full ${statusStyles[status]}`}
      >
        {status === "running" && (
          <RefreshCw size={16} className="mr-2 animate-spin" />
        )}
        {status}
      </Badge>
      {started_at && ended_at && (
        <Badge className="inline-flex items-center px-3 py-1 rounded-full bg-gray-100 text-gray-800 hover:bg-gray-100">
          <Clock size={16} className="mr-2" />{" "}
          {formatTimeDifference(started_at, ended_at)}
        </Badge>
      )}
    </div>
  );
};

export default TaskStatusComponent;
