export type Rust_Flow = {
  id: string;
  name: string;
  version: string;
  description: string;
  trigger: any;
  variables: any;
  dag: any;
  root: any;
};


export type EventInput = {
  flow_id: string; //flow needs a computer friendly name that can be changed without changing processing
  flow_name: string; //flow needs a user friendly name
  flow_version: string; //flows will have versions so you can have confidence messing arround in future
  node_id: string; //represents exact_id inside a flow
  node_type: string; //represents front_end representation of node
  node_label: string; //what the user will see in the flow
  worker_type: string; //worker type === "start" or "javascript interpreter" or "rest" etc
  worker_name: string; //what the user will use to reference the node in props for args. needs to be snake_case
  stage: string;
  event_status: string;
  session_status: string;
  created_at: string;
  data: any;
};