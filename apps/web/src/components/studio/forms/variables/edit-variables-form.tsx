import { useEffect, useState, useCallback } from "react";

import { Button } from "@/components/ui/button";
import { Edit2, Trash2 } from "lucide-react";
import { useAnything } from "@/context/AnythingContext";
import { EditVariableFormMode } from "@/context/VariablesContext";
import { VariableProperty } from "./edit-variable-schema";
import DeleteVariableDialog from "./delete-variable-dialog";

type VariableSchema = {
    type: string;
    properties: Record<string, any>;
    "x-jsf-order": string[];
    required: string[];
    additionalProperties: boolean;
};


function getOrderedVariables(schema: VariableSchema): VariableProperty[] {
    const { properties, "x-jsf-order": order } = schema;

    return order.map((key) => {
        const property = properties[key];
        return {
            key,
            title: property.title,
            description: property.description,
            type: property.type,
            oneOf: property.oneOf,
            "x-jsf-presentation": property["x-jsf-presentation"],
        };
    });
}


export default function EditVariablesForm() {
    const { variables } = useAnything();

    const [variablesList, setVariablesList] = useState<VariableProperty[]>([]);

    useEffect(() => {
        if (!variables.variables_schema) return;
        const varsList = getOrderedVariables(variables.variables_schema);
        setVariablesList(varsList);
    }, [variables.variables_schema]);

    const handleEdit = useCallback((variable: VariableProperty | null
    ) => {
        console.log("Create Variable");
        variables.setSelectedProperty(variable);
        variables.setEditingMode(EditVariableFormMode.EDIT)
    }, []);

    // const handleDelete = useCallback((variable: VariableProperty) => {
    //     console.log("Delete Variable");
    //     if (!variable.key) return;
    //     variables.setSelectedProperty(variable);
    //     variables.deleteVariable(variable.key);
    // }, []);



    return (
        <div className="space-y-2 mt-4">
            <Button variant="default" className="w-full" onClick={() => handleEdit(null)}>Add Variable</Button>
            {variablesList.map((variable) => (
                <div key={variable.key} className="rounded-lg border p-1 flex flex-row align-center ">
                    <h2 className="flex items-center text-xl text-left w-full ">{variable.title}</h2>
                    <div className="flex-1" />
                    <Button variant="outline" size="sm" className="ml-2" onClick={() => handleEdit(variable)}>
                        <Edit2 className="size-5" />
                    </Button>
                    <DeleteVariableDialog variable={variable} />
                    {/* <Button variant="outline" size="sm" className="ml-2" onClick={() => handleDelete(variable)}>
                        <Trash2 className="size-5" />
                    </Button> */}
                </div>
            ))}
        </div>
    );
}
