import { ChevronRight } from "lucide-react";
import { useState } from "react";
import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "./json-schema-form";
import { useAnything } from "@/context/AnythingContext";
import { useMemo } from "react";
import { Lock } from "lucide-react";
import { Button } from "@repo/ui/components/ui/button";

export default function ConfigurationForm(): JSX.Element {
  const {
    workflow: {
      selected_node_input,
      selected_node_data,
      selected_node_input_schema,
      updateNodeData,
      explorerTab,
      setExplorerTab,
      showExplorer,
      setShowExplorer,
    },
  } = useAnything();

  const [isCollapsed, setIsCollapsed] = useState(
    selected_node_data?.input_locked ?? false,
  );

  const { fields, handleValidation } = useMemo(() => {
    if (
      selected_node_input &&
      selected_node_input_schema &&
      Object.keys(selected_node_input_schema).length > 0
    ) {
      console.log("[CREATING HEADLESS FORM FOR INPUT SCHEMA]");
      console.log("Selected Node Input:", selected_node_input);
      console.log("Selected Node Input Schema:", selected_node_input_schema);
      const result = createHeadlessForm(selected_node_input_schema, {
        initialValues: selected_node_input,
      });

      // Add detailed field logging
      console.log(
        "[CONFIGURATION FORM DEBUG] Created fields details:",
        result.fields.map((field: any) => ({
          name: field.name,
          type: field.type,
          inputType: field.inputType,
          isVisible: field.isVisible,
          default: field.default,
          value: field.value,
        })),
      );

      return result;
    } else {
      console.log("[CONFIGURATION FORM DEBUG] Skipping field Creation");
    }
    return { fields: undefined, handleValidation: undefined };
  }, [selected_node_input, selected_node_input_schema]);

  async function handleOnSubmit(jsonValues: any, { formValues }: any) {
    await updateNodeData(["input"], [formValues]);
    console.log("[CONFIGURATION FORM] Submitted!", { formValues, jsonValues });
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
        <div
          className="flex flex-row items-center cursor-pointer"
          onClick={() => setIsCollapsed(!isCollapsed)}
        >
          <ChevronRight
            className={`h-4 w-4 transition-transform mr-2 ${!isCollapsed ? "rotate-90" : ""}`}
          />
          <div className="font-bold">Configuration</div>
          <div className="flex-1" />
          {selected_node_data?.input_locked && (
            <Lock size={16} className="text-gray-400" />
          )}
        </div>
        {!isCollapsed && (
          <JsonSchemaForm
            name="configuration-form"
            onSubmit={handleOnSubmit}
            fields={fields}
            formContext="configuration"
            showVariablesExplorer={true}
            showResultsExplorer={false}
            onFocus={(fieldName: string) => {
              if (explorerTab !== "variables") {
                setExplorerTab("variables");
              }
              if (!showExplorer) {
                setShowExplorer(true);
              }
            }}
            disabled={selected_node_data?.input_locked}
            initialValues={selected_node_input}
            handleValidation={handleValidation}
          />
        )}
      </div>
      {/* )} */}
    </>
  );
}
