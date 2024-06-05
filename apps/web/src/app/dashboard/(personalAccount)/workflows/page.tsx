"use client"
import ManageWorkflows from "@/components/workflows/manage-workflows";
import { PartyPopper } from "lucide-react";
import DashboardTitleWithAction from "@/components/workflows/dashboard-title-with-action";
import { Separator } from "@/components/ui/separator";

export default function Workflows() {
    const createWorkflow = () => {
        //TODO: Implement
    }
    return (
        <div className="space-y-6 w-full">
            <DashboardTitleWithAction title="Workflows" description="Manage workflows." action={createWorkflow} />
            <Separator />
            <ManageWorkflows />
        </div>
    )
}