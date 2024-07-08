import { useState } from "react";

import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "./json-schema-form";
import { Button } from "@/components/ui/button";

export default function InputVariablesForm({ variables_schema, variables, setEditing }: any) {

    const { fields, handleValidation } = createHeadlessForm(variables_schema, {
        strictInputType: false, // so you don't need to pass presentation.inputType,
        initialValues: variables,
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
           
            <JsonSchemaForm
                name="input-variables-form"
                onSubmit={handleVariableInputSubmit}
                fields={fields}
                initialValues={variables}
                handleValidation={handleValidation}
            />
        </>
    );
}
