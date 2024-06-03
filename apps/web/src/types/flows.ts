import { HandleProps, Edge, NodeProps } from "reactflow";

//Top level metadata about a flow
//like you might see in a package explorer like NPM
export type FlowFrontMatter = {
  active: boolean; //processign state ( all on or all off kinda like pause and start )
  name: string;
  anything_flow_version: string; //defines compatability
  flow_id: string;
  flow_version_id: string;
  version: string; //the flow version this represents Should be Semver someday
  username?: string; //for sharing online or in organization
  user_id?: string; //for sharing online or in organization
  description?: string;
  variables?: Variable[]; //Global variables
  environment?: string; //Stub for future
};

//Configuration needed to display and run a Flow
export interface Flow extends FlowFrontMatter {
  active: boolean; //only one version may be
  trigger: Trigger; //Triggering
  actions: Action[]; //Processing
  edges: Edge[]; //Needed for BFS traversal and flow rendering
}

//Node Configuration needed to display and run a Node
interface BaseNode {
  trigger: boolean;
  node_name: string; //will use as nodeID
  icon: string;
  node_label: string;
  description?: string;
  variables: Variable[]; //Local variables
  config: Variable; //TODO: make these a generic to config and mockdata must have the same shape
  //TODO: make it so we can run an indivudual node to test it
  mockData: Variable; //we need the user to be able to press "play" and imitate a real run
  //TODO: config_info: => same shape but documents what each config key does
  //TODO: config_defaults: => same shape but sets sensible defaults for each config key
  //Maybe we do this on the rust plugin side and "assemble" them in the frontend or the backend assembles them somehow
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

export interface Action extends BaseNode {
  trigger: false;
  anything_action_version: string; //defines compatability
  extension_id: string;
}

export interface Trigger extends BaseNode {
  trigger: true;
  anything_trigger_version: string; //defines compatability
  trigger_type: string;
  //Mock Data should be shaped like Config. Zod to confirm?

}

export type Node = Action | Trigger;

interface Variable {
  [key: string]: string; // Using an index signature since the keys can vary.
}

export type AnythingNodeProps = NodeProps<Node>;

//TODO: refactor this to be more like the above
//This event type will be deprecated to follow new standards soon
export type EventInput = {
  flow_id: string; //flow needs a computer friendly name that can be changed without changing processing
  flowName: string; //flow needs a user friendly name
  version: string; //flows will have versions so you can have confidence messing arround in future
  nodeId: string; //represents exact_id inside a flow
  nodeType: string; //represents front_end representation of node
  nodeLabel: string; //what the user will see in the flow
  workerType: string; //worker type === "start" or "javascript interpreter" or "rest" etc
  workerName: string; //what the user will use to reference the node in props for args. needs to be snake_case
  stage: string;
  eventStatus: string;
  sessionStatus: string;
  createdAt: string;
  data: any;
};