import { HandleProps, Edge, NodeProps } from "reactflow";

//Configuration needed to display and run a Flow
export interface Workflow {
  actions: Action[]; //Processing
  edges: Edge[]; //Needed for BFS traversal and flow rendering
}

//Guessing to what the total list of 
enum ActionType {
  Trigger = "trigger",
  Action = "action",
  Loop = "loop",
  Decision = "decision",
  Filter = "filter"
}

//Node Configuration needed to display and run a Node
interface Action {
  anything_action_version: string; //defines compatability
  action_type: ActionType.Trigger;
  plugin_id: string;
  node_name: string; //will use as nodeID
  icon: string;
  node_label: string;
  description?: string;
  variables: Variable[]; //Local variables
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
  [key: string]: string; // Using an index signature since the keys can vary.
}

export type AnythingNodeProps = NodeProps<Action>;
