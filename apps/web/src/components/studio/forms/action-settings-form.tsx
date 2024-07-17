import { useAnything } from "@/context/AnythingContext";
import ConfigurationForm from "./configuration-form";
import { VariablesFormLayout } from "./variables/variables-form-layout";
import { ArrowBigLeft, Play } from "lucide-react";
import { Button } from "@/components/ui/button";

export default function ActionSettingsForm() {
    const { workflow, testing } = useAnything();

    const testAction = async (action_id: string) => {
        try {
            testing.testAction(action_id);
        } catch {
            console.error("Error testing workflow");
        }
    }

    const testWorkflow = async () => {
        try {
            testing.testWorkflow();
        } catch {
            console.error("Error testing workflow");
        }
    }

    return (
        <div className="flex flex-col h-full w-full">
            {(workflow.selected_node_data && workflow.selected_node_id) ?
                <div className="grid w-full items-start gap-6">
                    <div className="grid gap-6">
                        {/* Debug essentially */}
                        <div className="flex flex-row gap-2 mt-2">
                            <Button onClick={() => testAction(workflow.selected_node_id)} className="hover:bg-green-500">
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
                :
                <div className="flex flex-col justify-center items-center h-96 w-full">
                    <div className="flex flex-row mt-auto mb-auto text-center border-2 border-dashed rounded-md p-4">
                        <div className="flex flex-col justify-center items-center mr-2">
                            <ArrowBigLeft size={36} />
                        </div>
                        <div className="text-xl font-normal">
                            <div>Select a node</div>
                            <div>to configure</div>
                        </div>
                    </div>
                </div>
            }
        </div>
    )
}