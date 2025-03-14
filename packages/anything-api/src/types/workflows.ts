import { HandleProps, Edge, NodeProps } from "reactflow";

//Configuration needed to display and run a Flow
export interface Workflow {
  actions: Action[]; //Processing
  edges: Edge[]; //Needed for BFS traversal and flow rendering
}

//Guessing to what the total list of 
export enum ActionType {
  Input = "input",
  Trigger = "trigger",
  Action = "action",
  Loop = "loop",
  Decision = "decision",
  Filter = "filter",
  Output = "output"
}

//Node Configuration needed to display and run a Node
export interface Action {
  anything_action_version: string; //defines compatability so in future we can upgrade actions
  type: ActionType;
  plugin_name: string;
  plugin_version: string; //TODO: so we can manage upgrade of plugins
  action_id: string; //unique id for react flow. probably generated based on action_id or plugin_id or slug of label
  label: string;
  description?: string;
  icon: string;
  inputs: Variable;
  inputs_schema: Variable; //Action Variables. Almost like node level .env
  plugin_config: Variable;
  plugin_config_schema: Variable;
  presentation?: NodePresentation;
  handles?: HandleProps[];
}

// Presentation data only needed for react flow but we need all of it
interface NodePresentation {

  position: {
    x: number;
    y: number;
  }
}

interface Variable {
  [key: string]: any; // Using an index signature since the keys can vary.
}

export type AnythingNodeProps = NodeProps<Action>;


export type DBFlowTemplate = {
  flow_template_id: string;
  account_id: string;
  flow_template_name: string;
  flow_template_description: string | null;
  public: boolean;
  approved: boolean;
  publisher_id: string;
  anonymous_publish: boolean;
  slug: string;
  flow_template_versions: DBFlowTemplateVersion[];
  tags: any[];
  profiles: any;
  archived: boolean;
  updated_at: string;
  created_at: string;
  updated_by: string | null;
  created_by: string | null;
};


export type DBFlowTemplateVersion = {
  flow_template_version_id: string;
  account_id: string;
  flow_template_version_name: string;
  flow_definition: Workflow;
  public: boolean;
  flow_template_version: string;
  publisher_id: string;
  flow_template_id: string;
  commit_message: string | null;
  anything_flow_version: string;
  recommended_version: boolean;
  archived: boolean;
  updated_at: string;
  created_at: string;
  updated_by: string | null;
  created_by: string | null;
};

//TODO: this is kinda an ugly hack but could not get an empty object to not change to null
//Specifically this was braking @anything/webhook_response when we tried to return anything but JSON.
//Because without repairing workflwos on save it did not work so we could onlly return jSON. 
//THis signals tehre are deeper problems in state management in the workflow wich is not surprising!
//
export function fillDefaultInputs(workflow: Workflow): Workflow {
  return {
    ...workflow,
    actions: workflow.actions.map(action => ({
      ...action,
      inputs: fillDefaultsFromSchema(action.inputs, action.inputs_schema)
    }))
  };
}

function fillDefaultsFromSchema(inputs: Variable, schema: Variable): Variable {
  const filledInputs = { ...inputs };

  console.log('filledInputs', filledInputs);
  console.log('schema', schema);
  
  // Only process if schema has properties
  if (schema.properties) {
    // Iterate through schema properties and fill in defaults
    for (const [key, prop] of Object.entries(schema.properties)) {
      if (filledInputs[key] === null || filledInputs[key] === undefined) {
        filledInputs[key] = (prop as { default?: any }).default;
      }
    }
  }
  console.log('filledInputs', filledInputs);

  return filledInputs;
}

