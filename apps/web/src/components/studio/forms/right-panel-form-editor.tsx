import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import SettingsForm from "./flow-settings-form";
import ActionSettingsForm from "./action-settings-form"
import { useAnything } from "@/context/AnythingContext";

export default function RightPanelFormEditor() {
    const { workflow } = useAnything();

    return (
        <Tabs defaultValue="account" value={workflow.panel_tab} onValueChange={workflow.setPanelTab} className="w-full">
            <TabsList className="">
                <TabsTrigger value="config">Configuration</TabsTrigger>
                <TabsTrigger value="settings">Settings</TabsTrigger>
            </TabsList>
            <TabsContent value="config">
                <ActionSettingsForm />
            </TabsContent>
            <TabsContent value="settings">
                <SettingsForm />
            </TabsContent>
        </Tabs>
    )
}