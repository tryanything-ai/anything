import SettingsNavigation from "@/components/dashboard/settings-navigation";
import DashboardTitle from "@/components/dashboard/dashboard-title";
import { Separator } from "@/components/ui/separator";

export default function WorkflowsManagerLayout({ children }: { children: React.ReactNode }) {
    return (
        <div className="space-y-6 w-full">
            <DashboardTitle title="Workflows" description="Manage workflows." />
            <Separator />
            <div className="flex-1">{children}</div>
        </div>
    )
}