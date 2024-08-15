import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "./variables/json-schema-form";
import { useAnything } from "@/context/AnythingContext";

export default function ConfigurationForm({
  input_schema,
  input,
  validate,
}: any): JSX.Element {
  const { workflow } = useAnything();

  let fields, handleValidation: any;

  if (input_schema && Object.keys(input_schema).length > 0) {
    ({ fields, handleValidation } = createHeadlessForm(input_schema, {
      strictInputType: false, // so you don't need to pass presentation.inputType,
      initialValues: input,
    }));
  }

  async function handleOnSubmit(jsonValues: any, { formValues }: any) {
    await workflow.updateNodeData(["input"], [formValues]);
    console.log("Submitted!", { formValues, jsonValues });
  }

  return (
    <>
      {input_schema &&
        Object.keys(input_schema).length > 0 &&
        Object.keys(input).length > 0 && (
          <div className="rounded-lg border p-4">
            <div className="flex flex-row items-center">
              <div className="font-bold">Configuration</div>
              <div className="flex-1" />
              {/* <Button variant={"link"} onClick={action}>{link_button_text}</Button> */}
            </div>
            <JsonSchemaForm
              name="configuration-form"
              onSubmit={handleOnSubmit}
              fields={fields}
              initialValues={input}
              handleValidation={handleValidation}
            />
          </div>
        )}
    </>
  );
}
