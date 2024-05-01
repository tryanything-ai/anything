export declare type UpdateFlowArgs = {
    flow_name: string;
    active: boolean;
    version?: string;
};
export declare type CreateFlowVersion = {
    flowId: string;
    flowVersion: string;
    description?: string;
    flowDefinition: any;
    published: boolean;
};
export declare class Anything {
    path: string;
    constructor(path: string);
    stop(): Promise<unknown>;
    getFlows<T>(): Promise<T>;
    getFlowByName<T>(flowName: string): Promise<T>;
    createFlow<T>(flowName: string): Promise<T>;
    CreateFlowVersion<T>(flowName: string, createFlowVersion: CreateFlowVersion): Promise<T>;
    updateFlow<T>(flowId: string, args: UpdateFlowArgs): Promise<T>;
    deleteFlow<T>(flowId: string): Promise<T>;
    updateFlowVersion<T>(flowId: string, flowVersionId: string, updateFlow: any): Promise<T>;
    executeFlow<T>(flowId: string, flowVersionId: string, sessionId?: string, stage?: string): Promise<T>;
    fetchSessionEvents<T>(sessionId: string): Promise<T>;
    getEvent<T>(eventId: string): Promise<T>;
    getActions<T>(): Promise<T>;
    saveAction<T>(action: any, actionName: String): Promise<T>;
}
