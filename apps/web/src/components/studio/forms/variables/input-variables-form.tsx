import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "./json-schema-form";
import { useAnything } from "@/context/AnythingContext";

export default function InputVariablesForm() {

    const { workflow } = useAnything();

    let fields, handleValidation;

    //Very ugly but we are working with deeply nested Json in the workflow object
    //There are better ways to do this that require more rewriting than i want right now. 
    if (
        workflow &&
        workflow.selected_node_data &&
        workflow.selected_node_data.variables &&
        typeof workflow.selected_node_data.variables === 'object' &&
        workflow.selected_node_data.variables_schema &&
        typeof workflow.selected_node_data.variables_schema === 'object'
    ) {
        console.log("Setting Variables List in Input")
        if (
            Object.keys(workflow.selected_node_data.variables).length > 0 &&
            Object.keys(workflow.selected_node_data.variables_schema).length > 0
        ) {

            ({ fields, handleValidation } = createHeadlessForm(workflow.selected_node_data.variables_schema, {
                strictInputType: false, // so you don't need to pass presentation.inputType,
                initialValues: workflow.selected_node_data.variables,
            }));
        }
    }

    //Update Configuration
    async function handleVariableInputSubmit(jsonValues: any, { formValues }: any) {
        await workflow.updateNodeData(['variables'], [formValues]);
        console.log("Submitted!", { formValues, jsonValues });
    }

    return (
        <>
            {
                workflow &&
                workflow.selected_node_data &&
                workflow.selected_node_data.variables &&
                typeof workflow.selected_node_data.variables === 'object' &&
                workflow.selected_node_data.variables_schema &&
                typeof workflow.selected_node_data.variables_schema === 'object' &&
                Object.keys(workflow.selected_node_data.variables).length > 0 &&
                Object.keys(workflow.selected_node_data.variables_schema).length > 0 &&
                <JsonSchemaForm
                    name="input-variables-form"
                    onSubmit={handleVariableInputSubmit}
                    fields={fields}
                    initialValues={workflow.selected_node_data.variables}
                    handleValidation={handleValidation}
                />
            }
        </>
    );
}
