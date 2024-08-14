"use client";

import * as React from "react";
import { Bar, BarChart, CartesianGrid, XAxis } from "recharts";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui//card";
import {
  ChartConfig,
  ChartContainer,
  ChartTooltip,
  ChartTooltipContent,
} from "@repo/ui/components/ui//chart";

const chartConfig = {
  pending: {
    label: "Pending",
    color: "hsl(var(--chart-1))",
  },
  waiting: {
    label: "Waiting",
    color: "hsl(var(--chart-2))",
  },
  running: {
    label: "Running",
    color: "hsl(var(--chart-3))",
  },
  completed: {
    label: "Completed",
    color: "rebeccapurple",
  },
  failed: {
    label: "Failed",
    color: "hsl(var(--failure))",
  },
  canceled: {
    label: "Canceled",
    color: "hsl(var(--chart-1))",
  },
} satisfies ChartConfig;

export function TaskChart({ chartData }: { chartData: any }) {
  const [loading, setLoading] = React.useState(true);

  React.useEffect(() => {
    if (chartData) {
      setLoading(false);
    }
  }, [chartData]);

  const total = React.useMemo(
    () => ({
      pending:
        chartData?.reduce((acc: any, curr: any) => acc + curr.pending, 0) || 0,
      waiting:
        chartData?.reduce((acc: any, curr: any) => acc + curr.waiting, 0) || 0,
      running:
        chartData?.reduce((acc: any, curr: any) => acc + curr.running, 0) || 0,
      completed:
        chartData?.reduce((acc: any, curr: any) => acc + curr.completed, 0) ||
        0,
      failed:
        chartData?.reduce((acc: any, curr: any) => acc + curr.failed, 0) || 0,
      canceled:
        chartData?.reduce((acc: any, curr: any) => acc + curr.canceled, 0) || 0,
    }),
    [chartData],
  );

  if (loading) {
    return <div>Loading...</div>;
  }

  return (
    <Card>
      <CardHeader className="flex flex-col items-stretch space-y-0 border-b p-0 sm:flex-row">
        <div className="flex flex-1 flex-col justify-center gap-1 px-6 py-5 sm:py-6">
          <CardTitle>Task Runs</CardTitle>
          <CardDescription>All Task Execution for last 30 days</CardDescription>
        </div>
        <div className="flex">
          {["pending", "waiting", "running", "completed", "failed"].map(
            (key) => {
              const chart = key as keyof typeof chartConfig;
              return (
                <button
                  key={chart}
                  className="relative z-30 flex flex-1 flex-col justify-center gap-1 border-t px-6 py-4 text-left even:border-l data-[active=true]:bg-muted/50 sm:border-l sm:border-t-0 sm:px-8 sm:py-6"
                >
                  <span className="text-xs text-muted-foreground">
                    {chartConfig[chart].label}
                  </span>
                  <span className="text-lg font-bold leading-none sm:text-3xl">
                    {total[key as keyof typeof total].toLocaleString()}
                  </span>
                </button>
              );
            },
          )}
        </div>
      </CardHeader>
      <CardContent className="px-2 sm:p-6">
        <ChartContainer
          config={chartConfig}
          className="aspect-auto h-[250px] w-full"
        >
          <BarChart
            accessibilityLayer
            data={chartData}
            margin={{
              left: 12,
              right: 12,
            }}
          >
            <CartesianGrid vertical={false} />
            <XAxis
              dataKey="date"
              tickLine={false}
              axisLine={false}
              tickMargin={8}
              minTickGap={32}
              tickFormatter={(value) => {
                const date = new Date(value);
                return date.toLocaleDateString("en-US", {
                  month: "short",
                  day: "numeric",
                });
              }}
            />
            {/* <ChartTooltip content={<ChartTooltipContent hideLabel />} /> */}
            <ChartTooltip
              content={
                <ChartTooltipContent
                  className="w-[150px]"
                  labelFormatter={(value) => {
                    return new Date(value).toLocaleDateString("en-US", {
                      month: "short",
                      day: "numeric",
                      year: "numeric",
                    });
                  }}
                />
              }
            />
            <Bar dataKey="pending" stackId="a" fill={`var(--color-pending)`} />
            <Bar dataKey="waiting" stackId="a" fill={`var(--color-waiting)`} />
            <Bar dataKey="running" stackId="a" fill={`var(--color-running)`} />
            <Bar
              dataKey="completed"
              stackId="a"
              fill={`var(--color-completed)`}
            />
            <Bar
              dataKey="canceled"
              stackId="a"
              fill={`var(--color-canceled)`}
            />
            <Bar dataKey="failed" stackId="a" fill={`var(--color-failed)`} />
          </BarChart>
        </ChartContainer>
      </CardContent>
    </Card>
  );
}
