"use client"

import {
    createContext,
    ReactNode,
    useEffect,
    useState,
} from "react";
import { cloneDeep } from 'lodash';
import slugify from "slugify";
import { useWorkflowVersionContext } from "./WorkflowVersionProvider";
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
    deleteVariable: (variableKey: string) => Promise<boolean>;
}

export const VariablesContext = createContext<VariablesContextInterface>({
    editingMode: EditVariableFormMode.INPUT,
    selectedProperty: null,
    setSelectedProperty: () => { },
    setEditingMode: () => { },
    updateVariablesProperty: () => Promise.resolve(false),
    deleteVariable: () => Promise.resolve(false),
})

export const VariablesProvider = ({ children }: { children: ReactNode }) => {

    const { selected_node_variables, selected_node_variables_schema, selected_node_id, updateNodeData } = useWorkflowVersionContext();
    const [editingMode, setEditingMode] = useState<EditVariableFormMode>(EditVariableFormMode.INPUT);
    const [selectedProperty, setSelectedProperty] = useState<any>(null)

    const updateVariablesProperty = async (form_data: any) => {
        try {
            console.log("Selected property -> ", selectedProperty);

            if (selectedProperty) {
                console.log("Updating existing property");

                if (!selected_node_variables_schema) return false;
                if (!selected_node_variables_schema.properties) return false;

                let new_schema = cloneDeep(selected_node_variables_schema);

                //Merge incoming data with existing property
                new_schema.properties[selectedProperty.key] = { ...new_schema.properties[selectedProperty.key], ...form_data };

                console.log("Updating variables property -> New Variables Schema: ", new_schema);

                //update to Anyting Context and Db
                await updateNodeData(["variables_schema"], [new_schema]);

            } else {
                console.log("Creating new property");

                //Use variable schema or create one if necessary
                let variables_schema = selected_node_variables_schema || cloneDeep(DEFAULT_VARIABLES_SCHEMA);

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
                let new_variables: any = {};
                //If we alerady have variables add to them. 
                if (selected_node_variables) {
                    new_variables = cloneDeep(selected_node_variables)
                }

                new_variables[key] = "";
                // new_variables[key] = form_data.type === "number" ? 0 : "";

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
