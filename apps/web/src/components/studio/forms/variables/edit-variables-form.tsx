import { useEffect, useState, useCallback } from "react";

import { Button } from "@/components/ui/button";
import { Edit2, Trash2 } from "lucide-react";
import { useAnything } from "@/context/AnythingContext";
import { EditVariableFormMode } from "@/context/VariablesContext";

type VariableSchema = {
    type: string;
    properties: Record<string, any>;
    "x-jsf-order": string[];
    required: string[];
    additionalProperties: boolean;
};

type OrderedVariable = {
    name: string;
    title: string;
    description: string;
    type: string;
    oneOf?: { value: string; title: string }[];
    "x-jsf-presentation"?: { inputType: string };
};

function getOrderedVariables(schema: VariableSchema): OrderedVariable[] {
    const { properties, "x-jsf-order": order } = schema;

    return order.map((key) => {
        const property = properties[key];
        return {
            name: key,
            title: property.title,
            description: property.description,
            type: property.type,
            oneOf: property.oneOf,
            "x-jsf-presentation": property["x-jsf-presentation"],
        };
    });
}

// type EditVariablesFormProps = {
//     variables_schema: VariableSchema;
//     editVariable: (variable: OrderedVariable | null) => void;
//     deleteVariable: (variable: OrderedVariable) => void
// };

export default function EditVariablesForm() {
    const { variables } = useAnything();

    const [variablesList, setVariablesList] = useState<OrderedVariable[]>([]);

    useEffect(() => {
        if (!variables.variables_schema) return;
        const varsList = getOrderedVariables(variables.variables_schema);
        setVariablesList(varsList);
    }, [variables.variables_schema]);

    const handleEdit = useCallback((variable: OrderedVariable | null
    ) => {
        console.log("Create Variable");
        // editVariable(variable);
        variables.setSelectedProperty(variable);
        variables.setEditingMode(EditVariableFormMode.EDIT)
    }, []);

    const handleDelete = useCallback((variable: OrderedVariable) => {
        console.log("Delete Variable");
        variables.setSelectedProperty(variable);
        // variables.deleteVariable(variable)
    }, []);

    const addVariable = () => {
        console.log("Add Variable");
    }

    return (
        <div className="space-y-2 mt-4">
            <Button variant="default" className="w-full" onClick={() => handleEdit(null)}>Add Variable</Button>
            {variablesList.map((variable) => (
                <div key={variable.name} className="rounded-lg border p-1 flex flex-row align-center ">
                    <h2 className="flex items-center text-xl text-left w-full ">{variable.title}</h2>
                    <div className="flex-1" />
                    <Button variant="outline" size="sm" className="ml-2" onClick={() => handleEdit(variable)}>
                        <Edit2 className="size-5" />
                    </Button>
                    <Button variant="outline" size="sm" className="ml-2" onClick={() => handleDelete(variable)}>
                        <Trash2 className="size-5" />
                    </Button>
                </div>
            ))}
        </div>
    );
}
