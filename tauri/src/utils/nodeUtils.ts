import { ReactNode } from "react";
import { HandleProps, NodeProps } from "reactflow";

//Things configured mandatory by node author
export type NodeProcessData = {
    worker_type: string;
};

//Things that deal with UI that we don't want in version control
export type NodePresentationData = {
    icon: string; //VSCode icon or url to image
    title?: string;
    alt: string;
    description?: string;
    handles: HandleProps[];
    component?: ReactNode; 
}

//ARGS TO BE PASSED TO THE NODE Processor but edited by user
export type NodeConfigurationData = {
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