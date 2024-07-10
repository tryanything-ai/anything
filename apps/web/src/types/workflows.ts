import { HandleProps, Edge, NodeProps } from "reactflow";

//Configuration needed to display and run a Flow
export interface Workflow {
  actions: Action[]; //Processing
  edges: Edge[]; //Needed for BFS traversal and flow rendering
}

//Guessing to what the total list of 
enum PluginType {
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
  type: PluginType;
  plugin_id: string;
  node_id: string; //unique id for react flow. probably generated based on action_id or plugin_id or slug of label
  plugin_version: string; //TODO: so we can manage upgrade of plugins
  label: string;
  description?: string;
  icon: string;
  variables: Variable;
  variables_schema: Variable; //Action Variables. Almost like node level .env
  input: Variable;
  input_schema: Variable;
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
