"use client"

import {
    createContext,
    ReactNode,
    useContext,
    useEffect,
    useState,
} from "react";

import { useWorkflowVersionContext } from "./WorkflowVersionProvider";

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
}

export const VariablesContext = createContext<VariablesContextInterface>({
    editingMode: EditVariableFormMode.INPUT,
    setEditingMode: () => { },
    variables: {},
    variables_schema: {},
    selectedProperty: null,
    setSelectedProperty: () => { }
});

export const userVariablesContext = () => useContext(VariablesContext);

export const VariablesProvider = ({ children }: { children: ReactNode }) => {

    const { selected_node_id, selected_node_data } = useWorkflowVersionContext();
    const [editingMode, setEditingMode] = useState<EditVariableFormMode>(EditVariableFormMode.INPUT);
    const [variables, setVariables] = useState<any>({});
    const [variables_schema, setVariablesSchema] = useState<any>({});
    const [selectedProperty, setSelectedProperty] = useState<any>(null);

    useEffect(() => {
        if (selected_node_data) {
            setVariables(selected_node_data.variables);
            setVariablesSchema(selected_node_data.variables_schema);
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
                setSelectedProperty
            }}
        >
            {children}
        </VariablesContext.Provider>
    );
};
