export let EDIT_FLOW_SCHEMA: any = {
    type: "object",
    properties: {
        "flow_name": {
            "title": "Name",
            "description": "Short name of Workflow.",
            "type": "string",
            "x-jsf-presentation": {
                "inputType": "simple_text"
            },
            "x-any-validation": {
                "type": "string"
            }
        },
        "description": {
            "title": "Description",
            "description": "Longer description of what the flow does.",
            "type": "string",
            "x-jsf-presentation": {
                "inputType": "simple_text"
            },
            "x-any-validation": {
                "type": "string"
            }
        }
    },
    "x-jsf-order": ["flow_name", "description"],
    "required": ["flow_name", "description"],
    "additionalProperties": false
}