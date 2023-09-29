import { HandleProps, NodeProps } from "reactflow";

export type Flow = {
  flow_name: string;
  flow_id?: string;
  author_username?: string;
  author_id?: string;
  version: string;
  description: string;
  variables: Variable[];
  trigger: Trigger;
  actions: Action[];
  environment: string;
};

// General Representation of a Node
interface Node {
  trigger: boolean;
  node_name: string; //will use as nodeID
  icon: string; //VSCode icon or url to image //TODO: make svg
  node_label: string;
  description?: string;
  variables: Variable[];
  config: Variable;
  presentation?: NodePresentation;
  handles: HandleProps[];
}

interface NodePresentation {
  position: {
    x: number;
    y: number;
  };
  width: number;
  height: number;
  selected: boolean;
  dragging: boolean;
  positionAbsolute: {
    x: number;
    y: number;
  };
}

interface Action extends Node {
  trigger: false;
  action_type: string;
  depends_on: string[]; //node_name for parallelization
}

interface Trigger extends Node {
  trigger: true; 
  trigger_type: string;
}

interface Variable {
  [key: string]: string; // Using an index signature since the keys can vary.
}

export type AnythingNodeProps = NodeProps<Action | Trigger>;
