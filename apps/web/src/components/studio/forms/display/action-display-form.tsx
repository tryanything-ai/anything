import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "../json-schema-form";
import { useAnything } from "@/context/AnythingContext";
import { EDIT_ACTION_DISPLAY_SCHEMA } from "./edit-display-schema";
import { useMemo } from "react";

export default function ActionDisplayForm(): JSX.Element {
  const {
    workflow: { selected_node_data, updateNodeData },
  } = useAnything();

  //TODO: Create input
  let input = {
    label: selected_node_data?.label,
    descriptionn: selected_node_data?.description,
    icon: selected_node_data?.icon,
  };

  const { fields, handleValidation } = useMemo(() => {
    let results = createHeadlessForm(EDIT_ACTION_DISPLAY_SCHEMA, {
      initialValues: input,
    });
    //Used to add "strict" feature kinda sideloading jsonschema form or our validation needs
    results.fields = results.fields.map((field: any) => ({
      ...field,
      strict:
        EDIT_ACTION_DISPLAY_SCHEMA.properties[field.name]?.["x-any-validation"]
          ?.strict || true,
    }));

    return results;
  }, [input]);

  async function handleOnSubmit(formValues: any) {
    await updateNodeData(
      ["label", "description", "icon"],
      [formValues.label, formValues.description, formValues.icon],
    );
    console.log("Submitted!", formValues);
  }

  // async function toggleStrictMode(field_name: string, strict: boolean) {
  //   await updateNodeData(
  //     [
  //       "plugin_config_schema",
  //       "properties",
  //       field_name,
  //       "x-any-validation",
  //       "strict",
  //     ],
  //     [strict],
  //   );
  //   console.log(
  //     "[ACTION DISPLAY FORM] Toggled Strict Mode!",
  //     field_name,
  //     strict,
  //   );
  // }

  return (
    <div className="rounded-lg border p-4">
      <JsonSchemaForm
        name="action-display-form"
        onSubmit={handleOnSubmit}
        fields={fields}
        initialValues={input}
        handleValidation={handleValidation}
        // onToggleStrictMode={(fieldName: string, strict: boolean) =>
        //   toggleStrictMode(fieldName, strict)
        // }
      />
    </div>
  );
}
