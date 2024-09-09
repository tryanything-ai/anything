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
import {
  SubscriptionProvider,
  SubscriptionContextInterface,
  SubscriptionContext,
} from "./SubscriptionContext";

interface AnythingContextInterface {
  accounts: AccountsContextInterface;
  subscription: SubscriptionContextInterface;
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
          <SubscriptionProvider>
            <SubscriptionContext.Consumer>
              {(subscription) => (
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
                                      subscription,
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
            </SubscriptionContext.Consumer>
          </SubscriptionProvider>
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
