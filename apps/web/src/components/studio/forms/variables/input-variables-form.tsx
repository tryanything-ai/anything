import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "../json-schema-form";
import { useAnything } from "@/context/AnythingContext";
import { useMemo } from "react";

export default function InputVariablesForm(): JSX.Element {
  const {
    workflow: {
      selected_node_inputs: selected_node_variables,
      selected_node_inputs_schema: selected_node_variables_schema,
      updateNodeData,
      updateInputStrictMode,
      setShowExplorer,
      explorerTab,
      showExplorer,
      setExplorerTab,
    },
  } = useAnything();

  const { fields, handleValidation } = useMemo(() => {
    if (
      selected_node_variables &&
      typeof selected_node_variables === "object" &&
      selected_node_variables_schema &&
      typeof selected_node_variables_schema === "object" &&
      selected_node_variables_schema.properties &&
      Object.keys(selected_node_variables_schema.properties || {}).length > 0
    ) {
      console.log(
        "[INPUT VARIABLES FORM DEBUG] Schema:",
        selected_node_variables_schema,
      );
      console.log(
        "[INPUT VARIABLES FORM DEBUG] Initial values:",
        selected_node_variables,
      );

      // Add logging for schema properties
      console.log(
        "[INPUT VARIABLES FORM DEBUG] Schema properties:",
        Object.keys(selected_node_variables_schema.properties),
      );

      const result = createHeadlessForm(selected_node_variables_schema, {
        initialValues: selected_node_variables,
      });

      //Used to add "strict" feature kinda sideloading jsonschema form or our validation needs
      result.fields = result.fields.map((field: any) => ({
        ...field,
        strict:
          selected_node_variables_schema.properties[field.name]?.[
            "x-any-validation"
          ]?.strict || true,
      }));

      return result;
    } else {
      console.log("[INPUT VARIABLES FORM DEBUG] Skipping field Creation");
    }
    return { fields: undefined, handleValidation: undefined };
  }, [selected_node_variables, selected_node_variables_schema]);

  //Update Configuration
  async function handleOnSubmit(formValues: any) {
    console.log("[INPUT VARIABLES FORM] Submitting!", formValues);

    await updateNodeData(["inputs"], [formValues]);
  }

  async function toggleStrictMode(field_name: string, strict: boolean) {
    await updateInputStrictMode("inputs_schema", field_name, strict);
    console.log(
      "[INPUT VARIABLES FORM] Toggled Strict Mode!",
      field_name,
      strict,
    );
  }

  return (
    <>
      {Object.keys(selected_node_variables_schema?.properties || {}).length ===
      0 ? (
        <div className="text-gray-500 py-2 ml-12">No inputs yet</div>
      ) : (
        <JsonSchemaForm
          name="input-variables-form"
          onSubmit={handleOnSubmit} 
          fields={fields}
          // onFocus={(fieldName: string) => {
          //   if (explorerTab !== "results") {
          //     setExplorerTab("results");
          //   }
          //   if (!showExplorer) {
          //     setShowExplorer(true);
          //   }
          // }}
          onToggleStrictMode={(fieldName: string, strict: boolean) =>
            toggleStrictMode(fieldName, strict)
          }
          initialValues={selected_node_variables}
          handleValidation={handleValidation}
          showInputsExplorer={false}
          showResultsExplorer={true}
        />
      )}
    </>
  );
}
