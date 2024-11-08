import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "../json-schema-form";
import { useAnything } from "@/context/AnythingContext";
import { EDIT_FLOW_SCHEMA } from "./flow-settings-schema";

export default function WorkflowSettingsForm(): JSX.Element {
  const {
    workflow: { db_flow, updateWorkflow },
  } = useAnything();

  let fields, handleValidation;

  //TODO: Create input
  let input = {
    flow_name: db_flow?.flow_name,
    description: db_flow?.description,
    // active: db_flow?.active,
  };

  console.log("input in worfklow-settings-form", input);

  ({ fields, handleValidation } = createHeadlessForm(EDIT_FLOW_SCHEMA, {
    strictInputType: false, // so you don't need to pass presentation.inputType,
    initialValues: input,
  }));

  async function handleOnSubmit(jsonValues: any, { formValues }: any) {
    await updateWorkflow(formValues);
    // await updateNodeData(["label", "description", "icon"], [formValues.label, formValues.description, formValues.icon])
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
