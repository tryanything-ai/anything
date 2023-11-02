export declare class Anything {
    path: string;
    constructor(path: string);
    stop(): Promise<unknown>;
    getFlows<T>(): Promise<T>;
    createFlow<T>(flowName: string, flowId: string): Promise<T>;
}
