import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import SettingsForm from "./settings/flow-settings-tab";
import ActionSettingsForm from "./action-settings-form"
import { useAnything } from "@/context/AnythingContext";
import ActionDisplayTab from "./display/action-display-tab";
import TestingTab from './testing/testing-tab';

export default function RightPanelFormEditor() {
    const { workflow } = useAnything();

    return (
        <Tabs defaultValue="account" value={workflow.panel_tab} onValueChange={workflow.setPanelTab} className="w-full h-full">
            <TabsList className="">
                <TabsTrigger value="config">Configuration</TabsTrigger>
                <TabsTrigger value="display">Display</TabsTrigger>
                <TabsTrigger value="testing">Testing</TabsTrigger>
                <TabsTrigger value="settings">Settings</TabsTrigger>

            </TabsList>
            <div className="flex-1 overflow-hidden">
                <TabsContent value="config" className="h-full overflow-y-auto">
                    <ActionSettingsForm />
                </TabsContent>
                <TabsContent value="display" className="h-full overflow-y-auto">
                    <ActionDisplayTab />
                </TabsContent>
                <TabsContent value="testing" className="h-full overflow-y-auto">
                    <TestingTab />
                </TabsContent>
                <TabsContent value="settings" className="h-full overflow-y-auto">
                    <SettingsForm />
                </TabsContent>

            </div>
        </Tabs>
    )
}