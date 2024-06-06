"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "../ui/card";
import { createClient } from "@/lib/supabase/server";
import { Table, TableRow, TableBody, TableCell } from "../ui/table";
import { Button } from "../ui/button";
import Link from "next/link";
import { Badge } from "../ui/badge";
import { useAnything } from "@/context/AnythingContext";
import { DB_FLOW_VERSION } from "@/types/supabase-anything";
import { BaseNodeIcon } from "../studio/nodes/node-icon";

export default function ManageWorkflows() {

    let { flows } = useAnything();

    console.log("flows in component", flows.flows);
    return (
        <div>
            {flows.flows.map((flow) => {

                let icons: string[] = flow.flow_versions[0].flow_definition.actions.map((action) => {
                    return action.icon;
                });

                if (flow.flow_versions[0].flow_definition.trigger?.icon) {
                    let trigger_icon = flow.flow_versions[0].flow_definition.trigger?.icon;
                    icons.unshift(trigger_icon);
                }

                return (
                    <Link href={`/dashboard/workflows/${flow.flow_id}`}>
                        <Card key={flow.flow_id} className="mt-2 flex flex-row hover:border-green-500">

                            <CardHeader>
                                <CardTitle>{flow.flow_name}</CardTitle>
                                <CardDescription>{flow.created_at}</CardDescription>

                            </CardHeader>
                            <CardContent>
                                <div className="flex flex-row h-full items-end">
                                    {/* <div className="flex flex-row"> */}
                                    {icons.map((icon) => {
                                        return (
                                            <BaseNodeIcon className="rounded-xl border" icon={icon} />
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
