import { useState } from "react";

import { formValuesToJsonValues, getDefaultValuesFromFields } from "@/lib/json-schema-utils";
import { Button } from "@/components/ui/button";
import { fieldsMap } from "../form-fields";

export function JsonSchemaForm({ name, fields, initialValues, handleValidation, onSubmit}: any) {

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
        <form name={name} onSubmit={handleSubmit} noValidate>
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
                <Button type="submit" variant={"default"} >Submit</Button>
            </div>
        </form>
    );
}
