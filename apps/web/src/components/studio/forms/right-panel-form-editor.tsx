import {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "@repo/ui/components/ui/tabs";
import SettingsForm from "./settings/flow-settings-tab";
import ActionSettingsForm from "./action-settings-form";
import { useAnything } from "@/context/AnythingContext";
import ActionDisplayTab from "./display/action-display-tab";
import TestingTab from "./testing/testing-tab";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";

export default function RightPanelFormEditor() {
  const { workflow } = useAnything();

  return (
    <Tabs
      defaultValue="account"
      value={workflow.panel_tab}
      onValueChange={workflow.setPanelTab}
      className="flex flex-col h-full p-2"
    >
      <TabsList className="w-[350px]">
        <TabsTrigger value="config">Configuration</TabsTrigger>
        <TabsTrigger value="display">Display</TabsTrigger>
        <TabsTrigger value="testing">Testing</TabsTrigger>
        <TabsTrigger value="settings">Settings</TabsTrigger>
      </TabsList>
      <TabsContent value="config" className="h-full overflow-y-auto">
        <ScrollArea>
          <ActionSettingsForm />
        </ScrollArea>
      </TabsContent>
      <TabsContent value="display" className="h-full overflow-y-auto">
        <ScrollArea className="h-full">
          <ActionDisplayTab />
        </ScrollArea>
      </TabsContent>
      <TabsContent value="testing" className="h-full overflow-y-auto">
        <ScrollArea className="h-full">
          <TestingTab />
        </ScrollArea>
      </TabsContent>
      <TabsContent value="settings" className="h-full overflow-y-auto">
        <ScrollArea className="h-full">
          <SettingsForm />
        </ScrollArea>
      </TabsContent>
    </Tabs>
  );
}
