import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@repo/ui/components/ui/table";
import { TaskRow } from "@repo/anything-api/testing";
import { format } from "date-fns";
import {
  DurationBadge,
  TaskStatusBadge,
  TriggerBadge,
} from "../studio/forms/testing/task-status-badges";
import { ActionType } from "@/types/workflows";

export function TaskTable({ tasks }: { tasks: TaskRow[] }): JSX.Element {
  return (
    <Table>
      <TableHeader>
        <TableRow>
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
            <TableCell colSpan={6} className="text-center" height={100}>
              No completed tasks
            </TableCell>
          </TableRow>
        ) : (
          tasks.map((task) => (
            <TableRow key={task.task_id}>
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
          ))
        )}
      </TableBody>
    </Table>
  );
}
