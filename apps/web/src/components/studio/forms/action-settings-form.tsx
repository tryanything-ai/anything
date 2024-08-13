import { useAnything } from "@/context/AnythingContext";
import ConfigurationForm from "./configuration-form";
import { VariablesFormLayout } from "./variables/variables-form-layout";
import { Play } from "lucide-react";
import { Button } from "@repo/ui/button";
import NoNodeSelected from "./no-node-selected";

export default function ActionSettingsForm() {
  const { workflow, testing } = useAnything();

  const testAction = async (action_id: string) => {
    try {
      testing.testAction(action_id);
    } catch {
      console.error("Error testing workflow");
    }
  };

  const testWorkflow = async () => {
    try {
      testing.testWorkflow();
    } catch {
      console.error("Error testing workflow");
    }
  };

  return (
    <div className="flex flex-col h-full w-full">
      {workflow.selected_node_data && workflow.selected_node_id ? (
        <div className="grid w-full items-start gap-6">
          <div className="grid gap-6">
            {/* Debug essentially */}
            <div className="flex flex-row gap-2 mt-2">
              <Button
                onClick={() => testAction(workflow.selected_node_id)}
                className="hover:bg-green-500"
              >
                Test Action
                <Play size={16} className="ml-2" />
              </Button>
              <Button onClick={testWorkflow} className="hover:bg-green-500">
                Test Workflow
                <Play size={16} className="ml-2" />
              </Button>
            </div>
            <VariablesFormLayout />
            <ConfigurationForm
              input_schema={workflow.selected_node_data.input_schema}
              input={workflow.selected_node_data.input}
            />
          </div>
        </div>
      ) : (
        <NoNodeSelected />
      )}
    </div>
  );
}
