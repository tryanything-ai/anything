export type RenameFlowArgs = {
    flow_name: string;
    active: boolean;
    version?: string;
};

export type CreateFlowVersion = {
    flowId: string;
    flowVersion: string;
    description?: string;
    flowDefinition: any;
    published: boolean;
};
