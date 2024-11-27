import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@repo/ui/components/ui/table";
import { TaskRow } from "@repo/anything-api";
import { format } from "date-fns";
import {
  DurationBadge,
  TaskStatusBadge,
  TriggerBadge,
} from "../studio/forms/testing/task-status-badges";
import { ActionType } from "@/types/workflows";
import { useState } from "react";
import { ResultComponent } from "@/components/studio/forms/testing/task-card";
import { ChevronDown, ChevronRight } from "lucide-react";

export function TaskTable({ tasks }: { tasks: TaskRow[] }): JSX.Element {
  const [expandedTaskIds, setExpandedTaskIds] = useState<Set<string>>(
    new Set(),
  );

  const toggleExpand = (taskId: string) => {
    setExpandedTaskIds((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(taskId)) {
        newSet.delete(taskId);
      } else {
        newSet.add(taskId);
      }
      return newSet;
    });
  };

  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead></TableHead> {/* Expand/Collapse */}
          <TableHead></TableHead> {/* Badges */}
          <TableHead>Task</TableHead>
          <TableHead className="">Task ID</TableHead>
          <TableHead>Start Time</TableHead>
          <TableHead>Duration</TableHead>
          <TableHead className="text-right">Status</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {tasks.length === 0 ? (
          <TableRow>
            <TableCell colSpan={7} className="text-center" height={100}>
              No completed tasks
            </TableCell>
          </TableRow>
        ) : (
          tasks.map((task) => (
            <>
              <TableRow
                key={task.task_id}
                className="cursor-pointer hover:bg-gray-100"
                onClick={() => toggleExpand(task.task_id)}
              >
                <TableCell className="w-4">
                  {task.result ? (
                    expandedTaskIds.has(task.task_id) ? (
                      <ChevronDown className="h-4 w-4 text-gray-500" />
                    ) : (
                      <ChevronRight className="h-4 w-4 text-gray-500" />
                    )
                  ) : null}
                </TableCell>
                <TableCell>
                  <TriggerBadge is_trigger={task.type == ActionType.Trigger} />
                </TableCell>
                <TableCell className="text-left font-medium">
                  {task.action_label}
                </TableCell>
                <TableCell className="text-left font-medium">
                  {task.task_id}
                </TableCell>
                <TableCell className="text-left font-medium">
                  {task.started_at
                    ? format(new Date(task.started_at), "Pp")
                    : "N/A"}
                </TableCell>
                <TableCell className="text-left font-medium">
                  <DurationBadge
                    started_at={task.started_at}
                    ended_at={task.ended_at}
                  />
                </TableCell>
                <TableCell className="text-right">
                  <TaskStatusBadge status={task.task_status} />
                </TableCell>
              </TableRow>
              {expandedTaskIds.has(task.task_id) && task.result && (
                <TableRow>
                  <TableCell colSpan={7} className="bg-gray-50">
                    <div className="p-4 text-left h-full">
                      <div className="text-md font-semibold mb-2">
                        Configuration:
                      </div>
                      {/* Add a client-side only wrapper if needed */}
                      <div suppressHydrationWarning className="h-full">
                        <ResultComponent
                          collapsed={true}
                          result={task.context}
                          collapseStringsAfterLength={1000}
                        />
                      </div>
                    </div>
                    <div className="p-4 text-left h-full">
                      <div className="text-md font-semibold mb-2">Results:</div>
                      {/* Add a client-side only wrapper if needed */}
                      <div suppressHydrationWarning className="h-full">
                        <ResultComponent
                          result={task.result}
                          collapseStringsAfterLength={1000}
                        />
                      </div>
                    </div>
                  </TableCell>
                </TableRow>
              )}
            </>
          ))
        )}
      </TableBody>
    </Table>
  );
}
