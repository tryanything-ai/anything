import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "../json-schema-form";
import { useAnything } from "@/context/AnythingContext";
import { EDIT_FLOW_SCHEMA } from "./flow-settings-schema";
import { useMemo } from "react";

export default function WorkflowSettingsForm(): JSX.Element {
  const {
    workflow: { db_flow, updateWorkflow },
  } = useAnything();

  //TODO: Create input
  let input = {
    flow_name: db_flow?.flow_name,
    description: db_flow?.description,
    // active: db_flow?.active,
  };

  console.log("input in worfklow-settings-form", input);

  const { fields, handleValidation } = useMemo(() => {
    let results = createHeadlessForm(EDIT_FLOW_SCHEMA, {
      initialValues: input,
    });
  // //Used to add "strict" feature kinda sideloading jsonschema form or our validation needs
  //   results.fields = results.fields.map((field: any) => ({
  //     ...field,
  //     strict: EDIT_FLOW_SCHEMA.properties[field.name]?.["x-any-validation"]?.strict || true,
  //   }));

    return results;
  }, [input]);
  async function handleOnSubmit(formValues: any) {
    await updateWorkflow(formValues);
    console.log("Submitted!", formValues);
  }

  return (
    <div className="rounded-lg border p-4">
      <JsonSchemaForm
        name="action-display-form"
        onSubmit={handleOnSubmit}
        fields={fields}
        initialValues={input}
        handleValidation={handleValidation}
        // onToggleStrictMode={(fieldName: string, strict: boolean) => toggleStrictMode(fieldName, strict)}
      />
    </div>
  );
}
