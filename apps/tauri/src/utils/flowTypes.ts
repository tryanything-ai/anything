import { HandleProps, Edge, NodeProps } from "reactflow";
import { v4 as uuidv4 } from "uuid";

//Top level metadata about a flow
//like you might see in a package explorer like NPM
export type FlowFrontMatter = {
  active: boolean; //processign state ( all on or all off kinda like pause and start )
  name: string;
  flow_id: string;
  flow_version_id: string;
  version: string; //the flow version this represents Should be Semver someday
  username?: string; //for sharing online or in organization
  user_id?: string; //for sharing online or in organization
  description?: string;
  variables?: Variable[]; //Global variables
  environment: string; //Stub for future
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
  config: Variable;
  presentation?: NodePresentation;
  handles?: HandleProps[];
}

// Presentation data only needed for react flow but we need all of it
interface NodePresentation {
  position: {
    x: number;
    y: number;
  };
}

export interface Action extends BaseNode {
  trigger: false;
  engine: string;
  depends_on: string[]; //node_name for parallelization
}

export interface Trigger extends BaseNode {
  trigger: true;
  trigger_type: string;
  mockData: any; //we need the user to be able to press "play" and imitate a real trigger
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

// Mocks for testing etc
export const MockNewFlows: Flow[] = [
  {
    active: true,
    name: "Mock Flow",
    username: "Mock Author",
    user_id: "1",
    environment: "dev",
    flow_id: uuidv4(),
    flow_version_id: uuidv4(),
    version: "0.0.1",
    description:
      "This is a mock flow with approximately 3 lines of text that needs to be concatted for the user. Actually its closer to two lines",
    variables: [],
    trigger: {
      trigger: true,
      node_name: "Mock Trigger",
      icon: `<svg width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><path d="M4.38 5h1V4h1V3h-1V2h-1v1h-1v1h1v1zm8 4h-1v1h-1v1h1v1h1v-1h1v-1h-1V9zM14 2V1h-1v1h-1v1h1v1h1V3h1V2h-1zm-2.947 2.442a1.49 1.49 0 0 0-2.12 0l-7.49 7.49a1.49 1.49 0 0 0 0 2.12c.59.59 1.54.59 2.12 0l7.49-7.49c.58-.58.58-1.53 0-2.12zm-8.2 8.9c-.2.2-.51.2-.71 0-.2-.2-.2-.51 0-.71l6.46-6.46.71.71-6.46 6.46zm7.49-7.49l-.32.32-.71-.71.32-.32c.2-.2.51-.2.71 0 .19.2.19.52 0 .71z"/></svg>`,
      node_label: "Mock Label",
      variables: [],
      config: {},
      handles: [],
      trigger_type: "Mock Trigger Type",
      mockData: {},
    },
    actions: [
      {
        trigger: false,
        node_name: "Mock Action",
        icon: `<svg width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><path d="M4.38 5h1V4h1V3h-1V2h-1v1h-1v1h1v1zm8 4h-1v1h-1v1h1v1h1v-1h1v-1h-1V9zM14 2V1h-1v1h-1v1h1v1h1V3h1V2h-1zm-2.947 2.442a1.49 1.49 0 0 0-2.12 0l-7.49 7.49a1.49 1.49 0 0 0 0 2.12c.59.59 1.54.59 2.12 0l7.49-7.49c.58-.58.58-1.53 0-2.12zm-8.2 8.9c-.2.2-.51.2-.71 0-.2-.2-.2-.51 0-.71l6.46-6.46.71.71-6.46 6.46zm7.49-7.49l-.32.32-.71-.71.32-.32c.2-.2.51-.2.71 0 .19.2.19.52 0 .71z"/></svg>`,
        node_label: "Mock Label",
        variables: [],
        config: {},
        handles: [],
        engine: "Mock Action Type",
        depends_on: [],
      },
      {
        trigger: false,
        node_name: "Mock Action 2",
        icon: `<svg width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><path d="M4.38 5h1V4h1V3h-1V2h-1v1h-1v1h1v1zm8 4h-1v1h-1v1h1v1h1v-1h1v-1h-1V9zM14 2V1h-1v1h-1v1h1v1h1V3h1V2h-1zm-2.947 2.442a1.49 1.49 0 0 0-2.12 0l-7.49 7.49a1.49 1.49 0 0 0 0 2.12c.59.59 1.54.59 2.12 0l7.49-7.49c.58-.58.58-1.53 0-2.12zm-8.2 8.9c-.2.2-.51.2-.71 0-.2-.2-.2-.51 0-.71l6.46-6.46.71.71-6.46 6.46zm7.49-7.49l-.32.32-.71-.71.32-.32c.2-.2.51-.2.71 0 .19.2.19.52 0 .71z"/></svg>`,
        node_label: "Mock Label",
        variables: [],
        config: {},
        handles: [],
        engine: "Mock Action Type",
        depends_on: [],
      },
      {
        trigger: false,
        node_name: "Mock Action 3",
        icon: `<svg width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><path d="M4.38 5h1V4h1V3h-1V2h-1v1h-1v1h1v1zm8 4h-1v1h-1v1h1v1h1v-1h1v-1h-1V9zM14 2V1h-1v1h-1v1h1v1h1V3h1V2h-1zm-2.947 2.442a1.49 1.49 0 0 0-2.12 0l-7.49 7.49a1.49 1.49 0 0 0 0 2.12c.59.59 1.54.59 2.12 0l7.49-7.49c.58-.58.58-1.53 0-2.12zm-8.2 8.9c-.2.2-.51.2-.71 0-.2-.2-.2-.51 0-.71l6.46-6.46.71.71-6.46 6.46zm7.49-7.49l-.32.32-.71-.71.32-.32c.2-.2.51-.2.71 0 .19.2.19.52 0 .71z"/></svg>`,
        node_label: "Mock Label",
        variables: [],
        config: {},
        handles: [],
        engine: "Mock Action Type",
        depends_on: [],
      },
    ],
    edges: [],
  },
  {
    active: true,
    name: "Mock Flow",
    username: "Mock Author",
    user_id: "1",
    environment: "dev",
    flow_id: uuidv4(),
    flow_version_id: uuidv4(),
    version: "0.0.1",
    description:
      "This is a mock flow with approximately 3 lines of text that needs to be concatted for the user. Actually its closer to two lines",
    variables: [],
    trigger: {
      trigger: true,
      node_name: "Mock Trigger",
      icon: `<svg width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><path d="M4.38 5h1V4h1V3h-1V2h-1v1h-1v1h1v1zm8 4h-1v1h-1v1h1v1h1v-1h1v-1h-1V9zM14 2V1h-1v1h-1v1h1v1h1V3h1V2h-1zm-2.947 2.442a1.49 1.49 0 0 0-2.12 0l-7.49 7.49a1.49 1.49 0 0 0 0 2.12c.59.59 1.54.59 2.12 0l7.49-7.49c.58-.58.58-1.53 0-2.12zm-8.2 8.9c-.2.2-.51.2-.71 0-.2-.2-.2-.51 0-.71l6.46-6.46.71.71-6.46 6.46zm7.49-7.49l-.32.32-.71-.71.32-.32c.2-.2.51-.2.71 0 .19.2.19.52 0 .71z"/></svg>`, // existing icon here
      node_label: "Mock Label",
      variables: [],
      config: {},
      handles: [],
      trigger_type: "Mock Trigger Type",
      mockData: {},
    },
    actions: [
      {
        trigger: false,
        node_name: "Mock Action",
        icon: `<svg width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><path d="M4.38 5h1V4h1V3h-1V2h-1v1h-1v1h1v1zm8 4h-1v1h-1v1h1v1h1v-1h1v-1h-1V9zM14 2V1h-1v1h-1v1h1v1h1V3h1V2h-1zm-2.947 2.442a1.49 1.49 0 0 0-2.12 0l-7.49 7.49a1.49 1.49 0 0 0 0 2.12c.59.59 1.54.59 2.12 0l7.49-7.49c.58-.58.58-1.53 0-2.12zm-8.2 8.9c-.2.2-.51.2-.71 0-.2-.2-.2-.51 0-.71l6.46-6.46.71.71-6.46 6.46zm7.49-7.49l-.32.32-.71-.71.32-.32c.2-.2.51-.2.71 0 .19.2.19.52 0 .71z"/></svg>`,
        node_label: "Mock Label",
        variables: [],
        config: {},
        handles: [],
        engine: "Mock Action Type",
        depends_on: [],
      },
      {
        trigger: false,
        node_name: "Mock Action 2",
        icon: `<svg width="16" height="16" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><path d="M4.38 5h1V4h1V3h-1V2h-1v1h-1v1h1v1zm8 4h-1v1h-1v1h1v1h1v-1h1v-1h-1V9zM14 2V1h-1v1h-1v1h1v1h1V3h1V2h-1zm-2.947 2.442a1.49 1.49 0 0 0-2.12 0l-7.49 7.49a1.49 1.49 0 0 0 0 2.12c.59.59 1.54.59 2.12 0l7.49-7.49c.58-.58.58-1.53 0-2.12zm-8.2 8.9c-.2.2-.51.2-.71 0-.2-.2-.2-.51 0-.71l6.46-6.46.71.71-6.46 6.46zm7.49-7.49l-.32.32-.71-.71.32-.32c.2-.2.51-.2.71 0 .19.2.19.52 0 .71z"/></svg>`,
        node_label: "Mock Label",
        variables: [],
        config: {},
        handles: [],
        engine: "Mock Action Type",
        depends_on: [],
      },
      {
        trigger: false,
        node_name: "Mock Action 4",
        icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"></path></svg>`, // a cross icon
        node_label: "Mock Label",
        variables: [],
        config: {},
        handles: [],
        engine: "Mock Action Type",
        depends_on: [],
      },
      {
        trigger: false,
        node_name: "Mock Action 5",
        icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M4 9h16v2H4z"></path></svg>`, // a minus icon
        node_label: "Mock Label",
        variables: [],
        config: {},
        handles: [],
        engine: "Mock Action Type",
        depends_on: [],
      },
      {
        trigger: false,
        node_name: "Mock Action 6?",
        icon: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z"></path></svg>`, // a cross icon
        node_label: "Mock Label",
        variables: [],
        config: {},
        handles: [],
        engine: "Mock Action Type",
        depends_on: [],
      },
    ],
    edges: [],
  },
];
