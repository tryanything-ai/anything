import { useState } from "react";

import { createHeadlessForm } from "@remoteoss/json-schema-form";
import { JsonSchemaForm } from "./json-schema-form";
import { Button } from "@/components/ui/button";

export default function InputVariablesForm({ variables_schema, variables, edit }: any) {

    const { fields, handleValidation } = createHeadlessForm(variables_schema, {
        strictInputType: false, // so you don't need to pass presentation.inputType,
        initialValues: variables,
    });

    //Update Configuration
    async function handleVariableInputSubmit(jsonValues: any, { formValues }: any) {
        alert(
            `Submitted with succes! ${JSON.stringify(
                { formValues, jsonValues },
                null,
                3
            )}`
        );
        console.log("Submitted!", { formValues, jsonValues });
    }

    const Header = () => {
        let header_title = "Variables";
        let action = () => edit();
        let link_button_text = "Edit";

        return (
            <div className="flex flex-row items-center">
                <div className="font-bold">{header_title}</div>
                <div className="flex-1" />
                <Button variant={"link"} onClick={action}>{link_button_text}</Button>
            </div>
        )
    }

    return (
        <>
            <Header />
            {
                (Object.keys(variables).length > 0) &&
                <JsonSchemaForm
                    name="input-variables-form"
                    onSubmit={handleVariableInputSubmit}
                    fields={fields}
                    initialValues={variables}
                    handleValidation={handleValidation}
                />
            }
        </>
    );
}
