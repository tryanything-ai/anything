import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "../variables/json-schema-form";
import { useAnything } from "@/context/AnythingContext";
import { EDIT_ACTION_DISPLAY_SCHEMA } from "./edit-display-schema";

export default function ActionDisplayForm() {
    const { workflow: { selected_node_data, updateNodeData } } = useAnything();

    let fields, handleValidation;

    //TODO: Create input
    let input = { label: selected_node_data?.label, "descriptionn": selected_node_data?.description, "icon": selected_node_data?.icon };

    ({ fields, handleValidation } = createHeadlessForm(EDIT_ACTION_DISPLAY_SCHEMA, {
        strictInputType: false, // so you don't need to pass presentation.inputType,
        initialValues: input,
    }));

    async function handleOnSubmit(jsonValues: any, { formValues }: any) {
        await updateNodeData(["label", "description", "icon"], [formValues.label, formValues.description, formValues.icon])
        console.log("Submitted!", { formValues, jsonValues });
    }

    return (
        <div className="rounded-lg border p-4">
            <JsonSchemaForm
                name="action-display-form"
                onSubmit={handleOnSubmit}
                fields={fields}
                initialValues={input}
                handleValidation={handleValidation}
            />
        </div>
    );
}