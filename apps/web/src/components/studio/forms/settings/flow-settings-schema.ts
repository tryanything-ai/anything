export let EDIT_FLOW_SCHEMA: any = {
    type: "object",
    properties: {
        "flow_name": {
            "title": "Name",
            "description": "Short name of Workflow.",
            "type": "string"
        },
        "description": {
            "title": "Description",
            "description": "Longer description of what the flow does.",
            "type": "string",
        },
        "active": {
            "title": "Flow Active",
            "description": "Turns the flow on or off.",
            "type": "boolean",
            "x-jsf-presentation": {
                "inputType": "checkbox"
            }
        }
    },
    "x-jsf-order": ["flow_name", "description", "active"],
    "required": ["flow_name", "description"],
    "additionalProperties": false
}