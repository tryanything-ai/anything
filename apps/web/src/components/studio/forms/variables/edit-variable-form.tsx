import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { EDIT_VARIABLES_SCHEMA, EDIT_VARIABLES_VARIABLES } from "./edit-variable-schema";
import { JsonSchemaForm } from "./json-schema-form";
import { useAnything } from "@/context/AnythingContext";

// type OrderedVariable = {
//     name: string;
//     title: string;
//     description: string;
//     type: string;
//     oneOf?: { value: string; title: string }[];
//     "x-jsf-presentation"?: { inputType: string };
// };

function extractObjectValues(obj: Record<string, any> | null, keys: string[]): Record<string, any> {
    if (obj === null) {
        return {};
    }

    const extractedObject: Record<string, any> = {};
    keys.forEach(key => {
        if (obj.hasOwnProperty(key)) {
            extractedObject[key] = obj[key];
        }
    });
    return extractedObject;
}

// Edit a single variable
export default function EditVariableForm() {

    const { variables } = useAnything();
    // console.log("Variable in EditVariableForm", variable);
    let the_variable = { ...EDIT_VARIABLES_VARIABLES, ...extractObjectValues(variables.selectedProperty, Object.keys(EDIT_VARIABLES_VARIABLES)) };

    console.log("The Variable", the_variable);

    const { fields, handleValidation } = createHeadlessForm(EDIT_VARIABLES_SCHEMA, {
        strictInputType: false, // so you don't need to pass presentation.inputType,
        initialValues: the_variable,
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
        <JsonSchemaForm
            name="edit-single-variable-form"
            onSubmit={handleOnSubmit}
            fields={fields}
            initialValues={EDIT_VARIABLES_VARIABLES}
            handleValidation={handleValidation}
        />
    );
}
