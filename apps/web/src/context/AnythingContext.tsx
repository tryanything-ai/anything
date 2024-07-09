"use client"

import React, { createContext, useContext } from 'react';
import { WorkflowsContext, WorkflowsProvider, WorkflowsContextInterface } from "./WorkflowsProvider";
import { WorkflowVersionProvider, WorkflowVersionContext, WorkflowVersionContextInterface } from './WorkflowVersionProvider'
import { VariablesContext, VariablesContextInterface, VariablesProvider } from './VariablesContext';

interface AnythingContextInterface {
    workflows: WorkflowsContextInterface;
    workflow: WorkflowVersionContextInterface;
    variables: VariablesContextInterface;
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
                                <VariablesProvider>
                                    <VariablesContext.Consumer>
                                        {variables => (
                                            <AnythingContext.Provider value={{ workflow, workflows, variables }}>
                                                {children}
                                            </AnythingContext.Provider>
                                        )}
                                    </VariablesContext.Consumer>
                                </VariablesProvider>)}
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