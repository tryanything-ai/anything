// TaskStatus.tsx
import React from "react";

// import 'tailwindcss/tailwind.css';
// import { Badge } from '@shadcn/ui';
import { Badge } from "@/components/ui/badge";
import { RefreshCw } from "lucide-react";

const statusStyles: Record<string, string> = {
  pending: "bg-gray-200 text-gray-800 hover:bg-gray-200",
  waiting: "bg-yellow-200 text-yellow-800, hover:bg-yellow-200",
  running: "bg-blue-200 text-blue-800 hover:bg-blue-200",
  completed: "bg-green-200 text-green-800 hover:bg-green-200",
  failed: "bg-red-200 text-red-800 hover:bg-red-200",
  canceled: "bg-purple-200 text-purple-800 hover:bg-purple-200",
};

const TaskStatusComponent = ({ status }: { status: string }) => {
  return (
    <Badge
      className={`inline-flex items-center px-3 py-1 rounded-full ${statusStyles[status]}`}
    >
      {status === "running" && (
        <RefreshCw size={16} className="mr-2 animate-spin" />
      )}
      {status}
    </Badge>
  );
};

export default TaskStatusComponent;
