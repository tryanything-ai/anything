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
                    "value": "text",
                    "title": "Text"
                },
                {
                    "value": "account",
                    "title": "Account"
                }
            ],
            "x-jsf-presentation": {
                "inputType": "select"
            }
        }, 
        "provider": {
            "title": "Authentication Provider",
            "description": "System your connecting too",
            "type": "string",
            "oneOf": [
                {
                    "value": "airtable",
                    "title": "Airtable"
                },
                {
                    "value": "gmail",
                    "title": "Gmail"
                }
            ],
            "x-jsf-presentation": {
                "inputType": "select"
            }
        }
    },
    "x-jsf-order": ["title", "description", "type", "provider"],
    "required": ["title", "description", "type"],
    "allOf": [
    {
      "if": {
        "properties": {
          "type": {
            "const": "account"
          }
        },
        "required": [
          "type"
        ]
      },
      "then": {
        "required": [
          "provider"
        ]
      },
      "else": {
        "properties": {
          "provider": false
        }
      }
    }
  ],
    "additionalProperties": false
}

export const EDIT_VARIABLES_VARIABLES = {
    "title": "",
    "description": "",
    "type": "",
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


export const isValidVariablesSchema = (obj: any): boolean => {
    const defaultKeys = Object.keys(DEFAULT_VARIABLES_SCHEMA);
    const objKeys = Object.keys(obj);

    if (defaultKeys.length !== objKeys.length) {
        return false;
    }

    for (let key of defaultKeys) {
        if (!objKeys.includes(key)) {
            return false;
        }
    }

    return true;
}
