export let EDIT_ACTION_DISPLAY_SCHEMA: any = {
  type: "object",
  properties: {
    label: {
      title: "Label",
      description: "Short name of Action.",
      type: "string",
      "x-jsf-presentation": {
        inputType: "simple_text",
      },
      "x-any-validation": {
        type: "string",
      },
    },
    description: {
      title: "Description",
      description:
        "Longer description of what the action does for extra clarity.",
      type: "string",
      "x-jsf-presentation": {
        inputType: "simple_text",
      },
      "x-any-validation": {
        type: "string",
      },
    },
    icon: {
      title: "Icon",
      description: "SVG icon to represent the action. Often a company logo.",
      type: "string",
      "x-jsf-presentation": {
        inputType: "simple_text",
      },
      "x-any-validation": {
        type: "string",
      },
    },
  },
  "x-jsf-order": ["label", "description", "icon"],
  required: ["label", "icon"],
  additionalProperties: false,
};
