"use client"

import SettingsNavigation from "@/components/dashboard/settings-navigation";
import DashboardTitle from "@/components/dashboard/dashboard-title";
import { Separator } from "@/components/ui/separator";
import DashboardTitleWithAction from "@/components/workflows/dashboard-title-with-action";

export default function WorkflowsManagerLayout({ children }: { children: React.ReactNode }) {
    const createWorkflow = () => {
        //TODO: Implement
    }

    return (
        <div className="space-y-6 w-full">
            <DashboardTitleWithAction title="Workflows" description="Manage workflows." action={createWorkflow} />
            <Separator />
            <div className="flex-1">{children}</div>
        </div>
    )
}