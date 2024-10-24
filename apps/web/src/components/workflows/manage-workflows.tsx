"use client";

import { useEffect, useState } from "react";
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

export default function ManageWorkflows(): JSX.Element {
  let {
    accounts: { selectedAccount },
  } = useAnything();

  const [workflows, setWorkflows] = useState([]);

  const getWorkflows = async (): Promise<void> => {
    console.log("Getting Flows from API");
    try {
      if (!selectedAccount) return;
      let res: any = await api.flows.getFlows(selectedAccount.account_id);
      console.log("getFlows:", res);
      if (res.length > 0) {
        setWorkflows(res);
      } else {
        setWorkflows([]);
      }
    } catch (error) {
      console.error("Error getting flows", error);
    }
  };

  useEffect(() => {
    getWorkflows();
  }, [selectedAccount]);

  return (
    <div>
      {workflows.map((flow: any) => {
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
          icons = flow_version.flow_definition.actions.map((action: any) => {
            return action.icon;
          });
        }

        return (
          <Card
            key={flow.flow_id}
            className="mt-2 flex flex-row hover:border-green-500"
          >
            <Link
              href={`/workflows/${flow.flow_id}/${flow_version.flow_version_id}/editor`}
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
      })}
    </div>
  );
}
