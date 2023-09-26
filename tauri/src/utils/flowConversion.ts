export interface RustFlow {
  flow_name: string;
  author?: string;
  author_id?: string;
  flow_id?: string;
  version: string;
  description: string;
  variables: any[]; // Empty array provided in example, so using any[] for now.
  trigger: RustTrigger;
  nodes: FlowNode[];
  environment: Environment;
}

export interface RustTrigger {
  name: string;
  settings: TriggerSettings;
}

interface TriggerSettings {
  [key: string]: any;
}

interface FlowNode {
  name: string;
  label: string;
  depends_on: string[];
  variables: Variable[];
  action: Action;
}

interface Variable {
  [key: string]: string; // Using an index signature since the keys can vary.
}

interface Action {
  action_type: string;
  config: ActionConfig;
}

interface ActionConfig {
  command: string;
  executor?: string; // Optional because it's not present in the first node.
  args?: string[]; // Optional because it's not present in the first node.
}

interface Environment {
  NODE_ENV: string;
}

const convertRustFlowToTsFlow = (flow: RustFlow) => {};
