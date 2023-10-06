import { HandleProps, NodeProps, Edge } from "reactflow";

export type Flow = {
  flow_name: string;
  flow_id?: string;
  author_username?: string;
  author_id?: string;
  version: string;
  description: string;
  variables: Variable[]; //Global variables
  environment: string; //Stub for future
  trigger: Trigger; //Triggering
  actions: Action[]; //Processing
  edges: Edge[]; //Needed for BFS traversal and flow render
};

// General Representation of a Node
interface Node {
  trigger: boolean;
  node_name: string; //will use as nodeID
  icon: string; //VSCode icon or url to image //TODO: make svg
  node_label: string;
  description?: string;
  variables: Variable[]; //Local variables
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

export const MockNewFlows: Flow[] = [
  {
    flow_name: "Mock Flow",
    author_username: "Mock Author",
    author_id: "1",
    environment: "dev",
    flow_id: "1",
    version: "0.1",
    description: "This is a mock flow",
    variables: [],
    trigger: {
      trigger: true,
      node_name: "Mock Trigger",
      icon: `<svg width="24" height="24" xmlns="http://www.w3.org/2000/svg" fill-rule="evenodd" clip-rule="evenodd">
                <path d="M10 2l4 12h-8l4 10l-4-6h8z"/>
              </svg>`,
      node_label: "Mock Label",
      variables: [],
      config: {},
      handles: [],
      trigger_type: "Mock Trigger Type",
    },
    actions: [
      {
        trigger: false,
        node_name: "Mock Action",
        icon: `<svg width="24" height="24" xmlns="http://www.w3.org/2000/svg" fill-rule="evenodd" clip-rule="evenodd">
                   <path d="M3 3h18v6h1v12h-20v-12h1v-6zm2 0v6h14v-6h-14zm-2 8h18v10h-18v-10zm5 4h8v1h-8v-1zm0 3h8v1h-8v-1zm0 3h8v1h-8v-1z"/>
                </svg>`,
        node_label: "Mock Label",
        variables: [],
        config: {},
        handles: [],
        action_type: "Mock Action Type",
        depends_on: [],
      },
      {
        trigger: false,
        node_name: "Mock Action 2",
        icon: `<svg width="24" height="24" xmlns="http://www.w3.org/2000/svg" fill-rule="evenodd" clip-rule="evenodd">
                <path d="M3 21h18v1h-18v-1zm3-8h3v7h-3v-7zm5-10h3v17h-3v-17zm5 8h3v9h-3v-9z"/>
              </svg>`,
        node_label: "Mock Label",
        variables: [],
        config: {},
        handles: [],
        action_type: "Mock Action Type",
        depends_on: [],
      },
      {
        trigger: false,
        node_name: "Mock Action 3",
        icon: `<svg width="24" height="24" xmlns="http://www.w3.org/2000/svg" fill-rule="evenodd" clip-rule="evenodd">
        <path d="M8 6l-4 4 3 3-4 4 1 1 4-4 1-1 2 2v-4l4 4 1-1-4-4 4-4-1-1-4 4v-3l-2-2-1 1zm8 0l-4 4 1 1 4-4 1-1-2-2v3zm-4 4v-3l-1-1 1-1v3l1 1z"/>
    </svg>
    `,
        node_label: "Mock Label",
        variables: [],
        config: {},
        handles: [],
        action_type: "Mock Action Type",
        depends_on: [],
      },
    ],
    edges: [],
  },
];
