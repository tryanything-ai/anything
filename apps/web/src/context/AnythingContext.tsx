"use client";

import React, { createContext, useContext } from "react";

import {
  WorkflowVersionProvider,
  WorkflowVersionContext,
  WorkflowVersionContextInterface,
} from "./WorkflowVersionProvider";
import {
  VariablesContext,
  VariablesContextInterface,
  VariablesProvider,
} from "./VariablesContext";
import {
  WorkflowTestingContext,
  WorkflowTestingContextInterface,
  WorkflowTestingProvider,
} from "./WorkflowTestingProvider";

interface AnythingContextInterface {
  workflow: WorkflowVersionContextInterface;
  variables: VariablesContextInterface;
  testing: WorkflowTestingContextInterface;
}

const AnythingContext = createContext<AnythingContextInterface | undefined>(
  undefined,
);

export const AnythingProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  return (
    <WorkflowVersionProvider>
      <WorkflowVersionContext.Consumer>
        {(workflow) => (
          <VariablesProvider>
            <VariablesContext.Consumer>
              {(variables) => (
                <WorkflowTestingProvider>
                  <WorkflowTestingContext.Consumer>
                    {(testing) => (
                      <AnythingContext.Provider
                        value={{
                          workflow,
                          variables,
                          testing,
                        }}
                      >
                        {children}
                      </AnythingContext.Provider>
                    )}
                  </WorkflowTestingContext.Consumer>
                </WorkflowTestingProvider>
              )}
            </VariablesContext.Consumer>
          </VariablesProvider>
        )}
      </WorkflowVersionContext.Consumer>
    </WorkflowVersionProvider>
  );
};

export const useAnything = () => {
  const context = useContext(AnythingContext);

  if (!context) {
    throw new Error("useAnything must be used within a AnythingProvider");
  }

  return context;
};
