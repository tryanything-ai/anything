"use client"

import React, { createContext, useContext } from 'react';
import { WorkflowsContext, WorkflowsProvider, WorkflowsContextInterface } from "./WorkflowsProvider";
import { WorkflowVersionProvider, WorkflowVersionContext, WorkflowVersionContextInterface } from './WorkflowVersionProvider'

interface AnythingContextInterface {
    workflows: WorkflowsContextInterface;
    workflow: WorkflowVersionContextInterface;
}

const AnythingContext = createContext<AnythingContextInterface | undefined>(undefined);

export const AnythingProvider = ({ children }: { children: React.ReactNode }) => {
    return (
        <WorkflowsProvider>
            <WorkflowsContext.Consumer>
                {workflows => (
                    <WorkflowVersionProvider>
                        <WorkflowVersionContext.Consumer>
                            {workflow => (
                                <AnythingContext.Provider value={{ workflow, workflows }}>
                                    {children}
                                </AnythingContext.Provider>
                            )}
                        </WorkflowVersionContext.Consumer>
                    </WorkflowVersionProvider>
                )}
            </WorkflowsContext.Consumer>
        </WorkflowsProvider>
    )
};

export const useAnything = () => {
    const context = useContext(AnythingContext);

    if (!context) {
        throw new Error('useAnything must be used within a AnythingProvider');
    }

    return context;
};