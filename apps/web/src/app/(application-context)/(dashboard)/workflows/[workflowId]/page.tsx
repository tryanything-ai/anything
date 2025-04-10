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

export default function WorkflowManager(): JSX.Element {
  const params = useParams<{ workflowId: string }>();
  const {
    accounts: { selectedAccount },
  } = useAnything();


  const endDate = new Date().toISOString();
  const startDate = new Date(
    new Date().setDate(new Date().getDate() - 30),
  ).toISOString();
  const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone;

  const workflowFetcher = async () => {
    if (!selectedAccount || !params.workflowId) return null;
    const supabase = await createClient();
    const flow = await api.flows.getFlow(
      supabase,
      selectedAccount.account_id,
      params.workflowId,
    );
    return flow?.[0];
  };


  const chartDataFetcher = async () => {
    if (!selectedAccount || !params.workflowId) return null;
    const supabase = await createClient();
    const chartData = await api.charts.getTasksChartForWorkflow(
      supabase,
      selectedAccount.account_id,
      params.workflowId,
      startDate,
      endDate,
      TimeUnit.Day,
      encodeURIComponent(timezone),
    );
    return chartData.chartData;
  };

  const { data: workflow } = useSWR(
    selectedAccount
      ? [`workflow`, selectedAccount.account_id, params.workflowId]
      : null,
    workflowFetcher,
    {
      revalidateOnFocus: true,
    },
  );

  const { data: chartData } = useSWR(
    selectedAccount
      ? [`chartData`, selectedAccount.account_id, params.workflowId]
      : null,
    chartDataFetcher,
    {
      revalidateOnFocus: true,
    },
  );

  return (
    <>
      {workflow ? (
        <div className="space-y-6 w-full">
          <DashboardTitleWithNavigation
            title={workflow?.flow_name}
            description="Manage workflows."
            href={`/workflows/${workflow.flow_id}/${workflow.flow_versions[0]?.flow_version_id}/editor`}
          />

          <Separator />
          <div className="flex flex-col gap-y-4 w-full mx-auto text-center">
            <TaskChart chartData={chartData} />
          </div>

          <div className="border rounded-md w-full">
            <TaskTable
              workflowId={params.workflowId}
              accountId={selectedAccount?.account_id ?? ""}
            />
          </div>
        </div>
      ) : null}
    </>
  );
}
