import { useState } from "react";

import { Button } from "@/components/ui/button";
import EditVariableForm from "./edit-variable-form";
import EditVariablesForm from "./edit-variables-form";

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

type EditVariablesFormProps = {
    variables_schema: VariableSchema;
    cancel: () => void;
};

export default function EditVariablesFormLayout({ variables_schema, cancel }: EditVariablesFormProps) {
    const [addingVariable, setAddingVariable] = useState(false);
    const [variable, setVariable] = useState<OrderedVariable | null>(null);

    const Header = () => {
        let header_title = "";
        let action: () => void = () => { };
        let link_button_text = "";

        if (addingVariable) {
            action = () => setAddingVariable(false);
            link_button_text = "Cancel";
            if(variable === null){
                header_title = "Create Variable";
            } else {
                header_title = "Edit Variable";
            }
           
        } else {
            action = () => cancel();
            link_button_text = "Cancel";
            header_title = "Edit Variables";
        }

        return (
            <div className="flex flex-row items-center">
                <div className="font-bold">{header_title}</div>
                <div className="flex-1" />
                <Button variant={"link"} onClick={action}>{link_button_text}</Button>
            </div>
        )
    }

    const editVariable = (variable: OrderedVariable | null) => {
        setAddingVariable(true);
        setVariable(variable);
    }

    const deleteVariable = async (variable: OrderedVariable) => {
        //TODO: Implement delete variable
        console.log("Delete Variable");
    }

    return (
        <div className="">
            <Header />
            {
                addingVariable
                    ?
                    <EditVariableForm variable={variable} />
                    :
                    <EditVariablesForm variables_schema={variables_schema} editVariable={editVariable} deleteVariable={deleteVariable} />
            }
        </div>
    );
}
