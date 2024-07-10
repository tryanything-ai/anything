import { useAnything } from "@/context/AnythingContext";
import ConfigurationForm from "./configuration-form";
import { VariablesFormLayout } from "./variables/variables-form-layout";
import { ArrowBigLeft } from "lucide-react";

export default function ActionSettingsForm() {
    const { workflow } = useAnything();

    return (
        <div className="flex flex-col h-full w-full">
            {(workflow.selected_node_data && workflow.selected_node_id) ?
                <div className="grid w-full items-start gap-6">
                    <div className="grid gap-6">
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