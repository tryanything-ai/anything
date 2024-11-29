import { useAnything } from "@/context/AnythingContext";
import ConfigurationForm from "./configuration-form";
import { VariablesFormLayout } from "./variables/variables-form-layout";
import { Play } from "lucide-react";
import { Button } from "@repo/ui/components/ui/button";
import NoNodeSelected from "./no-node-selected";

export default function ActionSettingsForm(): JSX.Element {
  const { workflow, testing } = useAnything();

  const testWorkflow = async () => {
    try {
      testing.testWorkflow();
    } catch {
      console.error("Error testing workflow");
    }
  };

  console.log("[RENDERING ACTION & VARIABLES FORMS]");

  return (
    <div className="flex flex-col h-full w-full">
      {workflow.selected_node_data && workflow.selected_action_id ? (
        <div className="grid w-full items-start gap-6">
          <div className="grid gap-2">
            <VariablesFormLayout />
            <ConfigurationForm />
          </div>
        </div>
      ) : (
        <NoNodeSelected />
      )}
    </div>
  );
}
