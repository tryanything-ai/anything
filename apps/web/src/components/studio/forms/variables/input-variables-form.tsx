import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "../json-schema-form";
import { useAnything } from "@/context/AnythingContext";
import { useMemo } from "react";

export default function InputVariablesForm(): JSX.Element {
  const {
    workflow: {
      selected_node_variables,
      selected_node_variables_schema,
      updateNodeData,
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

      // Add detailed field logging
      console.log(
        "[INPUT VARIABLES FORM DEBUG] Created fields details:",
        result.fields.map((field: any) => ({
          name: field.name,
          type: field.type,
          inputType: field.inputType,
          isVisible: field.isVisible,
          default: field.default,
          value: field.value,
        })),
      );

      console.log(
        "[INPUT VARIABLES FORM DEBUG] Created fields:",
        result.fields,
      );
      return result;
    } else {
      console.log("[INPUT VARIABLES FORM DEBUG] Skipping field Creation");
    }
    return { fields: undefined, handleValidation: undefined };
  }, [selected_node_variables, selected_node_variables_schema]);

  //Update Configuration
  async function handleVariableInputSubmit(
    jsonValues: any,
    { formValues }: any,
  ) {
    console.log("[INPUT VARIABLES FORM] Submitting!", {
      formValues,
      jsonValues,
    });
    await updateNodeData(["variables"], [formValues]);
    console.log("Submitted!", { formValues, jsonValues });
  }

  return (
    <>
      {Object.keys(selected_node_variables_schema?.properties || {}).length ===
      0 ? (
        <div className="text-gray-500 py-2 ml-12">No variables yet</div>
      ) : (
        <JsonSchemaForm
          name="input-variables-form"
          onSubmit={handleVariableInputSubmit}
          fields={fields}
          onFocus={(fieldName: string) => {
            if (explorerTab !== "results") {
              setExplorerTab("results");
            }
            if (!showExplorer) {
              setShowExplorer(true);
            }
          }}
          initialValues={selected_node_variables}
          handleValidation={handleValidation}
        />
      )}
    </>
  );
}
