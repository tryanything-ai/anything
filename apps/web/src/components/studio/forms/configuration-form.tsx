import { useState } from "react";

import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { formValuesToJsonValues, getDefaultValuesFromFields } from "@/lib/json-schema-utils";
import { fieldsMap } from "./form-fields";

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
        <SmartForm
            name="my-form"
            onSubmit={handleOnSubmit}
            // From JSF
            fields={fields}
            initialValues={input}
            handleValidation={handleValidation}
        />
    );
}

// ===============================
// ====== UI COMPONENTS ==========
// ===============================

function SmartForm({ name, fields, initialValues, handleValidation, onSubmit }: any) {

    const [values, setValues] = useState<{ [key: string]: any }>(() =>
        getDefaultValuesFromFields(fields, initialValues)
    );
    const [errors, setErrors] = useState<{ [key: string]: any }>({});

    const [submited, setSubmited] = useState(false);

    function handleInternalValidation(valuesToValidate: any) {
        const valuesForJson = formValuesToJsonValues(fields, valuesToValidate);
        const { formErrors } = handleValidation(valuesForJson);

        setErrors(formErrors || {});

        return {
            errors: formErrors,
            jsonValues: valuesForJson
        };
    }

    function handleFieldChange(fieldName: any, value: any) {
        const newValues = {
            ...values,
            [fieldName]: value
        };
        setValues(newValues);

        handleInternalValidation(newValues);
    }

    function handleSubmit(e: any) {
        e.preventDefault();
        setSubmited(true);

        const validation = handleInternalValidation(values);

        if (validation.errors) {
            return null;
        }

        return onSubmit(validation.jsonValues, { formValues: values });
    }

    return (
        <form name={name} onSubmit={handleSubmit} noValidate className="rounded-lg border p-4">
            <div>
                {fields?.map((field: any) => {
                    const { name: fieldName, inputType } = field;
                    const FieldComponent = fieldsMap[inputType] || fieldsMap.error;

                    return (
                        <FieldComponent
                            key={fieldName}
                            value={values?.[fieldName]}
                            error={errors[fieldName]}
                            submited={submited}
                            onChange={handleFieldChange}
                            onValueChange={(value: any) => handleFieldChange(fieldName, value)}
                            {...field}
                        />
                    );
                })}

                <button type="submit">Submit</button>
            </div>
        </form>
    );
}