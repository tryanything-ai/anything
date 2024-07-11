import { useEffect, useState, useCallback } from "react";

import { Button } from "@/components/ui/button";
import { Edit2 } from "lucide-react";
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

    console.log("Properties: getOrderedVariables", properties);

    return order.reduce((acc: VariableProperty[], key) => {
        if (!properties[key]) {
            console.log("No property for key: ", key);
            return acc;
        }

        const property = properties[key];
        acc.push({
            key,
            title: property.title,
            description: property.description,
            type: property.type,
            oneOf: property.oneOf,
            "x-jsf-presentation": property["x-jsf-presentation"],
        });
        return acc;
    }, []);
}

export default function EditVariablesForm() {
    const { variables, workflow } = useAnything();
    const [variablesList, setVariablesList] = useState<VariableProperty[]>([]);

    useEffect(() => {
        if (!workflow) return;
        if (!workflow.selected_node_data) return;
        if (!workflow.selected_node_data.variables_schema) return;
        if (!workflow?.selected_node_data?.variables_schema) {
            console.log("No Variables Schema. Not setting variables list.");
            return;
        }
        console.log("Setting Variables List")
        const varsList = getOrderedVariables(workflow.selected_node_data.variables_schema as VariableSchema);
        setVariablesList(varsList);
    }, [workflow.selected_node_data?.variables_schema]);

    const handleEdit = useCallback((property: any | undefined
    ) => {
        console.log("Create Variable");
        variables.setSelectedProperty(property);
        variables.setEditingMode(EditVariableFormMode.EDIT)
    }, []);

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
                </div>
            ))}
        </div>
    );
}
