import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "./json-schema-form";
import { useAnything } from "@/context/AnythingContext";

export default function ConfigurationForm(): JSX.Element {
  const {
    workflow: {
      selected_node_input,
      selected_node_input_schema,
      updateNodeData,
    },
  } = useAnything();

  let fields, handleValidation: any;

  if (
    selected_node_input &&
    selected_node_input_schema &&
    Object.keys(selected_node_input_schema).length > 0
  ) {
    console.log("[CREATING HEADLESS FORM FOR INPUT SCHEMA]");
    console.log("Selected Node Input:", selected_node_input);
    console.log("Selected Node Input Schema:", selected_node_input_schema);
    ({ fields, handleValidation } = createHeadlessForm(
      selected_node_input_schema,
      {
        strictInputType: false, // so you don't need to pass presentation.inputType,
        initialValues: selected_node_input,
      },
    ));
  } else {
    console.log("[NO INPUT SCHEMA IN CONFIGURATION FORM]");
  }

  async function handleOnSubmit(jsonValues: any, { formValues }: any) {
    await updateNodeData(["input"], [formValues]);
    console.log("Submitted!", { formValues, jsonValues });
  }

  console.log("[RENDERING INPUTS FORM]");
  console.log("Fields:", fields);

  return (
    <>
      {/* {selected_node_input_schema &&
        selected_node_input &&
        Object.keys(selected_node_input_schema).length > 0 &&
        Object.keys(selected_node_input).length > 0 && ( */}
      <div className="rounded-lg border p-4">
        <div className="flex flex-row items-center">
          <div className="font-bold">Configuration</div>
          <div className="flex-1" />
        </div>
        <JsonSchemaForm
          name="configuration-form"
          onSubmit={handleOnSubmit}
          fields={fields}
          initialValues={selected_node_input}
          handleValidation={handleValidation}
        />
      </div>
      {/* )} */}
    </>
  );
}
