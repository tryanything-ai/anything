import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@repo/ui/components/ui/table";
import { TaskRow, TimeUnit } from "@repo/anything-api";
import { format } from "date-fns";
import {
  DurationBadge,
  TaskStatusBadge,
  TriggerBadge,
} from "../studio/forms/testing/task-status-badges";
import { ActionType } from "@/types/workflows";
import { useState, useEffect } from "react";
import { ResultComponent } from "@/components/studio/forms/testing/task-card";
import { ChevronDown, ChevronRight, Search } from "lucide-react";
import { useInView } from "react-intersection-observer";
import { createClient } from "@/lib/supabase/client";
import api from "@repo/anything-api";
import useSWR from "swr";
import { Input } from "@repo/ui/components/ui/input";
import { useDebounce } from "@/hooks/use-debounce";

interface TaskTableProps {
  accountId: string;
  workflowId?: string;
}

const DEFAULT_PAGINATION = {
  page: 1,
  page_size: 20,
  total: 0,
};

export function TaskTable({
  accountId,
  workflowId,
}: TaskTableProps): JSX.Element {
  const [expandedTaskIds, setExpandedTaskIds] = useState<Set<string>>(
    new Set(),
  );
  const [currentPage, setCurrentPage] = useState(1);
  const [allTasks, setAllTasks] = useState<TaskRow[]>([]);
  const [searchTerm, setSearchTerm] = useState("");
  const debouncedSearch = useDebounce(searchTerm, 300);

  const { ref: loadMoreRef, inView } = useInView({
    threshold: 0.1,
    rootMargin: "50px",
  });
  const pageSize = 20;

  // Reset tasks and page when account, workflow, or search changes
  useEffect(() => {
    setAllTasks([]);
    setCurrentPage(1);
  }, [accountId, workflowId, debouncedSearch]);

  const tasksFetcher = async () => {
    const supabase = await createClient();
    if (workflowId) {
      return api.tasks.getTasksForWorkflow(supabase, accountId, workflowId, {
        page: currentPage,
        page_size: pageSize,
        search: debouncedSearch || undefined,
      });
    } else {
      return api.tasks.getTasks(supabase, accountId, {
        page: currentPage,
        page_size: pageSize,
        search: debouncedSearch || undefined,
      });
    }
  };

  const {
    data: tasksData = { data: [], pagination: DEFAULT_PAGINATION },
    isLoading,
    isValidating,
  } = useSWR(
    [`tasks`, accountId, workflowId, currentPage, debouncedSearch],
    tasksFetcher,
    {
      revalidateOnFocus: true,
      onSuccess: (data) => {
        if (data?.data) {
          setAllTasks((prev) => {
            if (currentPage === 1) {
              return data.data;
            }
            const existingIds = new Set(prev.map((t) => t.task_id));
            const newTasks = data.data.filter(
              (t) => !existingIds.has(t.task_id),
            );
            return [...prev, ...newTasks];
          });
        }
      },
    },
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

  useEffect(() => {
    if (
      inView &&
      !isValidating &&
      allTasks.length < tasksData.pagination.total
    ) {
      console.log("Loading more tasks, current page:", currentPage);
      setCurrentPage((prev) => prev + 1);
    }
  }, [
    inView,
    isValidating,
    allTasks.length,
    tasksData.pagination.total,
    currentPage,
  ]);

  return (
    <div className="flex flex-col gap-2 bg-gray-100">
      <div className="relative mt-2 mx-2">
        <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-gray-500" />
        <Input
          type="text"
          placeholder="Search tasks..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="pl-9"
        />
      </div>
      {/* <div className="bg-black h-2" /> */}
      <Table className="bg-white">
        <TableHeader className="bg-gray-100">
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
          {allTasks.length === 0 && isLoading ? (
            <TableRow>
              <TableCell colSpan={7} className="text-center" height={100}>
                <div className="animate-pulse">Loading tasks...</div>
              </TableCell>
            </TableRow>
          ) : allTasks.length === 0 ? (
            <TableRow>
              <TableCell colSpan={7} className="text-center" height={100}>
                {searchTerm ? "No matching tasks found" : "No completed tasks"}
              </TableCell>
            </TableRow>
          ) : (
            <>
              {allTasks.map((task) => (
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
                      <TriggerBadge
                        is_trigger={task.type == ActionType.Trigger}
                      />
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
                          <div suppressHydrationWarning className="h-full">
                            <ResultComponent
                              collapsed={true}
                              result={task.context}
                              collapseStringsAfterLength={1000}
                            />
                          </div>
                        </div>
                        <div className="p-4 text-left h-full">
                          <div className="text-md font-semibold mb-2">
                            Results:
                          </div>
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
              ))}
              <tr ref={loadMoreRef} className="h-20">
                <td colSpan={7} className="p-4 text-center">
                  {isValidating ? (
                    <div className="animate-pulse">Loading more tasks...</div>
                  ) : allTasks.length < tasksData.pagination.total ? (
                    <div className="text-sm text-gray-500">
                      Scroll to load more ({allTasks.length} of{" "}
                      {tasksData.pagination.total})
                    </div>
                  ) : (
                    <div className="text-sm text-gray-500">
                      All tasks loaded ({allTasks.length} total)
                    </div>
                  )}
                </td>
              </tr>
            </>
          )}
        </TableBody>
      </Table>
    </div>
  );
}
