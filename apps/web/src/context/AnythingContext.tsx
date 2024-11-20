"use client";

import React, { createContext, useContext, useMemo } from "react";

import {
  WorkflowVersionProvider,
  useWorkflowVersion,
  WorkflowVersionContextInterface,
} from "./WorkflowVersionProvider";
import {
  VariablesContextInterface,
  VariablesProvider,
  useVariables,
} from "./VariablesContext";
import {
  WorkflowTestingContextInterface,
  WorkflowTestingProvider,
  useWorkflowTesting,
} from "./WorkflowTestingProvider";
import {
  AccountsContextInterface,
  AccountsProvider,
  useAccounts,
} from "./AccountsContext";
import {
  SubscriptionProvider,
  SubscriptionContextInterface,
  useSubscription,
} from "./SubscriptionContext";
import {
  WorkflowVersionControlContextInterface,
  WorkflowVersionControlProvider,
  useWorkflowVersionControl,
} from "./WorkflowVersionControlContext";
import {
  useVariablesExplorer,
  VariablesExplorerInterface,
  VariablesExplorerProvider,
} from "./VariablesExplorerContext";

interface AnythingContextInterface {
  accounts: AccountsContextInterface;
  subscription: SubscriptionContextInterface;
  workflow: WorkflowVersionContextInterface;
  version_control: WorkflowVersionControlContextInterface;
  variables: VariablesContextInterface;
  testing: WorkflowTestingContextInterface;
  explorer: VariablesExplorerInterface;
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
      <AnythingProviderInner>{children}</AnythingProviderInner>
    </AccountsProvider>
  );
};

const AnythingProviderInner = ({ children }: { children: React.ReactNode }) => {
  const accounts = useAccounts();

  if (!accounts.selectedAccount) {
    return null; // Or any loading indicator
  }

  return (
    <SubscriptionProvider>
      <WorkflowVersionControlProvider>
        <WorkflowVersionProvider>
          <VariablesProvider>
            <WorkflowTestingProvider>
              <VariablesExplorerProvider>
                <AnythingContextProvider>{children}</AnythingContextProvider>
              </VariablesExplorerProvider>
            </WorkflowTestingProvider>
          </VariablesProvider>
        </WorkflowVersionProvider>
      </WorkflowVersionControlProvider>
    </SubscriptionProvider>
  );
};

const AnythingContextProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const accounts = useAccounts();
  const subscription = useSubscription();
  const version_control = useWorkflowVersionControl();
  const workflow = useWorkflowVersion();
  const variables = useVariables();
  const testing = useWorkflowTesting();
  const explorer = useVariablesExplorer();

  const value = useMemo(
    () => ({
      accounts,
      subscription,
      version_control,
      workflow,
      variables,
      testing,
      explorer,
    }),
    [
      accounts,
      subscription,
      version_control,
      workflow,
      variables,
      testing,
      explorer,
    ],
  );

  return (
    <AnythingContext.Provider value={value}>
      {children}
    </AnythingContext.Provider>
  );
};

export const useAnything = () => {
  const context = useContext(AnythingContext);

  if (!context) {
    throw new Error("useAnything must be used within a AnythingProvider");
  }

  return context;
};
