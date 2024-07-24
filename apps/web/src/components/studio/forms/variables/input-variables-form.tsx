import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "./json-schema-form";
import { useAnything } from "@/context/AnythingContext";

export default function InputVariablesForm() {

    const { workflow: { selected_node_variables, selected_node_variables_schema, updateNodeData } } = useAnything();

    let fields, handleValidation;

    //Very ugly but we are working with deeply nested Json in the workflow object
    //There are better ways to do this that require more rewriting than i want right now. 
    if (
        selected_node_variables &&
        typeof selected_node_variables === 'object' &&
        selected_node_variables_schema &&
        typeof selected_node_variables_schema === 'object'
    ) {
        console.log("Setting Variables List in Input", selected_node_variables, selected_node_variables_schema)
        if (
            Object.keys(selected_node_variables).length > 0 &&
            Object.keys(selected_node_variables_schema).length > 0
        ) {
            ({ fields, handleValidation } = createHeadlessForm(selected_node_variables_schema, {
                strictInputType: false, // so you don't need to pass presentation.inputType,
                initialValues: selected_node_variables,
            }));
        }
    } else {
        console.log("No Variables List in Input", selected_node_variables, selected_node_variables_schema)
    }

    //Update Configuration
    async function handleVariableInputSubmit(jsonValues: any, { formValues }: any) {
        await updateNodeData(['variables'], [formValues]);
        console.log("Submitted!", { formValues, jsonValues });
    }

    return (
        <>
            {
                selected_node_variables &&
                typeof selected_node_variables === 'object' &&
                selected_node_variables_schema &&
                typeof selected_node_variables_schema === 'object' &&
                Object.keys(selected_node_variables).length > 0 &&
                Object.keys(selected_node_variables_schema).length > 0 &&
                <JsonSchemaForm
                    name="input-variables-form"
                    onSubmit={handleVariableInputSubmit}
                    fields={fields}
                    initialValues={selected_node_variables}
                    handleValidation={handleValidation}
                />
            }
        </>
    );
}
