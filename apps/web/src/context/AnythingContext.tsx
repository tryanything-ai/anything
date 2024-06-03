"use client"

import React, { createContext, useContext } from 'react';
import { FlowsContextInterface, FlowsProvider, useFlowsContext } from "./flowsprovider";

interface AnythingContextInterface {
    flows: FlowsContextInterface;
}

const AnythingContext = createContext<AnythingContextInterface | undefined>(undefined);

export const AnythingProvider = ({ children }: { children: React.ReactNode }) => {
    const flows = useFlowsContext();

    return (
        <AnythingContext.Provider value={{ flows }}>
            <FlowsProvider>
                {children}
            </FlowsProvider>
        </AnythingContext.Provider>
    );
};

export const useAnything = () => {
    const context = useContext(AnythingContext);

    if (!context) {
        throw new Error('useAnything must be used within a CombinedProvider');
    }

    return context;
};