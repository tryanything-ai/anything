"use client";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";
import Link from "next/link";
import { BaseNodeIcon } from "../studio/nodes/node-icon";
import { useAnything } from "@/context/AnythingContext";
import { Edit } from "lucide-react";
import { Button } from "../ui/button";

export default function ManageWorkflows() {
  let { workflows } = useAnything();

  console.log("flows in component", workflows.flows);
  return (
    <div>
      {workflows.flows.map((flow) => {
        let icons: string[] = [];

        //only do if we have actual data
        if (flow.flow_versions.length !== 0) {
          icons = flow.flow_versions[0].flow_definition.actions.map(
            (action) => {
              return action.icon;
            }
          );
        }

        return (
          <Link key={flow.flow_id} href={`/workflows/${flow.flow_id}`}>
            <Card
              key={flow.flow_id}
              className="mt-2 flex flex-row hover:border-green-500"
            >
              <CardHeader className="">
                <CardTitle>{flow.flow_name}</CardTitle>
                <CardDescription>{flow.description}</CardDescription>
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
                  <Link
                    className="flex flex-col justify-end h-full"
                    href={`/workflows/${flow.flow_id}/${flow.flow_versions[0]?.flow_version_id}/editor`}
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
