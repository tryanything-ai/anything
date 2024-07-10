import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "./json-schema-form";
import { useAnything } from "@/context/AnythingContext";

export default function InputVariablesForm() {

    const { variables, workflow } = useAnything();

    let fields, handleValidation;

    if (variables && variables.variables_schema && Object.keys(variables.variables_schema).length > 0) {
        ({ fields, handleValidation } = createHeadlessForm(variables.variables_schema, {
            strictInputType: false, // so you don't need to pass presentation.inputType,
            initialValues: variables.variables,
        }));
    }

    //Update Configuration
    async function handleVariableInputSubmit(jsonValues: any, { formValues }: any) {
        await workflow.updateNodeData(['variables'], [formValues]);
        console.log("Submitted!", { formValues, jsonValues });
    }

    console.log("Variables Schema: ", variables.variables_schema);
    console.log("Variables: ", variables.variables);

    return (
        <>
            {
                variables && variables.variables_schema && variables.variables && Object.keys(variables.variables_schema).length > 0 && Object.keys(variables.variables).length > 0 &&
                <JsonSchemaForm
                    name="input-variables-form"
                    onSubmit={handleVariableInputSubmit}
                    fields={fields}
                    initialValues={variables.variables}
                    handleValidation={handleValidation}
                />
            }
        </>
    );
}
