"use client";
import { PartyPopper } from "lucide-react";
import DashboardTitleWithAction from "@/components/workflows/dashboard-title-with-action";
import { Separator } from "@repo/ui/components/ui//separator";
import { useAnything } from "@/context/AnythingContext";
import { useParams } from "next/navigation";
import { useEffect, useState } from "react";
import DashboardTitleWithNavigation from "@/components/workflows/dahsbloard-title-with-navigation";
import { TaskRow } from "@/lib/anything-api/testing";
import api from "@/lib/anything-api";
import { Table } from "@repo/ui/components/ui//table";
import { TaskTable } from "@/components/tasks/task-table";
import { TaskChart } from "@/components/tasks/task-chart";
import { TimeUnit } from "@/lib/anything-api/charts";
// import { DB_WORKFLOWS_QUERY } from "@/types/supabase-anything";

export default function WorkflowManager() {
  const {
    workflows: { getWorkflowById, flows },
  } = useAnything();
  const [workflow, setWorkflow] = useState<any | undefined>(undefined);
  const [tasks, setTasks] = useState<TaskRow[]>([]);
  const [chartData, setChartData] = useState<any | undefined>(undefined);
  const params = useParams<{ workflowId: string }>();

  useEffect(() => {
    const fetchData = async () => {
      console.log("params in useEffect", params);
      if (params.workflowId) {
        let flow = await getWorkflowById(params.workflowId);
        console.log("flow", flow);
        if (flow && flow.length > 0) {
          setWorkflow(flow[0]);
        }
        let tasks = await api.tasks.getTasksForWorkflow(params.workflowId);
        console.log("tasks", tasks);
        setTasks(tasks);

        const endDate = new Date().toISOString();
        const startDate = new Date(
          new Date().setDate(new Date().getDate() - 30),
        ).toISOString();
        let chardDataRes = await api.charts.getTasksChart(
          params.workflowId,
          startDate,
          endDate,
          TimeUnit.Day,
        );

        console.log("chart data", chardDataRes);
        setChartData(chardDataRes.chartData);
      }
    };

    fetchData();
  }, [params.workflowId, flows]);

  console.log("workflow", workflow);
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
          <div className=" flex flex-col gap-y-4 w-full  mx-auto text-center">
            <TaskChart chartData={chartData} />
          </div>

          <div className="border rounded-md flex flex-col gap-y-4  h-full w-full items-center justify-center content-center mx-auto text-center">
            <TaskTable tasks={tasks} />
          </div>
        </div>
      ) : null}
    </>
  );
}
