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

export default function MainDashboard(): JSX.Element {
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const endDate = new Date().toISOString();
  const startDate = new Date(
    new Date().setDate(new Date().getDate() - 30),
  ).toISOString();
  const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  const tasksFetcher = async () => {
    if (!selectedAccount) return [];
    const supabase = await createClient();
    return api.tasks.getTasks(supabase, selectedAccount.account_id);
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

  const { data: tasks = [] } = useSWR(
    selectedAccount ? [`tasks`, selectedAccount.account_id] : null,
    tasksFetcher,
    {
      revalidateOnFocus: true,
    }
  );

  const { data: chartData } = useSWR(
    selectedAccount ? [`chartData`, selectedAccount.account_id] : null,
    chartDataFetcher,
    {
      revalidateOnFocus: true,
    }
  );

  return (
    <>
      <div className="space-y-6 w-full">
        <div className=" flex flex-col gap-y-4 w-full  mx-auto text-center">
          <TaskChart chartData={chartData} />
        </div>

        <div className="border rounded-md flex flex-col gap-y-4  h-full w-full items-center justify-center content-center mx-auto text-center">
          <TaskTable tasks={tasks} />
        </div>
      </div>
    </>
  );
}
