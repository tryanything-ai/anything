import { NodeProps } from "reactflow";

type NodeProcessData = {
    worker_type: string;
};
  
export type NodeData = NodeProcessData & {
    description?: string;
    [key: string]: any;
};

export type Node = {
    nodeType: string;
    image_src?: string;
    title?: string;
    alt: string;
    nodeData: NodeData;
    specialData?: any;
  };

export type AnythingNodeProps = NodeProps<NodeData>;