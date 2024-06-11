"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "../ui/card";
import Link from "next/link";
import { BaseNodeIcon } from "../studio/nodes/node-icon";
import { useAnything } from "@/context/AnythingContext";

export default function ManageWorkflows() {

    let { workflows } = useAnything();

    console.log("flows in component", workflows.flows);
    return (
        <div>
            {workflows.flows.map((flow) => {
                let icons: string[] = [];
                
                //only do if we have actual data
                if (flow.flow_versions.length !== 0) {
                    icons = flow.flow_versions[0].flow_definition.actions.map((action) => {
                        return action.icon;
                    });
                }

                return (
                    <Link key={flow.flow_id} href={`/dashboard/workflows/${flow.flow_id}/${flow.flow_versions[0]?.flow_version_id}/editor`}>
                        <Card key={flow.flow_id} className="mt-2 flex flex-row hover:border-green-500">

                            <CardHeader>
                                <CardTitle>{flow.flow_name}</CardTitle>
                                <CardDescription>{flow.created_at}</CardDescription>

                            </CardHeader>
                            <CardContent>
                                <div className="flex flex-row h-full items-end">
                                    {/* <div className="flex flex-row"> */}
                                    {icons.map((icon, index) => {
                                        return (
                                            <BaseNodeIcon key={index} className="rounded-xl border" icon={icon} />
                                        )
                                    })}
                                    {/* </div> */}
                                    {/* TODO: add flow definition to get icons etc. */}
                                    {/* {flow.flow_name} */}
                                    {/* <Badge variant={team.account_role === 'owner' ? 'default' : 'outline'}>{team.is_primary_owner ? 'Primary Owner' : team.account_role}</Badge> */}
                                </div>
                            </CardContent>

                            {/* <TableCell className="text-right"><Button variant="outline" asChild><Link href={`/dashboard/${team.slug}`}>View</Link></Button></TableCell> */}
                        </Card>
                    </Link>
                )
            })}
        </div>
    )
}
