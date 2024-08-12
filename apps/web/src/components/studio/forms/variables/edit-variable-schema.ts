export let EDIT_VARIABLES_SCHEMA: any = {
    type: "object",
    properties: {
        "title": {
            "title": "Name",
            "description": "The name of the variable",
            "type": "string"
        },
        "description": {
            "title": "Description",
            "description": "A description of the variable",
            "type": "string",
        },
        "type": {
            "title": "Type",
            "description": "The type of the variable",
            "type": "string",
            "oneOf": [
                {
                    "value": "string",
                    "title": "Text"
                },
                {
                    "value": "number",
                    "title": "Number"
                }
            ],
            "x-jsf-presentation": {
                "inputType": "select"
            }
        }
    },
    "x-jsf-order": ["title", "description", "type"],
    "required": ["title", "description", "type"],
    "additionalProperties": false
}

export const EDIT_VARIABLES_VARIABLES = {
    "title": "",
    "description": "",
    "type": ""
}

export type VariableProperty = {
    key?: string;
    title: string;
    description: string;
    type: string;
    oneOf?: { value: string; title: string }[];
    "x-jsf-presentation"?: { inputType: string };
};

export type SimpleVariablesSchema = {
    properties: Record<string, VariableProperty>
}

export const DEFAULT_VARIABLES_SCHEMA: any = {
    type: "object",
    properties: {},
    "x-jsf-order": [],
    required: [],
    additionalProperties: false
}
