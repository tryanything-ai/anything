"use client";

import React from "react";
import { TaskChart } from "@/components/tasks/task-chart";
import { TaskTable } from "@/components/tasks/task-table";
import { useParams } from "next/navigation";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";
import { TimeUnit } from "@repo/anything-api";
import api from "@repo/anything-api";
import useSWR from "swr";

const WorkflowPage: React.FC = () => {
  const params = useParams();
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const endDate = new Date().toISOString();
  const startDate = new Date(
    new Date().setDate(new Date().getDate() - 30),
  ).toISOString();
  const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  const chartDataFetcher = async () => {
    if (!selectedAccount || !params.id) return null;
    const supabase = await createClient();
    const chartData = await api.charts.getTasksChartForWorkflow(
      supabase,
      selectedAccount.account_id,
      params.id as string,
      startDate,
      endDate,
      TimeUnit.Day,
      encodeURIComponent(timezone),
    );
    return chartData.chartData;
  };

  const { data: chartData } = useSWR(
    selectedAccount
      ? [`chartData`, selectedAccount.account_id, params.id]
      : null,
    chartDataFetcher,
    {
      revalidateOnFocus: true,
    },
  );

  return (
    <div className="space-y-6">
      <div className="flex flex-col gap-y-4">
        <TaskChart chartData={chartData} />
      </div>

      <div className="border rounded-md">
        <TaskTable
          accountId={selectedAccount?.account_id ?? ""}
          workflowId={params.id as string}
        />
      </div>
    </div>
  );
};

export default WorkflowPage;
