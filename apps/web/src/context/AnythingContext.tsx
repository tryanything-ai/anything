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
import {
  AccountsContext,
  AccountsContextInterface,
  AccountsProvider,
} from "./AccountsContext";

interface AnythingContextInterface {
  accounts: AccountsContextInterface;
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
    <AccountsProvider>
      <AccountsContext.Consumer>
        {(accounts) => (
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
                                accounts,
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
        )}
      </AccountsContext.Consumer>
    </AccountsProvider>
  );
};

export const useAnything = () => {
  const context = useContext(AnythingContext);

  if (!context) {
    throw new Error("useAnything must be used within a AnythingProvider");
  }

  return context;
};
