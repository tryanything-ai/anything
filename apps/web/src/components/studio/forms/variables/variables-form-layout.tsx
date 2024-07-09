import { useState } from "react";

import { Button } from "@/components/ui/button";
import EditVariablesFormLayout from "./edit-variables-form-layout";
import InputVariablesForm from "./input-variables-form";

export function VariablesFormLayout({ variables, variables_schema }: any) {
    const [editing, setEditing] = useState(false);

    // const EditButton = () => {
    //     let action: () => void = () => { };
    //     let text = "";

    //     if (editing) {
    //         action = () => setEditing(false);
    //         text = "Cancel";
    //     } else {
    //         if (Object.keys(variables).length === 0) {
    //             action = () => setEditing(true);
    //             text = "Add Variables";
    //         } else {
    //             action = () => setEditing(true);
    //             text = "Edit";
    //         }
    //     }

    //     return (
    //         <Button variant={"link"} onClick={action}>{text}</Button>
    //     )
    // }

    return (
        <div className="rounded-lg border p-4">
            {/* <div className="flex flex-row items-center">
                <div className="font-bold">{editing ? "Edit Variables" : "Variables"}</div>
                <div className="flex-1" />
                <EditButton />
            </div> */}
            {
                editing
                    ?
                    <EditVariablesFormLayout variables_schema={variables_schema} cancel={() => setEditing(false)} />
                    :
                    <InputVariablesForm variables={variables} variables_schema={variables_schema} edit={() => setEditing(true)} />
            }
        </div>
    )
}
