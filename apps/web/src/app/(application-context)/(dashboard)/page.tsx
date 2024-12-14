"use client";

import { Separator } from "@repo/ui/components/ui/separator";
import { useParams } from "next/navigation";
import { useEffect, useState } from "react";
import DashboardTitleWithNavigation from "@/components/workflows/dahsbloard-title-with-navigation";
import { TaskRow, TimeUnit } from "@repo/anything-api";
import api from "@repo/anything-api";
import { TaskTable } from "@/components/tasks/task-table";
import { TaskChart } from "@/components/tasks/task-chart";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";
export default function MainDashboard(): JSX.Element {
  const [tasks, setTasks] = useState<TaskRow[]>([]);
  const [chartData, setChartData] = useState<any | undefined>(undefined);
  const {
    accounts: { selectedAccount },
  } = useAnything();

  useEffect(() => {
    const fetchData = async () => {
      if (selectedAccount) {
        let tasks = await api.tasks.getTasks(await createClient(), selectedAccount.account_id);
        console.log("tasks", tasks);
        setTasks(tasks);

        const endDate = new Date().toISOString();
        const startDate = new Date(
          new Date().setDate(new Date().getDate() - 30),
        ).toISOString();
        // Get the user's timezone
        const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

        console.log("Timezone:", timezone);

        console.log("Start Date:", startDate);
        console.log("End Date:", endDate);

        let chardDataRes = await api.charts.getTasksChartForAccount(
          await createClient(),
          selectedAccount.account_id,
          startDate,
          endDate,
          TimeUnit.Day,
          encodeURIComponent(timezone)
        );

        console.log("chart data", chardDataRes);
        setChartData(chardDataRes.chartData);
      }
    };

    fetchData();
  }, [selectedAccount]);

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
