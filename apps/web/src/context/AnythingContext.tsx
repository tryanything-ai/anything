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
import {
  WorkflowVersionControlContextInterface,
  WorkflowVersionControlContext,
  WorkflowVersionControlProvider,
} from "./WorkflowVersionControlContext";
import { Workflow } from "lucide-react";

interface AnythingContextInterface {
  accounts: AccountsContextInterface;
  subscription: SubscriptionContextInterface;
  workflow: WorkflowVersionContextInterface;
  version_control: WorkflowVersionControlContextInterface;
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
        {(accounts) => {
          if (!accounts.selectedAccount) {
            return null; // Or any loading indicator
            // TODO: not great but works
          }
          return (
            <SubscriptionProvider>
              <SubscriptionContext.Consumer>
                {(subscription) => (
                  <WorkflowVersionControlProvider>
                    <WorkflowVersionControlContext.Consumer>
                      {(version_control) => (
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
                                              version_control,
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
                    </WorkflowVersionControlContext.Consumer>
                  </WorkflowVersionControlProvider>
                )}
              </SubscriptionContext.Consumer>
            </SubscriptionProvider>
          );
        }}
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
