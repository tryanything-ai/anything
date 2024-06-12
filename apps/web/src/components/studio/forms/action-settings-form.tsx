import { useAnything } from "@/context/AnythingContext";
import JsonSchemaForm from "./json-schema-form";

export default function ActionSettingsForm() {
    const { workflow } = useAnything();

    return (
        <div>
            {(workflow.selected_node_data && workflow.selected_node_id) ?
                <div className="grid w-full items-start gap-6">
                    <div className="grid gap-6 rounded-lg border p-4">
                        <JsonSchemaForm
                            input_schema={workflow.selected_node_data.input_schema}
                            input={workflow.selected_node_data.input}
                        />
                    </div>
                </div>
                :
                <div>
                    Select A Node
                </div>
            }

        </div>

    )
}