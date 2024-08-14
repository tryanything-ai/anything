import { useState } from "react";
import { useAnything } from "@/context/AnythingContext";
import { Switch } from "@repo/ui/components/ui//switch";
import { Label } from "@repo/ui/components/ui//label";

const WorkflowToggle = ({
  active,
  workflow_id,
}: {
  active: boolean;
  workflow_id: string;
}) => {
  const { workflow } = useAnything();

  const updateActive = async () => {
    try {
      await workflow.updateWorkflow({ active: !active });
    } catch (error) {
      console.error("Error updating workflow:", error);
    }
  };

  return (
    <div className="flex items-center space-x-2 mx-2">
      <Switch
        id="workflow-active"
        className="data-[state=checked]:bg-green-400 data-[state=unchecked]:bg-input"
        checked={active}
        onCheckedChange={updateActive}
      />
      <Label htmlFor="workflow-active">{active ? "ON" : "OFF"}</Label>
    </div>
  );
};

export default WorkflowToggle;
