import { useAnything } from "@/context/AnythingContext";
import ConfigurationForm from "./configuration-form";
import InputVariablesForm from "./variables/input-variables-form";
import { VariablesFormLayout } from "./variables/variables-form-layout";

export default function ActionSettingsForm() {
    const { workflow } = useAnything();

    return (
        <div>
            {(workflow.selected_node_data && workflow.selected_node_id) ?
                <div className="grid w-full items-start gap-6">
                    <div className="grid gap-6">
                        <VariablesFormLayout variables={workflow.selected_node_data.variables} variables_schema={workflow.selected_node_data.variables_schema} />
                        <ConfigurationForm
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