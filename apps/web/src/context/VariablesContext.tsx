"use client";

import {
  createContext,
  ReactNode,
  useEffect,
  useState,
  useContext,
} from "react";
import { cloneDeep } from "lodash";
import slugify from "slugify";
import { useWorkflowVersion } from "./WorkflowVersionProvider";
import {
  DEFAULT_VARIABLES_SCHEMA,
  isValidVariablesSchema,
  VARIABLE_TYPES_JSF_PRESENTATION_AND_ANY_VALIDATION,
} from "@/components/studio/forms/variables/create-variable-schema";
export enum EditVariableFormMode {
  INPUT = "input",
  DELETE = "delete",
  EDIT = "edit",
}

export interface VariablesContextInterface {
  editingMode: EditVariableFormMode;
  selectedProperty: any;
  isFormVisible: boolean;
  setSelectedProperty: (property: any) => void;
  setEditingMode: (mode: EditVariableFormMode) => void;
  setIsFormVisible: (visible: boolean) => void;
  updateVariablesProperty: (data: any) => Promise<boolean>;
  deleteVariable: (variableKey: string) => Promise<boolean>;
}

export const VariablesContext = createContext<VariablesContextInterface>({
  editingMode: EditVariableFormMode.INPUT,
  selectedProperty: null,
  isFormVisible: false,
  setSelectedProperty: () => {},
  setEditingMode: () => {},
  setIsFormVisible: () => {},
  updateVariablesProperty: () => Promise.resolve(false),
  deleteVariable: () => Promise.resolve(false),
});

export const useVariables = () => useContext(VariablesContext);

export const VariablesProvider = ({
  children,
}: {
  children: ReactNode;
}): JSX.Element => {
  const {
    selected_node_inputs,
    selected_node_inputs_schema,
    selected_action_id,
    updateNodeData,
  } = useWorkflowVersion();
  const [editingMode, setEditingMode] = useState<EditVariableFormMode>(
    EditVariableFormMode.INPUT,
  );
  const [selectedProperty, setSelectedProperty] = useState<any>(null);
  const [isFormVisible, setIsFormVisible] = useState<boolean>(false);

  const updateVariablesSchemaProperties = async (form_data: any) => {
    try {
      console.log(
        "[VARIABLES CONTEXT] Selected property -> ",
        selectedProperty,
      );

      console.log("[VARIABLES CONTEXT] Form Data: ", form_data);

      if (selectedProperty) {
        console.log("[VARIABLES CONTEXT] Updating existing property");

        if (!selected_node_inputs_schema) return false;
        if (!selected_node_inputs_schema.properties) return false;

        let new_schema = cloneDeep(selected_node_inputs_schema);

        console.log(
          "[VARIABLES CONTEXT] Current Inputs Schema to update: ",
          new_schema,
        );

        //Merge incoming data with existing property and get x-jsf-presentation from VARIABLE_TYPES
        new_schema.properties[selectedProperty.key] = {
          ...new_schema.properties[selectedProperty.key],
          type: VARIABLE_TYPES_JSF_PRESENTATION_AND_ANY_VALIDATION[
            form_data.type
          ].type,
          "x-jsf-presentation": {
            ...VARIABLE_TYPES_JSF_PRESENTATION_AND_ANY_VALIDATION[
              form_data.type
            ]["x-jsf-presentation"],
          },
          "x-any-validation": {
            ...VARIABLE_TYPES_JSF_PRESENTATION_AND_ANY_VALIDATION[
              form_data.type
            ]["x-any-validation"],
          },
        };

        //Manage Account Provider
        if (form_data.type === "account") {
          console.log(
            "[VARIABLES CONTEXT] Adding provider to x-jsf-presentation",
          );
          new_schema.properties[selectedProperty.key]["x-jsf-presentation"][
            "provider"
          ] = form_data.provider;
        } else {
          delete new_schema.properties[selectedProperty.key][
            "x-jsf-presentation"
          ]["provider"];
        }

        console.log(
          "[VARIABLES CONTEXT] Updated Variables Schema: ",
          new_schema,
        );

        //update to Anyting Context and Db
        await updateNodeData(["inputs_schema"], [new_schema]);
      } else {
        console.log("[VARIABLES CONTEXT] Creating new property");

        // Use variable schema or create one if necessary
        let inputs_schema = isValidVariablesSchema(
          selected_node_inputs_schema,
        )
          ? selected_node_inputs_schema
          : cloneDeep(DEFAULT_VARIABLES_SCHEMA);
        console.log(
          "[VARIABLES CONTEXT] Inputs schema after checking existing schema or creating new one: ",
          inputs_schema,
        );

        let key = slugify(form_data.title, {
          replacement: "_", // replace spaces with replacement character, defaults to `-`
        });
        console.log(
          "[VARIABLES CONTEXT] Generated key for new property: ",
          key,
        );

        console.log("[VARIABLES CONTEXT] Form Data: ", form_data);
        // Create new property
        inputs_schema.properties[key] = {
          title: key,
          description: "",
          type: VARIABLE_TYPES_JSF_PRESENTATION_AND_ANY_VALIDATION[
            form_data.type
          ].type,
          "x-jsf-presentation": {
            ...VARIABLE_TYPES_JSF_PRESENTATION_AND_ANY_VALIDATION[
              form_data.type
            ]["x-jsf-presentation"],
          },
          "x-any-validation": {
            ...VARIABLE_TYPES_JSF_PRESENTATION_AND_ANY_VALIDATION[
              form_data.type
            ]["x-any-validation"],
          },
        };

        console.log(
          "[VARIABLES CONTEXT] Inputs schema after adding new property: ",
          inputs_schema,
        );

        // Make sure we add to order and required
        inputs_schema["x-jsf-order"].push(key);
        inputs_schema.required.push(key);
        console.log(
          "[VARIABLES CONTEXT] Inputs schema after updating order and required fields: ",
          inputs_schema,
        );

        //Add input type to let users select Accounts for example
        // variables_schema.properties[key]["x-jsf-presentation"] = {
        //   inputType: form_data["x-jsf-presentation"].inputType,
        // };

        if (form_data.type === "account") {
          console.log(
            "[VARIABLES CONTEXT] Adding provider to x-jsf-presentation",
          );
          inputs_schema.properties[key]["x-jsf-presentation"]["provider"] =
            form_data.provider;
        }

        // Need to add empty version to variables also
        let new_inputs: any = {};
        // If we already have variables add to them.
        if (selected_node_inputs) {
          new_inputs = cloneDeep(selected_node_inputs);
          console.log(
            "[VARIABLES CONTEXT] Cloned existing inputs: ",
            new_inputs,
          );
        }

        new_inputs[key] =
          VARIABLE_TYPES_JSF_PRESENTATION_AND_ANY_VALIDATION[form_data.type]
            .default || "";
        console.log(
          "[VARIABLES CONTEXT] New inputs after adding new key: ",
          new_inputs,
        );

        // Update to Anything Context and Db
        console.log(
          "[VARIABLES CONTEXT] Updating node data with new variables schema and variables",
        );
        await updateNodeData(["inputs_schema", "inputs"], [inputs_schema, new_inputs]);
        console.log("[VARIABLES CONTEXT] Node data updated successfully");
      }
    } catch (e) {
      console.log("[VARIABLES CONTEXT] Error updating variables property: ", e);
      return false;
    } finally {
      setSelectedProperty(null);
    }
    return true;
  };

  const deleteVariable = async (variableKey: string) => {
    try {
      console.log("[VARIABLES CONTEXT] Deleting variable: ", variableKey);
      if (!selected_node_variables) return false;
      if (!selected_node_variables_schema) return false;

      console.log("[VARIABLES CONTEXT] Made it through checks in delete ");
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

      console.log(
        "[VARIABLES CONTEXT] Variables after deleteVariable: ",
        updated_variables,
      );
      console.log(
        "[VARIABLES CONTEXT] Updated Schema after delete: ",
        updated_schema,
      );

      // Update the database
      await updateNodeData(
        ["variables_schema", "variables"],
        [updated_schema, updated_variables],
      );

      return true;
    } catch (e) {
      console.log("[VARIABLES CONTEXT] Error deleting variable: ", e);
      return false;
    }
  };

  useEffect(() => {
    //Reset form to main view when we select differnt node ids
    setEditingMode(EditVariableFormMode.INPUT);
  }, [selected_action_id]);

  return (
    <VariablesContext.Provider
      value={{
        editingMode,
        setEditingMode,
        selectedProperty,
        setSelectedProperty,
        isFormVisible,
        setIsFormVisible,
        updateVariablesProperty: updateVariablesSchemaProperties,
        deleteVariable,
      }}
    >
      {children}
    </VariablesContext.Provider>
  );
};
