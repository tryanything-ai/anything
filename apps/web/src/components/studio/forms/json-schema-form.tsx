import { useState } from "react";

// import "./styles.css";
import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { formValuesToJsonValues, getDefaultValuesFromFields } from "@/lib/json-schema-utils";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
// import {
//   Box,
//   Stack,
//   LabelRadio,
//   RadioOptions,
//   Fieldset,
//   InputText,
//   Hint,
//   ErrorMessage,
//   Label
// } from "./App.styled";

const fieldsMap: { [key: string]: any } = {
    text: FieldText,
    // number: FieldNumber,
    // radio: FieldRadio,
    // error: FieldUnknown
};

// const fieldsMap = {
// //   text: FieldText,
// //   number: FieldNumber,
// //   radio: FieldRadio,
// //   error: FieldUnknown
// };

// const initialValuesFromAPI = {
//   name: 'Mega team'
// }

export default function WithReact({ input_schema, input }: any) {
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

    console.log("fields in form", fields);

    return (
        <article>
            <h1>json-schema-form + React</h1>
            <p>This demo uses React without any other Form library.</p>
            <br />

            <SmartForm
                name="my-form"
                onSubmit={handleOnSubmit}
                // From JSF
                fields={fields}
                initialValues={input}
                handleValidation={handleValidation}
            />
        </article>
    );
}

// ===============================
// ====== UI COMPONENTS ==========
// ===============================

function SmartForm({ name, fields, initialValues, handleValidation, onSubmit }: any) {
    // const [values, setValues] = useState(() =>
    //     getDefaultValuesFromFields(fields, initialValues)
    // );
    // const [errors, setErrors] = useState({});

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
                            {...field}
                        />
                    );
                })}

                <button type="submit">Submit</button>
            </div>
        </form>
    );
}

function FieldText({
    type,
    name,
    label,
    description,
    value,
    isVisible,
    error,
    submited,
    onChange,
    required,
    ...props
}: any) {
    const [touched, setTouched] = useState(false);

    if (!isVisible) return null;

    function handleChange(e: any) {
        if (!touched) setTouched(true);
        onChange(name, e.target.value);
    }

    return (
        <div>
            <Label htmlFor={name}>{label}</Label>
            {/* {description && <Hint id={`${name}-description`}>{description}</Hint>} */}
            <Input
                id={name}
                type="text"
                defaultValue={value}
                onChange={handleChange}
                aria-invalid={!!error}
                aria-describedby={`${name}-error ${name}-description`}
                aria-required={required}
                {...props}
            />
            {(touched || submited) && error && (
                <div id={`${name}-error`}>{error}</div>
            )}
        </div>
    );
}

// function FieldNumber(props) {
//     return (
//         <FieldText
//             inputMode="decimal"
//             // accepts numbers and dots (eg 10, 15.50)
//             pattern="^[0-9.]*$"
//             {...props}
//         />
//     );
// }

// function FieldRadio({
//     name,
//     label,
//     description,
//     value,
//     options,
//     isVisible,
//     error,
//     submited,
//     onChange
// }) {
//     const [touched, setTouched] = useState(false);

//     if (!isVisible) return null;

//     function handleChange(e) {
//         if (!touched) setTouched(true);
//         onChange(name, e.target.value);
//     }

//     const displayError = submited || touched ? error : null;

//     return (
//         <Fieldset key={name}>
//             {/* A11Y errors: https://blog.tenon.io/accessible-validation-of-checkbox-and-radiobutton-groups/ */}
//             <Label as="legend" aria-label={`${label} ${displayError}`}>
//                 {label}
//             </Label>
//             {description && <Hint>{description}</Hint>}
//             <RadioOptions onChange={handleChange}>
//                 {options.map((opt) => (
//                     <LabelRadio key={opt.value}>
//                         <input
//                             type="radio"
//                             name={name}
//                             value={opt.value}
//                             defaultChecked={value === opt.value}
//                         />
//                         {opt.label}
//                     </LabelRadio>
//                 ))}
//             </RadioOptions>
//             {displayError && <ErrorMessage>{displayError}</ErrorMessage>}
//         </Fieldset>
//     );
// }

// function FieldUnknown({ type, name, error }) {
//     return (
//         <p style={{ border: "1px dashed gray", padding: "8px" }}>
//             Field "{name}" unsupported: The type "{type}" has no UI component built
//             yet.
//             {error && <ErrorMessage id={`${name}-error`}>{error}</ErrorMessage>}
//         </p>
//     );
// }
