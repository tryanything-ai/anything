import { useState } from "react";

import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { formValuesToJsonValues, getDefaultValuesFromFields } from "@/lib/json-schema-utils";
import { Button } from "@/components/ui/button";
import { fieldsMap } from "../form-fields";
import { EDIT_VARIABLES_SCHEMA, EDIT_VARIABLES_VARIABLES } from "./edit-variable-schema";

// Edit a single variable
export default function EditVariableForm() {

    const { fields, handleValidation } = createHeadlessForm(EDIT_VARIABLES_SCHEMA, {
        strictInputType: false, // so you don't need to pass presentation.inputType,
        initialValues: EDIT_VARIABLES_VARIABLES,
    });

    async function handleOnSubmit(jsonValues: any, { formValues }: any) {
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
        <SmartForm
            name="my-form"
            onSubmit={handleOnSubmit}
            // From JSF
            fields={fields}
            initialValues={EDIT_VARIABLES_VARIABLES}
            handleValidation={handleValidation}
        />
    );
}
