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
      Object.keys(selected_node_variables_schema.properties).length > 0
    ) {
      return createHeadlessForm(selected_node_variables_schema, {
        strictInputType: false,
        initialValues: selected_node_variables,
      });
    }
    return { fields: undefined, handleValidation: undefined };
  }, [selected_node_variables, selected_node_variables_schema]);

  //Update Configuration
  async function handleVariableInputSubmit(
    jsonValues: any,
    { formValues }: any,
  ) {
    await updateNodeData(["variables"], [formValues]);
    console.log("Submitted!", { formValues, jsonValues });
  }

  return (
    <>
      {selected_node_variables &&
        typeof selected_node_variables === "object" &&
        selected_node_variables_schema &&
        typeof selected_node_variables_schema === "object" &&
        Object.keys(selected_node_variables_schema.properties).length > 0 && (
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
