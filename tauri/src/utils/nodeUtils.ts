import { HandleProps, Edge, NodeProps } from "reactflow";
export type { HandleProps, Edge, NodeProps }; 
//Things configured mandatory by node author
export type NodeProcessData = {
  worker_type: string;
  trigger: boolean;
  worker_name: string;
};

//Things that deal with UI that we don't want in version control
export type NodePresentationData = {
  icon: string; //VSCode icon or url to image
  node_label: string;
  alt: string;
  description?: string;
  handles: HandleProps[];
  nodeType?: string;
  // component?: ReactNode;
};

//ARGS TO BE PASSED TO THE NODE Processor but edited by user
export type NodeConfigurationData = {
  [key: string]: any;
};

export type NodeData = NodeProcessData &
  NodePresentationData &
  NodeConfigurationData;

export type Node = {
  nodeProcessData: NodeProcessData;
  nodePresentationData: NodePresentationData;
  nodeConfigurationData: NodeConfigurationData;
};

export type AnythingNodeProps = NodeProps<NodeData>;