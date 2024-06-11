import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import SettingsForm from "./flow-settings-form";
import ActionSettingsForm from "./action-settings-form"

export default function RightPanelFormEditor() {
    return (
        <Tabs defaultValue="account" className="w-full">
            <TabsList className="">
                <TabsTrigger value="config">Config</TabsTrigger>
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