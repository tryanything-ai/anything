"use client"
import { PartyPopper } from "lucide-react";
import DashboardTitleWithAction from "@/components/workflows/dashboard-title-with-action";
import { Separator } from "@/components/ui/separator";
import { useAnything } from "@/context/AnythingContext";
import { useParams } from 'next/navigation'
import { useEffect, useState } from "react";
import { DBWorkflow } from "@/context/FlowsProvider";
import DashboardTitleWithNavigation from "@/components/workflows/dahsbloard-title-with-navigation";

export default function WorkflowManager() {
    const { flows: { getFlowById, flows } } = useAnything();
    const [workflow, setWorkflow] = useState<DBWorkflow | undefined>(undefined);
    const params = useParams<{ workflowId: string; }>()

    useEffect(() => {
        const fetchData = async () => {
            console.log("params in useEffect", params);
            if (params.workflowId) {
                let flow = await getFlowById(params.workflowId);
                setWorkflow(flow);
            }
        };

        fetchData();
    }, [params.workflowId, flows]);

    console.log("workflow", workflow);
    return (
        <>
            {
                workflow ?
                    <div className="space-y-6 w-full">

                        < DashboardTitleWithNavigation title={workflow?.flow_name} description="Manage workflows." href="/edit" />
                        < Separator />

                        <div className="flex flex-col gap-y-4 py-12 h-full w-full items-center justify-center content-center max-w-screen-md mx-auto text-center">
                            <PartyPopper className="h-12 w-12 text-gray-400" />
                            <h1 className="text-2xl font-bold">A single Workflow</h1>
                        </div>

                    </div >
                    : null}
        </>
    )
}