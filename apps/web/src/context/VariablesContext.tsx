"use client";

import { createContext, ReactNode, useEffect, useState } from "react";
import { cloneDeep } from "lodash";
import slugify from "slugify";
import { useWorkflowVersionContext } from "./WorkflowVersionProvider";
import {
  DEFAULT_VARIABLES_SCHEMA,
  isValidVariablesSchema,
} from "@/components/studio/forms/variables/edit-variable-schema";
export enum EditVariableFormMode {
  INPUT = "input",
  DELETE = "delete",
  EDIT = "edit",
}

export interface VariablesContextInterface {
  editingMode: EditVariableFormMode;
  selectedProperty: any;
  setSelectedProperty: (property: any) => void;
  setEditingMode: (mode: EditVariableFormMode) => void;
  updateVariablesProperty: (data: any) => Promise<boolean>;
  deleteVariable: (variableKey: string) => Promise<boolean>;
}

export const VariablesContext = createContext<VariablesContextInterface>({
  editingMode: EditVariableFormMode.INPUT,
  selectedProperty: null,
  setSelectedProperty: () => {},
  setEditingMode: () => {},
  updateVariablesProperty: () => Promise.resolve(false),
  deleteVariable: () => Promise.resolve(false),
});

export const VariablesProvider = ({
  children,
}: {
  children: ReactNode;
}): JSX.Element => {
  const {
    selected_node_variables,
    selected_node_variables_schema,
    selected_node_id,
    updateNodeData,
  } = useWorkflowVersionContext();
  const [editingMode, setEditingMode] = useState<EditVariableFormMode>(
    EditVariableFormMode.INPUT,
  );
  const [selectedProperty, setSelectedProperty] = useState<any>(null);

  const updateVariablesProperty = async (form_data: any) => {
    try {
      console.log(
        "[VARIABLES CONTEXT] Selected property -> ",
        selectedProperty,
      );

      if (selectedProperty) {
        console.log("Updating existing property");

        if (!selected_node_variables_schema) return false;
        if (!selected_node_variables_schema.properties) return false;

        let new_schema = cloneDeep(selected_node_variables_schema);

        console.log("Current Variables Schema to update: ", new_schema);

        //Merge incoming data with existing property
        new_schema.properties[selectedProperty.key] = {
          ...new_schema.properties[selectedProperty.key],
          ...form_data,
        };

        if (form_data.inputType !== "account") {
          delete new_schema.properties[selectedProperty.key][
            "x-jsf-presentation"
          ]["provider"];
        }

        console.log("Updated Variables Schema: ", new_schema);

        //update to Anyting Context and Db
        await updateNodeData(["variables_schema"], [new_schema]);
      } else {
        console.log("[VARIABLES CONTEXT] Creating new property");

        // Use variable schema or create one if necessary
        let variables_schema = isValidVariablesSchema(
          selected_node_variables_schema,
        )
          ? selected_node_variables_schema
          : cloneDeep(DEFAULT_VARIABLES_SCHEMA);
        console.log(
          "Variables schema after checking existing schema or creating new one: ",
          variables_schema,
        );

        let key = slugify(form_data.title, {
          replacement: "_", // replace spaces with replacement character, defaults to `-`
          lower: true, // convert to lower case, defaults to `false`
        });
        console.log("Generated key for new property: ", key);

        console.log("[VARIABLES PROIVDER] Form Data: ", form_data);
        // Create new property
        variables_schema.properties[key] = {
          title: form_data.title,
          description: form_data.description,
          type: form_data.type,
        };

        console.log(
          "Variables schema after adding new property: ",
          variables_schema,
        );

        // Make sure we add to order and required
        variables_schema["x-jsf-order"].push(key);
        variables_schema.required.push(key);
        console.log(
          "Variables schema after updating order and required fields: ",
          variables_schema,
        );

        //Add input type to let users select Accounts for example
        variables_schema.properties[key]["x-jsf-presentation"] = {
          inputType: form_data.type,
        };

        if (form_data.type === "account") {
          console.log("Adding provider to x-jsf-presentation");
          variables_schema.properties[key]["x-jsf-presentation"]["provider"] =
            form_data.provider;
        }

        // Need to add empty version to variables also
        let new_variables: any = {};
        // If we already have variables add to them.
        if (selected_node_variables) {
          new_variables = cloneDeep(selected_node_variables);
          console.log("Cloned existing variables: ", new_variables);
        }

        new_variables[key] = "";
        console.log("New variables after adding new key: ", new_variables);

        // Update to Anything Context and Db
        console.log(
          "Updating node data with new variables schema and variables",
        );
        await updateNodeData(
          ["variables_schema", "variables"],
          [variables_schema, new_variables],
        );
        console.log("Node data updated successfully");
      }
    } catch (e) {
      console.log("Error updating variables property: ", e);
      return false;
    } finally {
      setSelectedProperty(null);
    }
    return true;
  };

  const deleteVariable = async (variableKey: string) => {
    try {
      console.log("Deleting variable: ", variableKey);
      if (!selected_node_variables) return false;
      if (!selected_node_variables_schema) return false;

      console.log("Made it through checks in delete ");
      //deep copy schema
      let updated_schema = cloneDeep(selected_node_variables_schema);
      //deep variables
      let updated_variables = cloneDeep(selected_node_variables);

      // Delete the variable from schema
      delete updated_schema.properties[variableKey];
      // Delete the variable from variables
      delete updated_variables[variableKey];

      //Remove from order in schema
      const index = updated_schema["x-jsf-order"].indexOf(variableKey);
      if (index > -1) {
        updated_schema["x-jsf-order"].splice(index, 1);
      }

      //Remove from required in schema
      const reqIndex = updated_schema.required.indexOf(variableKey);
      if (reqIndex > -1) {
        updated_schema.required.splice(reqIndex, 1);
      }

      console.log("Variables after deleteVariable: ", updated_variables);
      console.log("Updated Schema after delete: ", updated_schema);

      // Update the database
      await updateNodeData(
        ["variables_schema", "variables"],
        [updated_schema, updated_variables],
      );

      return true;
    } catch (e) {
      console.log("Error deleting variable: ", e);
      return false;
    }
  };

  useEffect(() => {
    //Reset form to main view when we select differnt node ids
    setEditingMode(EditVariableFormMode.INPUT);
  }, [selected_node_id]);

  return (
    <VariablesContext.Provider
      value={{
        editingMode,
        setEditingMode,
        selectedProperty,
        setSelectedProperty,
        updateVariablesProperty,
        deleteVariable,
      }}
    >
      {children}
    </VariablesContext.Provider>
  );
};
