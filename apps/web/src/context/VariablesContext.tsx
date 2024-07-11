"use client"

import {
    createContext,
    ReactNode,
    useEffect,
    useContext,
    useState,
} from "react";
import { cloneDeep } from 'lodash';
// import { useMemo } from 'react';
import slugify from "slugify";
import { useWorkflowVersionContext, WorkflowVersionContextInterface } from "./WorkflowVersionProvider";
import { DEFAULT_VARIABLES_SCHEMA } from "@/components/studio/forms/variables/edit-variable-schema";
export enum EditVariableFormMode {
    INPUT = "input",
    DELETE = "delete",
    EDIT = "edit"
}

export interface VariablesContextInterface {
    editingMode: EditVariableFormMode;
    selectedProperty: any;
    setSelectedProperty: (property: any) => void;
    setEditingMode: (mode: EditVariableFormMode) => void;
    updateVariablesProperty: (data: any) => Promise<boolean>;
    deleteVariable: (variableName: string) => Promise<boolean>;
    // variables: any;
    // variables_schema: any;
}

export const VariablesContext = createContext<VariablesContextInterface>({
    editingMode: EditVariableFormMode.INPUT,
    selectedProperty: null,
    setSelectedProperty: () => { },
    setEditingMode: () => { },
    updateVariablesProperty: () => Promise.resolve(false),
    deleteVariable: () => Promise.resolve(false),
    // variables: {},
    // variables_schema: {},
})

// export const userVariablesContext = () => useContext(VariablesContext);

// const selectedVariables = (state: WorkflowVersionContextInterface) => {
//     return state.selected_node_data?.variables;
// };

// const selectedVariablesSchema = (state: WorkflowVersionContextInterface) => {
//     return state.selected_node_data?.variables_schema;
// }

// // Memoized selector using useMemo
// export const useVariables = (state: WorkflowVersionContextInterface) => {
//     return useMemo(() => selectedVariables(state), [state]);
//   };

// // Memoized selector using useMemo
// export const userVariablesSchema = (state: WorkflowVersionContextInterface) => {
//     return useMemo(() => selectedVariablesSchema(state), [state]);
//   };

export const VariablesProvider = ({ children }: { children: ReactNode }) => {

    const { selected_node_data, selected_node_id, updateNodeData } = useWorkflowVersionContext();
    const [editingMode, setEditingMode] = useState<EditVariableFormMode>(EditVariableFormMode.INPUT);
    const [selectedProperty, setSelectedProperty] = useState<any>(null)

    // const variables = selected_node_data?.variables; // || {}; 
    // const variables_schema = selected_node_data?.variables_schema;  // \\ DEFAULT_VARIABLES_SCHEMA;

    const updateVariablesProperty = async (form_data: any) => {
        try {
            console.log("Selected property -> ", selectedProperty);

            if (selectedProperty) {
                console.log("Updating existing property");

                if (!selected_node_data) return false;
                if (!selected_node_data.variables_schema) return false;
                if (!selected_node_data.variables_schema.properties) return false;

                let new_schema = cloneDeep(selected_node_data.variables_schema);
                //Merge incoming data with existing property
                new_schema.properties[selectedProperty.key] = { ...new_schema.properties[selectedProperty.key], ...form_data };

                console.log("Updating variables property -> New Variables Schema: ", new_schema);

                //update to Anyting Context and Db
                await updateNodeData(["variables_schema"], [new_schema]);

            } else {
                console.log("Creating new property");

                //Use variable schema or create one if necessary
                let variables_schema = selected_node_data?.variables_schema || cloneDeep(DEFAULT_VARIABLES_SCHEMA);

                let key = slugify(form_data.title, {
                    replacement: '_',  // replace spaces with replacement character, defaults to `-`
                    lower: true,      // convert to lower case, defaults to `false`
                })

                //Create new property
                variables_schema.properties[key] = form_data;

                //Make sure we add to order and required
                variables_schema["x-jsf-order"].push(key);
                variables_schema.required.push(key);

                //Need to add empty version to variables also
                let new_variables = selected_node_data?.variables || {};
                new_variables[key] = form_data.type === "number" ? 0 : "";

                //update to Anyting Context and Db
                await updateNodeData(["variables_schema", "variables"], [variables_schema, new_variables]);
            }

        } catch (e) {
            console.log("Error updating variables property: ", e);
            return false;
        } finally {
            setSelectedProperty(null);
        }
        return true;
    }

    const deleteVariable = async (variableKey: string) => {
        try {
            console.log("Deleting variable: ", variableKey);
            if (!selected_node_data) return false;
            if (!selected_node_data.variables) return false;
            if (!selected_node_data.variables_schema) return false;

            console.log("Made it through checks in delete ");
            //deep copy
            let updated_schema = cloneDeep(selected_node_data.variables_schema);

            // Delete the variable from properties
            delete updated_schema.properties[variableKey];

            const index = updated_schema["x-jsf-order"].indexOf(variableKey);
            if (index > -1) {
                updated_schema["x-jsf-order"].splice(index, 1);
            }

            const reqIndex = updated_schema.required.indexOf(variableKey);
            if (reqIndex > -1) {
                updated_schema.required.splice(reqIndex, 1);
            }

            //Remove the key from the variables
            let updated_variables = cloneDeep(selected_node_data.variables);

            delete updated_variables[variableKey];
            console.log("Variables after deleteVariable: ", updated_variables);

            console.log("Updated Schema after delete: ", updated_schema);
            // Update the database
            await updateNodeData(["variables_schema", "variables"], [updated_schema, updated_variables]);

            return true;
        } catch (e) {
            console.log("Error deleting variable: ", e);
            return false;
        }
    };

    useEffect(() => {
        //Reset form to main view when we select differnt node ids
        setEditingMode(EditVariableFormMode.INPUT);
    }, [selected_node_id])

    return (
        <VariablesContext.Provider
            value={{
                // variables,
                // variables_schema,
                editingMode,
                setEditingMode,
                selectedProperty,
                setSelectedProperty,
                updateVariablesProperty,
                deleteVariable
            }}
        >
            {children}
        </VariablesContext.Provider>
    );
};
