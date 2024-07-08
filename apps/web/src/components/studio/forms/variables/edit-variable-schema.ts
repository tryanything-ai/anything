export const EDIT_VARIABLES_SCHEMA = {
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