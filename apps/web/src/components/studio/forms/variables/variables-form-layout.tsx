import { useState } from "react";
import InputVariablesForm from "./input-variables-form";
import EditVariablesForm from "./edit-variables-form";
import { Button } from "@/components/ui/button";

export function VariablesFormLayout({ variables, variables_schema }: any) {
    const [editing, setEditing] = useState(false);

    return (
        <div className="rounded-lg border p-4 ">
            <div className="flex flex-row items-center mb-6">
                <div className="font-bold">{editing ? "Edit Variables" : "Variables"}</div>
                <div className="flex-1" />
                <Button variant={"link"} onClick={() => setEditing(!editing)}>{editing ? "Cancel" : "Edit"}</Button>
            </div>
            {
                editing
                    ?
                    <EditVariablesForm variables_schema={variables_schema} setEditing={setEditing} />
                    :
                    <InputVariablesForm variables={variables} variables_schema={variables_schema} setEditing={setEditing} />
            }
        </div>
    )
}
