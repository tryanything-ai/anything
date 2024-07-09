import { useState } from "react";

import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { formValuesToJsonValues, getDefaultValuesFromFields } from "@/lib/json-schema-utils";
import { fieldsMap } from "./form-fields";
import { Button } from "@/components/ui/button";
import { JsonSchemaForm } from "./variables/json-schema-form";

export default function ConfigurationForm({ input_schema, input }: any) {
    const { fields, handleValidation } = createHeadlessForm(input_schema, {
        strictInputType: false, // so you don't need to pass presentation.inputType,
        initialValues: input,
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
        <>
            {Object.keys(input).length > 0 &&
                <div className="rounded-lg border p-4">
                    <JsonSchemaForm
                        name="configuration-form"
                        onSubmit={handleOnSubmit}
                        // From JSF
                        fields={fields}
                        initialValues={input}
                        handleValidation={handleValidation}
                    />
                </div>
            }
        </>
    );
}