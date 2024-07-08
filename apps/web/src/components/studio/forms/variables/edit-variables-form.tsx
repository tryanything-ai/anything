import { useEffect, useState, useCallback } from "react";

import { Button } from "@/components/ui/button";
import { Edit2, Trash2 } from "lucide-react";

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

type EditVariablesFormProps = {
    variables_schema: VariableSchema;
    setEditing: (editing: boolean) => void;
};

export default function EditVariablesForm({ variables_schema, setEditing }: EditVariablesFormProps) {
    const [variablesList, setVariablesList] = useState<OrderedVariable[]>([]);

    useEffect(() => {
        if (!variables_schema) return;
        const varsList = getOrderedVariables(variables_schema);
        setVariablesList(varsList);
    }, [variables_schema]);

    const handleEdit = useCallback(() => {
        console.log("Create Variable");
    }, []);

    const handleDelete = useCallback(() => {
        console.log("Delete Variable");
    }, []);

    return (
        <div className="space-y-2">
            <Button variant="default" className="w-full">Add Variable</Button>
            {variablesList.map((variable) => (
                <div key={variable.name} className="rounded-lg border p-1 flex flex-row align-center ">
                    <h2 className="flex items-center text-xl text-left w-full ">{variable.title}</h2>
                    <div className="flex-1" />
                    <Button variant="outline" size="sm" className="ml-2" onClick={handleDelete}>
                        <Edit2 className="size-5" />
                    </Button>
                    <Button variant="outline" size="sm" className="ml-2" onClick={handleEdit}>
                        <Trash2 className="size-5" />
                    </Button>
                    {/* <Button onClick={handleEdit}>Edit</Button>
                    <Button onClick={handleDelete}>Delete</Button> */}
                </div>
            ))}
            <Button onClick={() => setEditing(false)} variant={"secondary"}>Cancel</Button>
        </div>
    );
}
