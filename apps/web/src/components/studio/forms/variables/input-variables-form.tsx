import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "./json-schema-form";
import { useAnything } from "@/context/AnythingContext";

export default function InputVariablesForm() {

    const { variables } = useAnything();

    const { fields, handleValidation } = createHeadlessForm(variables.variables_schema, {
        strictInputType: false, // so you don't need to pass presentation.inputType,
        initialValues: variables.variables,
    });

    //Update Configuration
    async function handleVariableInputSubmit(jsonValues: any, { formValues }: any) {
        alert(
            `Submitted with succes! ${JSON.stringify(
                { formValues, jsonValues },
                null,
                3
            )}`
        );

        console.log("Submitted!", { formValues, jsonValues });
    }

    return (
        <>
            {
                (Object.keys(variables.variables).length > 0) &&
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
