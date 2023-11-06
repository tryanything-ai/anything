import {z} from 'zod'

export type UpdateFlow = {
    flowName: string;
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
