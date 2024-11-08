import { useState, useEffect, useRef } from "react";

import {
  formValuesToJsonValues,
  getDefaultValuesFromFields,
} from "@/lib/json-schema-utils";
import { Button } from "@repo/ui/components/ui/button";
import { fieldsMap } from "./form-fields";
import { useAnything } from "@/context/AnythingContext";

let GLOBAL_CURSOR_LOCATION = 0;
let GLOBAL_ACTIVE_FIELD = "";
let GLOBAL_ACTIVE_FORM_NAME = "";

export function JsonSchemaForm({
  name,
  fields,
  initialValues,
  handleValidation,
  onSubmit,
  onFocus,
  onBlur,
}: any): JSX.Element {
  const {
    explorer: { registerCallback, unRegisterCallback },
  } = useAnything();

  const [values, setValues] = useState<{ [key: string]: any }>({});
  const [errors, setErrors] = useState<{ [key: string]: any }>({});
  const [submited, setSubmitted] = useState(false);

  const valuesRef = useRef(values);

  useEffect(() => {
    valuesRef.current = values;
  }, [values]);

  useEffect(() => {
    console.log("[JSON SCHEMA FORM] Initial values:", initialValues);
    console.log("[JSON SCHEMA FORM] Fields:", fields);
    const defaultValues = getDefaultValuesFromFields(fields, initialValues);
    console.log("[JSON SCHEMA FORM] Default values:", defaultValues);
    setValues(defaultValues);
    setErrors({});
  }, [fields, initialValues]);

  const handleInternalValidation = (valuesToValidate: any) => {
    const valuesForJson = formValuesToJsonValues(fields, valuesToValidate);
    const { formErrors } = handleValidation(valuesForJson);
    return { errors: formErrors || {}, jsonValues: valuesForJson };
  };

  const handleFieldChange = (fieldName: any, value: any) => {
    console.log(`[FIELD CHANGE] ${fieldName}:`, value);
    setValues((prevValues) => {
      console.log("[PREV VALUES]", prevValues);
      const newValues = {
        ...prevValues,
        [fieldName]: value,
      };
      console.log("[NEW VALUES]", newValues);
      return newValues;
    });
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

  //used to hook into showing variables etc
  const handleFieldFocus = (fieldName: string) => {
    // setFocusedField(fieldName);
    console.log("Show something?");
    if (onFocus) {
      onFocus(fieldName);
    }
  };

  const handleFieldBlur = (e: any) => {
    // setFocusedField(null);
    console.log("STOP showing something?");
    if (onBlur) {
      onBlur();
    }
  };

  useEffect(() => {
    console.log("[JSON SCHEMA FORM] Values after update:", values);
  }, [values]);

  console.log("[RENDERING JSON SCHEMA FORM]");
  console.log("Values:", values);

  const handleCursorChange = (e: React.SyntheticEvent, fieldName: string) => {
    const target = e.target as HTMLTextAreaElement;

    GLOBAL_CURSOR_LOCATION = target.selectionStart;

    GLOBAL_ACTIVE_FIELD = fieldName;

    GLOBAL_ACTIVE_FORM_NAME = name;

    console.log("Event type:", e.type);
    console.log("Cursor position:", {
      start: target.selectionStart,
      end: target.selectionEnd,
    });
    console.log("Active field name:", fieldName);
  };

  const insertVariable = (variable: string) => {
    
    if(GLOBAL_ACTIVE_FORM_NAME !== name) {
      console.log("Not the active form");
      return;
    }

    const values = valuesRef.current; // Use the latest values
    console.log("Inserting variable:", variable);
    if (!GLOBAL_ACTIVE_FIELD || GLOBAL_CURSOR_LOCATION === null) {
      console.log("No active field or cursor position");
      return;
    }

    console.log("[INSERT VARIABLE] Cursor location:", GLOBAL_CURSOR_LOCATION);
    console.log("[INSERT VARIABLE] Active field:", GLOBAL_ACTIVE_FIELD);
    console.log("[INSERT VARIABLE] Values:", values);

    const currentValue = values[GLOBAL_ACTIVE_FIELD] || "";
    console.log("[INSERT VARIABLE] Current value:", currentValue);
    const beforeCursor = currentValue.slice(0, GLOBAL_CURSOR_LOCATION);
    console.log("[INSERT VARIABLE] Before cursor:", beforeCursor);
    const afterCursor = currentValue.slice(GLOBAL_CURSOR_LOCATION);
    console.log("[INSERT VARIABLE] After cursor:", afterCursor);
    const newValue = beforeCursor + variable + afterCursor;

    handleFieldChange(GLOBAL_ACTIVE_FIELD, newValue);
  };

  // const insertVariable = (variable: string) => {
  //   console.log("Inserting variable:", variable);
  //   if (!GLOBAL_ACTIVE_FIELD || GLOBAL_CURSOR_LOCATION === null) {
  //     console.log("No active field or cursor position");
  //     return;
  //   }

  //   console.log("[INSERT VARIABLE] Cursor location:", GLOBAL_CURSOR_LOCATION);
  //   console.log("[INSERT VARIABLE] Active field:", GLOBAL_ACTIVE_FIELD);
  //   console.log("[INSERT VARIABLE] Values:", values);

  //   const currentValue = values[GLOBAL_ACTIVE_FIELD] || "";
  //   console.log("[INSERT VARIABLE] Current value:", currentValue);
  //   const beforeCursor = currentValue.slice(0, GLOBAL_CURSOR_LOCATION);
  //   console.log("[INSERT VARIABLE] Before cursor:", beforeCursor);
  //   const afterCursor = currentValue.slice(GLOBAL_CURSOR_LOCATION);
  //   console.log("[INSERT VARIABLE] After cursor:", afterCursor);
  //   const newValue = beforeCursor + variable + afterCursor;

  //   handleFieldChange(GLOBAL_ACTIVE_FIELD, newValue);
  // };

  useEffect(() => {
    registerCallback(name, insertVariable);
    return () => {
      unRegisterCallback(name);
    };
  }, []);

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
              onFocus={() => handleFieldFocus(fieldName)}
              onBlur={handleFieldBlur}
              onSelect={(e: any) => handleCursorChange(e, fieldName)}
              onClick={(e: any) => handleCursorChange(e, fieldName)}
              onKeyUp={(e: any) => handleCursorChange(e, fieldName)}
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
