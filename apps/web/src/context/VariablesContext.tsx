"use client"

import {
    createContext,
    ReactNode,
    useContext,
    useEffect,
    useState,
} from "react";

import slugify from "slugify";
import { useWorkflowVersionContext } from "./WorkflowVersionProvider";
import { VariableProperty, DEFAULT_VARIABLES_SCHEMA } from "@/components/studio/forms/variables/edit-variable-schema";
export enum EditVariableFormMode {
    INPUT = "input",
    DELETE = "delete",
    EDIT = "edit"
}

export interface VariablesContextInterface {
    editingMode: EditVariableFormMode;
    setEditingMode: (mode: EditVariableFormMode) => void;
    variables: any;
    variables_schema: any;
    selectedProperty: any;
    setSelectedProperty: (property: any) => void;
    updateVariablesProperty: (data: any) => Promise<boolean>;
    deleteVariable: (variableName: string) => Promise<boolean>;
}

export const VariablesContext = createContext<VariablesContextInterface>({
    editingMode: EditVariableFormMode.INPUT,
    setEditingMode: () => { },
    variables: {},
    variables_schema: {},
    selectedProperty: null,
    setSelectedProperty: () => { },
    updateVariablesProperty: () => Promise.resolve(false),
    deleteVariable: () => Promise.resolve(false)
});

export const userVariablesContext = () => useContext(VariablesContext);

export const VariablesProvider = ({ children }: { children: ReactNode }) => {

    const { selected_node_id, selected_node_data, updateNodeData } = useWorkflowVersionContext();
    const [editingMode, setEditingMode] = useState<EditVariableFormMode>(EditVariableFormMode.INPUT);
    const [variables, setVariables] = useState<any>({});
    const [variables_schema, setVariablesSchema] = useState<any>({});
    const [selectedProperty, setSelectedProperty] = useState<VariableProperty | null>(null);

    const updateVariablesProperty = async (data: any) => {
        try {
            if (selectedProperty) {

                if (!variables_schema) return false;
                if (!selectedProperty) return false;
                if (!variables_schema.properties) return false;
                if (!selectedProperty.key) return false;

                //Merge incoming data with existing property
                variables_schema.properties[selectedProperty.key] = { ...variables_schema.properties[selectedProperty.key], ...data };

                console.log("Updating variables property -> New Variables Schema: ", variables_schema);

                //update to Anyting Context and Db
                await updateNodeData("variables_schema", variables_schema);

                //Update local state
                setVariablesSchema(variables_schema);
            } else {

                //TODO: this is a new variable create it from scratch
                console.log("Creating new variable: ", data);
                let key = slugify(data.title, {
                    replacement: '_',  // replace spaces with replacement character, defaults to `-`
                    lower: true,      // convert to lower case, defaults to `false`
                })

                //Create new property
                variables_schema.properties[key] = data;

                //Make sure we add to order and required
                variables_schema["x-jsf-order"].push(key);
                variables_schema.required.push(key);

                //update to Anyting Context and Db
                await updateNodeData("variables_schema", variables_schema);

                //Update local state
                setVariablesSchema(variables_schema);
            }

        } catch (e) {
            console.log("Error updating variables property: ", e);
            return false;
        }
        return true;
    }

    const deleteVariable = async (variableName: string) => {
        try {
            // Create a copy of the current state
            const updatedSchema = { ...variables_schema };

            // Delete the variable from properties
            delete updatedSchema.properties[variableName];

            // Remove the variable from x-jsf-order
            const orderIndex = updatedSchema["x-jsf-order"].indexOf(variableName);
            if (orderIndex > -1) {
                updatedSchema["x-jsf-order"] = [
                    ...updatedSchema["x-jsf-order"].slice(0, orderIndex),
                    ...updatedSchema["x-jsf-order"].slice(orderIndex + 1),
                ];
            }

            // Remove the variable from required
            const reqIndex = updatedSchema.required.indexOf(variableName);
            if (reqIndex > -1) {
                updatedSchema.required = [
                    ...updatedSchema.required.slice(0, reqIndex),
                    ...updatedSchema.required.slice(reqIndex + 1),
                ];
            }

            // Update the database
            await updateNodeData("variables_schema", updatedSchema);

            console.log("Deleted variable - new schema: ", updatedSchema);

            // Update the local state
            setVariablesSchema(updatedSchema);

            //Remove the key from teh variables
            let updatedVariables = { ...variables };

            delete updatedVariables[variableName];

            //Save to db
            await updateNodeData("variables", updatedVariables);

            setVariables(updatedVariables);

            return true;
        } catch (e) {
            console.log("Error deleting variable: ", e);
            return false;
        }
    };


    // const deleteVariable = async (variableName: string) => {
    //     try {


    //         delete variables_schema.properties[variableName]

    //         const index = variables_schema["x-jsf-order"].indexOf(variableName);
    //         if (index > -1) {
    //             variables_schema["x-jsf-order"].splice(index, 1);
    //         }

    //         const reqIndex = variables_schema.required.indexOf(variableName);
    //         if (reqIndex > -1) {
    //             variables_schema.required.splice(reqIndex, 1);
    //         }
    //         //update db
    //         await updateNodeData("variables_schema", variables_schema);

    //         console.log("Deleted variable - new schema: ", variables_schema);
    //         //update local state
    //         setVariablesSchema(variables_schema);

    //         return true;
    //     } catch (e) {
    //         console.log("Error deleting variable: ", e);
    //         return false;
    //     }
    // }

    useEffect(() => {
        if (selected_node_data) {
            setVariables(selected_node_data.variables);
            if (!selected_node_data.variables_schema || Object.keys(selected_node_data.variables_schema).length === 0) {
                //We are starting a fresh schema.
                //We need to create a new schema framework
                setVariablesSchema(DEFAULT_VARIABLES_SCHEMA);
            } else {
                setVariablesSchema(selected_node_data.variables_schema);
            }

            setEditingMode(EditVariableFormMode.INPUT);
        } else {
            setVariables({});
            setVariablesSchema({});
            setEditingMode(EditVariableFormMode.INPUT);
        }
    }, [selected_node_data])

    return (
        <VariablesContext.Provider
            value={{
                editingMode,
                setEditingMode,
                variables,
                variables_schema,
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
