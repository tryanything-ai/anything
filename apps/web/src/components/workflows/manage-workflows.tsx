"use client";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import Link from "next/link";
import { BaseNodeIcon } from "../studio/nodes/node-icon";
import { useAnything } from "@/context/AnythingContext";
import { Edit } from "lucide-react";
import { Button } from "@repo/ui/components/ui/button";
import WorkflowStatusComponent from "./workflow-status";
import { AnyAaaaRecord } from "dns";

export default function ManageWorkflows(): JSX.Element {
  let { workflows } = useAnything();

  console.log("flows in component", workflows.flows);
  return (
    <div>
      {workflows.flows.map((flow: any) => {
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
          <Link key={flow.flow_id} href={`/workflows/${flow.flow_id}`}>
            <Card
              key={flow.flow_id}
              className="mt-2 flex flex-row hover:border-green-500"
            >
              <CardHeader className="w-1/4">
                <CardTitle className="truncate">{flow.flow_name}</CardTitle>
                <CardDescription className="truncate">
                  {flow.description}
                </CardDescription>
              </CardHeader>
              <CardContent className="flex-1">
                <div className="flex flex-row h-full items-end">
                  {/* <div className="flex flex-row"> */}
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
                  {/* {draft && <div className="mx-3">DRAFT </div>} */}
                  <div className="mx-3">
                    <WorkflowStatusComponent
                      active={flow.active}
                      draft={draft}
                    />
                  </div>

                  <Link
                    className="flex flex-col justify-end h-full"
                    href={`/workflows/${flow.flow_id}/${flow_version.flow_version_id}/editor`}
                  >
                    <Button>
                      <Edit size={16} />
                    </Button>
                  </Link>
                </div>
              </CardContent>
            </Card>
          </Link>
        );
      })}
    </div>
  );
}
