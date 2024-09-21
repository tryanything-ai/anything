import { useState, useEffect } from "react";

import {
  formValuesToJsonValues,
  getDefaultValuesFromFields,
} from "@/lib/json-schema-utils";
import { Button } from "@repo/ui/components/ui/button";
import { fieldsMap } from "../form-fields";

export function JsonSchemaForm({
  name,
  fields,
  initialValues,
  handleValidation,
  onSubmit,
}: any): JSX.Element {
  const [values, setValues] = useState<{ [key: string]: any }>({});
  const [errors, setErrors] = useState<{ [key: string]: any }>({});
  const [submited, setSubmitted] = useState(false);


  useEffect(() => {
    const defaultValues = getDefaultValuesFromFields(fields, initialValues);
    setValues(defaultValues);
    setErrors({});
  }, [fields, initialValues]);


   const handleInternalValidation = (valuesToValidate: any) => {
    const valuesForJson = formValuesToJsonValues(fields, valuesToValidate);
    const { formErrors } = handleValidation(valuesForJson);
    return { errors: formErrors || {}, jsonValues: valuesForJson };
  };

  const handleFieldChange = (fieldName: any, value: any) => {
    setValues((prevValues) => ({
      ...prevValues,
      [fieldName]: value,
    }));
  };

  const handleSubmit = (e: any) => {
    e.preventDefault();
    setSubmitted(true);
    const { errors, jsonValues } = handleInternalValidation(values);
    setErrors(errors);
    if (Object.keys(errors).length === 0) {
      onSubmit(jsonValues, { formValues: values });
    }
  };


  console.log("[RENDERING JSON SCHEMA FORM]");
  console.log("Values:", values);

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
              onValueChange={(value: any) =>
                handleFieldChange(fieldName, value)
              }
              {...field}
            />
          );
        })}
        <Button type="submit" variant={"default"}>
          Submit
        </Button>
      </div>
    </form>
  );
}
