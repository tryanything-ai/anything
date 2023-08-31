import { NodeProps } from "reactflow";

type NodeProcessData = {
    worker_type: string;
};

type NodePresentationData = {
    image_src?: string;
    title?: string;
    alt: string;
    description?: string;
}

//ARGS TO BE PASSED TO THE NODE Processor but edited by user
type NodeConfigurationData = {
    [key: string]: any; 
}
  
export type NodeData = NodeProcessData & NodePresentationData & NodeConfigurationData;

export type Node = {
    nodeType: string;
    nodeProcessData: NodeProcessData;
    nodePresentationData: NodePresentationData;
    nodeConfigurationData: NodeConfigurationData;
  };

export type AnythingNodeProps = NodeProps<NodeData>;