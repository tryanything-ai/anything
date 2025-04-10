"use client";

import { Separator } from "@repo/ui/components/ui/separator";
import { useParams } from "next/navigation";
import DashboardTitleWithNavigation from "@/components/workflows/dahsbloard-title-with-navigation";
import { TaskRow, TimeUnit } from "@repo/anything-api";
import api from "@repo/anything-api";
import { TaskTable } from "@/components/tasks/task-table";
import { TaskChart } from "@/components/tasks/task-chart";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";
import useSWR from "swr";
import { useState, useCallback, useEffect } from "react";

const DEFAULT_PAGINATION = {
  page: 1,
  page_size: 20,
  total: 0,
};

export default function MainDashboard(): JSX.Element {
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const [currentPage, setCurrentPage] = useState(1);
  const [allTasks, setAllTasks] = useState<TaskRow[]>([]);
  const pageSize = 20;

  // Reset tasks and page when account changes
  useEffect(() => {
    setAllTasks([]);
    setCurrentPage(1);
  }, [selectedAccount?.account_id]);

  const endDate = new Date().toISOString();
  const startDate = new Date(
    new Date().setDate(new Date().getDate() - 30),
  ).toISOString();
  const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  const tasksFetcher = async () => {
    if (!selectedAccount)
      return {
        data: [],
        pagination: DEFAULT_PAGINATION,
      };
    const supabase = await createClient();
    return api.tasks.getTasks(supabase, selectedAccount.account_id, {
      page: currentPage,
      page_size: pageSize,
    });
  };

  const chartDataFetcher = async () => {
    if (!selectedAccount) return null;
    const supabase = await createClient();
    const chartData = await api.charts.getTasksChartForAccount(
      supabase,
      selectedAccount.account_id,
      startDate,
      endDate,
      TimeUnit.Day,
      encodeURIComponent(timezone),
    );
    return chartData.chartData;
  };

  const {
    data: tasksData = {
      data: [],
      pagination: DEFAULT_PAGINATION,
    },
    isLoading: isTasksLoading,
  } = useSWR(
    selectedAccount ? [`tasks`, selectedAccount.account_id, currentPage] : null,
    tasksFetcher,
    {
      revalidateOnFocus: true,
      onSuccess: (data) => {
        if (data?.data) {
          setAllTasks((prev) => {
            // Create a Set of existing task IDs for efficient lookup
            const existingIds = new Set(prev.map((t) => t.task_id));
            // Filter out any duplicates from the new data
            const newTasks = data.data.filter(
              (t) => !existingIds.has(t.task_id),
            );
            return [...prev, ...newTasks];
          });
        }
      },
    },
  );

  const { data: chartData } = useSWR(
    selectedAccount ? [`chartData`, selectedAccount.account_id] : null,
    chartDataFetcher,
    {
      revalidateOnFocus: true,
    },
  );

  const handlePageChange = useCallback((newPage: number) => {
    setCurrentPage(newPage);
  }, []);

  return (
    <>
      <div className="space-y-6 w-full">
        <div className="flex flex-col gap-y-4 w-full mx-auto text-center">
          <TaskChart chartData={chartData} />
        </div>

        <div className="border rounded-md w-full">
          <TaskTable accountId={selectedAccount?.account_id ?? ""} />
        </div>
      </div>
    </>
  );
}
