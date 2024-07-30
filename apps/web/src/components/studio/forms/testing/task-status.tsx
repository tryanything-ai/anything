// TaskStatus.tsx
import React from "react";
import { DurationBadge, TaskStatusBadge } from "./task-status-badges";

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
      <TaskStatusBadge status={status} />
      <DurationBadge started_at={started_at} ended_at={ended_at} />
    </div>
  );
};

export default TaskStatusComponent;
