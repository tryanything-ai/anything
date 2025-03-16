"use client";

import useSWR from "swr";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import Link from "next/link";
import { BaseNodeIcon } from "../studio/nodes/node-icon";
import { BarChart, Edit } from "lucide-react";
import { Button } from "@repo/ui/components/ui/button";
import WorkflowStatusComponent from "./workflow-status";
import api from "@repo/anything-api";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";

export default function ManageWorkflows(): JSX.Element {
  let {
    accounts: { selectedAccount },
  } = useAnything();

  // Create fetcher function
  const fetcher = async () => {
    if (!selectedAccount) return [];
    const supabase = await createClient();
    return api.flows.getFlows(supabase, selectedAccount.account_id);
  };

  // Replace useState and useEffect with useSWR
  const {
    data: workflows = [],
    error,
    isLoading,
  } = useSWR(
    // Cache key - will refetch if selectedAccount changes
    selectedAccount ? [`workflows`, selectedAccount.account_id] : null,
    fetcher,
    {
      revalidateOnFocus: true, // Refresh when user focuses the page
    },
  );

  if (error) {
    console.error("Error fetching workflows:", error);
  }

  return (
    <div>
      {isLoading ? (
        <div>Loading workflows...</div>
      ) : (
        workflows.map((flow: any) => {
          let icons: string[] = [];

          let flow_version: any;
          let draft = true;

          //grab published if we have it
          if (
            flow.published_workflow_versions &&
            flow.published_workflow_versions.length > 0
          ) {
            flow_version = flow.published_workflow_versions[0];
            draft = false;
          }
          //grab draft if we don't
          if (
            !flow_version &&
            flow.draft_workflow_versions &&
            flow.draft_workflow_versions.length > 0
          ) {
            flow_version = flow.draft_workflow_versions[0];
          }

          //only do if we have actual data
          if (flow_version) {
            icons = Array.from(
              flow_version.flow_definition.actions
                .reduce((unique: Map<string, any>, action: any) => {
                  if (!unique.has(action.plugin_name)) {
                    unique.set(action.plugin_name, action);
                  }
                  return unique;
                }, new Map())
                .values(),
            )
              .slice(0, 5)
              .map((action: any) => action.icon);
          }

          return (
            <Card
              key={flow.flow_id}
              className="mt-2 flex flex-row hover:border-green-500"
            >
              <Link
                href={`/workflows/${flow.flow_id}/${flow_version?.flow_version_id}/editor`}
                // href={`/workflows/${flow.flow_id}`}
                className="flex-1 flex"
              >
                <CardHeader className="w-1/4">
                  <CardTitle className="truncate leading-tight">
                    {flow.flow_name}
                  </CardTitle>
                  <CardDescription className="truncate">
                    {flow.description}
                  </CardDescription>
                </CardHeader>
                <CardContent className="flex-1">
                  <div className="flex flex-row h-full items-end">
                    {icons.map((icon, index) => {
                      return (
                        <BaseNodeIcon
                          key={index}
                          className="rounded-xl border"
                          icon={icon}
                        />
                      );
                    })}

                    <div className="flex-1" />
                    <div className="mx-3">
                      <WorkflowStatusComponent
                        active={flow.active}
                        draft={draft}
                      />
                    </div>
                  </div>
                </CardContent>
              </Link>
              <div className="flex items-end p-6">
                <Link
                  href={`/workflows/${flow.flow_id}`}
                  // href={`/workflows/${flow.flow_id}/${flow_version.flow_version_id}/editor`}
                >
                  <Button>
                    {/* <Edit size={16} /> */}
                    <BarChart size={16} />
                  </Button>
                </Link>
              </div>
            </Card>
          );
        })
      )}
    </div>
  );
}
