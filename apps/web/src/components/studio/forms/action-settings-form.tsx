import { useAnything } from "@/context/AnythingContext";
import JsonSchemaForm from "./json-schema-form";
import VariablesForm from "./variables-form";

export default function ActionSettingsForm() {
    const { workflow } = useAnything();

    return (
        <div>
            {(workflow.selected_node_data && workflow.selected_node_id) ?
                <div className="grid w-full items-start gap-6">
                    <div className="grid gap-6">
                        <VariablesForm variables={workflow.selected_node_data.variables} variables_schema={workflow.selected_node_data.variables_schema} />
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