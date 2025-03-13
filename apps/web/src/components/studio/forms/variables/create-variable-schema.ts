//This is a little tricky. 

//Json schema form uses the value form this dropdown as the "type" key.
//Type keys are used for form validation.
//Since forms need to validate successfull with {{variables}} in them we need it to be text
//Even if the undelying goals is number, boolean etc.
//We need to be json compliant and form compliant so we use the inputType to determine the form field and proper server parsing of
//Numbers, booleans, etc, everything on client is text unless its object. 

export const VARIABLE_TYPES_JSF_PRESENTATION_AND_ANY_VALIDATION: any = {
    text: {
        default: "",
        type: "string",  //used for validation in JSForm
        "x-jsf-presentation": {
            inputType: "text", //Used to pick UI element on client
        },
        "x-any-validation": {
            type: "string", //Used to assist template renderer on server give correct types from variables
        }
    },
    number: {
        default: 0,    
        type: "text",
        "x-jsf-presentation": {
            inputType: "number_or_variable",
        },
        "x-any-validation": {
            type: "number",
        }
    },
    boolean: {
        default: true,
        type: "string", 
        "x-jsf-presentation": {
            inputType: "boolean_or_variable",
        },
        "x-any-validation": {
            type: "boolean",
        }
    },
    html: {
        default: "",
        type: "string",
        "x-jsf-presentation": {
            inputType: "html_or_variable",
        },
        "x-any-validation": {
            type: "string",
        }
    },
    xml: {
        default: "",
        type: "string",
        "x-jsf-presentation": {
            inputType: "xml_or_variable",
        },
        "x-any-validation": {
            type: "string",
        }
    },
    object: {
        default: {},
        type: "object",
        "x-jsf-presentation": { 
            inputType: "object_or_variable",
        },
        "x-any-validation": {
            type: "object",
        }
    },
    account: {
        type: "account",
        "x-jsf-presentation": {
            inputType: "account",
        },
        "x-any-validation": {   
            type: "object",
        }
    }
}


export let CREATE_VARIABLE_SCHEMA: any = {
    type: "object",
    properties: {
        "title": {
            "title": "Name",
            "description": "The name of the input",
            "type": "string",
            "x-jsf-presentation": {
                "inputType": "simple_text"
            },
            "x-any-validation": {
                "type": "string"
            }
        },
        "type": {
            "title": "Type",
            "description": "The type of the input",
            "type": "string",
            "oneOf": [
                {
                    "value": "text",
                    "title": "Text",
                   
                },
                {
                    "value": "html", 
                    "title": "HTML"
                },
                {
                    "value": "xml",
                    "title": "XML"
                },
                { 
                    "value": "number",
                    "title": "Number"
                },
                {
                    "value": "boolean",
                    "title": "Boolean"
                },
                {
                    "value": "object",
                    "title": "JSON"
                },
                {
                    "value": "account",
                    "title": "Account"
                }
            ],
            "x-jsf-presentation": {
                "inputType": "select_or_variable"
            },
            "x-any-validation": {
                "type": "string"
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
                    "value": "google",
                    "title": "Google"
                }
            ],
            "x-jsf-presentation": { 
                "inputType": "select_or_variable"
            },
            "x-any-validation": {
                "type": "string"
            }
        }
    },
    "x-jsf-order": ["title", "type", "provider"],
    "required": ["title", "type"],
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

export type InputProperty = {
    key?: string;
    title: string;
    description: string;
    type: string;
    oneOf?: { value: string; title: string }[];
    "x-jsf-presentation"?: { inputType: string };
};

export type SimpleVariablesSchema = {
    properties: Record<string, InputProperty>
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
