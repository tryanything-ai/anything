import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { TaskRow } from "@/lib/anything-api/testing";
import { format } from "date-fns";
import {
  DurationBadge,
  TaskStatusBadge,
} from "../studio/forms/testing/task-status-badges";

export function TaskTable({ tasks }: { tasks: TaskRow[] }) {
  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead className="">Task ID</TableHead>
          <TableHead>Task</TableHead>
          <TableHead>Time</TableHead>
          <TableHead>Duration</TableHead>
          <TableHead className="text-right">Status</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {tasks.map((task) => (
          <TableRow key={task.task_id}>
            <TableCell className="text-left font-medium">
              {task.task_id}
            </TableCell>
            <TableCell className="text-left font-medium">
              {task.action_label}
            </TableCell>
            <TableCell>
              {task.started_at
                ? format(new Date(task.started_at), "Pp")
                : "N/A"}
            </TableCell>
            <TableCell>
              <DurationBadge
                started_at={task.started_at}
                ended_at={task.ended_at}
              />
            </TableCell>
            <TableCell className="text-right">
              <TaskStatusBadge status={task.task_status} />
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}
