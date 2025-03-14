import { useState, useEffect, useRef } from "react";

import {
  applyDefaultValuesToEmptyFields,
  formValuesToJsonValues,
  getDefaultValuesFromFields,
} from "@/lib/json-schema-utils";
import { Button } from "@repo/ui/components/ui/button";
import { fieldsMap } from "./fields/form-fields";
import { useAnything } from "@/context/AnythingContext";
import { TriangleAlertIcon } from "lucide-react";

//YES these GLOBALS are super naughty and I never do this but damn
//i could just not get it to work fast enough
//any other way right now
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
  // onBlur,
  disabled = false,
  showInputsExplorer = false,
  showResultsExplorer = false,
}: any): JSX.Element {
  const {
    explorer: { registerCallback, unRegisterCallback },
  } = useAnything();

  const [values, setValues] = useState<{ [key: string]: any }>({});
  const [errors, setErrors] = useState<{ [key: string]: any }>({});
  const [submited, setSubmitted] = useState(false);
  const [hasUnsavedChanges, setHasUnsavedChanges] = useState(false);
  const [invalidJsonFields, setInvalidJsonFields] = useState<Set<string>>(
    new Set(),
  );

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
    setHasUnsavedChanges(false);
  }, [fields, initialValues]);

  const handleInternalValidation = (valuesToValidate: any) => {
    console.log(
      "[HANDLE INTERNAL VALIDATION] Values to validate:",
      valuesToValidate,
    );
    const valuesForJson = formValuesToJsonValues(fields, valuesToValidate);
    console.log(
      "[HANDLE INTERNAL VALIDATION] Values for JSON validation:",
      valuesForJson,
    );
    const { formErrors } = handleValidation(valuesForJson);
    return { errors: formErrors || {}, jsonValues: valuesForJson };
  };

  const handleFieldChange = (fieldName: any, value: any, isValid?: boolean) => {
    if (disabled) return;

    // Update invalid JSON fields tracking
    setInvalidJsonFields((prev) => {
      const newSet = new Set(prev);
      if (isValid === false) {
        newSet.add(fieldName);
      } else {
        newSet.delete(fieldName);
      }
      return newSet;
    });

    setValues((prevValues) => {
      const newValues = {
        ...prevValues,
        [fieldName]: value,
      };

      // Only validate if all JSON fields are valid
      if (isValid !== false) {
        const { errors, jsonValues } = handleInternalValidation(newValues);
        setErrors(errors);
      }

      return newValues;
    });
    setHasUnsavedChanges(true);
  };

  const handleSubmit = (e: any) => {
    e.preventDefault();
    if (disabled) return;
    setSubmitted(true);
    const { errors, jsonValues } = handleInternalValidation(values);
    console.log("[JSON SCHEMA FORM - SUBMIT] Errors:", errors);
    setErrors(errors);
    if (Object.keys(errors).length === 0) {
      console.log(
        "[JSON SCHEMA FORM - SUBMIT] No errors, submitting:",
        jsonValues,
      );

      onSubmit(values);
      setHasUnsavedChanges(false);
    }
  };

  //used to hook into showing variables etc
  const handleFieldFocus = (fieldName: string) => {
    if (disabled) return;
    // setFocusedField(fieldName);
    console.log("Show something?");
    if (onFocus) {
      onFocus(fieldName);
    }
  };

  useEffect(() => {
    console.log("[JSON SCHEMA FORM] Values after update:", values);
  }, [values]);

  useEffect(() => {
    console.log("[JSON SCHEMA FORM] Fields after update:", fields);
  }, [fields]);

  console.log("[RENDERING JSON SCHEMA FORM]");
  console.log("Values:", values);

  const handleCursorChange = (e: React.SyntheticEvent, fieldName: string) => {
    if (disabled) return;
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
    if (disabled) return;

    if (GLOBAL_ACTIVE_FORM_NAME !== name) {
      console.log("Not the active form");
      return;
    }

    // Add check for JavaScript field type
    const activeField = fields.find((f: any) => f.name === GLOBAL_ACTIVE_FIELD);
    if (activeField?.inputType === "javascript_or_variable") {
      alert(
        "Variable insertion not supported for JavaScript fields. Use inputs.input_name in the javascript editor instead.",
      );
      return;
    }

    const values = valuesRef.current;
    console.log("Inserting variable:", variable);
    if (!GLOBAL_ACTIVE_FIELD || GLOBAL_CURSOR_LOCATION === null) {
      console.log("No active field or cursor position");
      return;
    }

    const currentValue = values[GLOBAL_ACTIVE_FIELD];
    console.log("[INSERT VARIABLE] Current value:", currentValue);

    // Handle JSON fields differently
    const field = fields.find((f: any) => f.name === GLOBAL_ACTIVE_FIELD);
    if (field?.inputType === "object") {
      // For JSON fields, we need to work with the string representation
      const stringValue =
        typeof currentValue === "string"
          ? currentValue
          : JSON.stringify(currentValue, null, 2);

      const beforeCursor = stringValue.slice(0, GLOBAL_CURSOR_LOCATION);
      const afterCursor = stringValue.slice(GLOBAL_CURSOR_LOCATION);
      const newValue = beforeCursor + variable + afterCursor;
      handleFieldChange(GLOBAL_ACTIVE_FIELD, newValue);
    } else {
      // Handle regular string fields
      const stringValue = currentValue?.toString() || "";
      const beforeCursor = stringValue.slice(0, GLOBAL_CURSOR_LOCATION);
      const afterCursor = stringValue.slice(GLOBAL_CURSOR_LOCATION);
      const newValue = beforeCursor + variable + afterCursor;
      handleFieldChange(GLOBAL_ACTIVE_FIELD, newValue);
    }
  };

  useEffect(() => {
    registerCallback(name, insertVariable);
    return () => {
      unRegisterCallback(name);
    };
  }, []);

  const isSubmitDisabled = disabled || invalidJsonFields.size > 0;

  return (
    <form
      name={name}
      onSubmit={handleSubmit}
      noValidate
      className="w-full overflow-hidden"
    >
      <div className="w-full overflow-hidden">
        {fields?.map((field: any) => {
          const { name: fieldName, inputType } = field;
          console.log("[DEBUG] Field mapping:", {
            fieldName,
            inputType,
            field,
          }); // Add this debug line

          const FieldComponent = fieldsMap[inputType] || fieldsMap.error;

          return (
            <div className="w-full overflow-hidden px-1" key={fieldName}>
              <FieldComponent
                value={values?.[fieldName]}
                error={errors[fieldName]}
                submited={submited}
                // type={field.type}
                onChange={handleFieldChange}
                onFocus={() => handleFieldFocus(fieldName)}
                onSelect={(e: any) => handleCursorChange(e, fieldName)}
                onClick={(e: any) => handleCursorChange(e, fieldName)}
                onKeyUp={(e: any) => handleCursorChange(e, fieldName)}
                onValueChange={(value: any) =>
                  handleFieldChange(fieldName, value)
                }
                disabled={disabled}
                name={field.name}
                label={field.label}
                options={field.options}
                description={field.description}
                isVisible={field.isVisible}
                required={field.required}
                provider={field.provider}
                showInputsExplorer={showInputsExplorer}
                showResultsExplorer={showResultsExplorer}
              />
            </div>
          );
        })}
        <div className="flex items-center gap-2 mt-2">
          <Button type="submit" variant={"default"} disabled={isSubmitDisabled}>
            Submit
          </Button>
          {/* {invalidJsonFields.size > 0 && (
            <span className="text-red-500">
              Please fix invalid JSON before submitting
            </span>
          )} */}
          {hasUnsavedChanges && (
            <>
              <TriangleAlertIcon className="w-4 h-4 text-yellow-400" />
              <span className="text-sm">Unsaved changes</span>
            </>
          )}
        </div>
      </div>
    </form>
  );
}
