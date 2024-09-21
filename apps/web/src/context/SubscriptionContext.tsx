"use client";

import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import { useAccounts } from "./AccountsContext";
import api from "@/lib/anything-api";

export interface SubscriptionContextInterface {
  stripe_customer_id: string | null;
  stripe_data: any | null;
  free_trial_days: number;
  free_trial_task_limit: number;
  free_trial_started_at: string | null;
  free_trial_ends_at: string | null;
  free_trial_task_usage: number;
  trial_ended: boolean;
  total_task_usage: number;
  total_execution_time_ms: number;
  paying_customer: boolean;
  customer_status: string;
  keep_processing_workflows: boolean;
}

export const SubscriptionContext = createContext<SubscriptionContextInterface>({
  stripe_customer_id: null,
  stripe_data: null,
  free_trial_days: 7,
  free_trial_task_limit: 1000,
  free_trial_started_at: null,
  free_trial_ends_at: null,
  free_trial_task_usage: 0,
  trial_ended: false,
  total_task_usage: 0,
  total_execution_time_ms: 0,
  paying_customer: false,
  customer_status: "",
  keep_processing_workflows: true,
});

export const useSubscription = () => useContext(SubscriptionContext);

export const SubscriptionProvider = ({
  children,
}: {
  children: ReactNode;
}): JSX.Element => {
  const { selectedAccount } = useAccounts();
  const [subscriptionData, setSubscriptionData] =
    useState<SubscriptionContextInterface>({
      stripe_customer_id: null,
      stripe_data: null,
      free_trial_days: 7,
      free_trial_task_limit: 1000,
      free_trial_started_at: null,
      free_trial_ends_at: null,
      free_trial_task_usage: 0,
      trial_ended: false,
      total_task_usage: 0,
      total_execution_time_ms: 0,
      paying_customer: false,
      customer_status: "",
      keep_processing_workflows: true,
    });

  const resetState = () => {
    setSubscriptionData({
      stripe_customer_id: null,
      stripe_data: null,
      free_trial_days: 7,
      free_trial_task_limit: 1000,
      free_trial_started_at: null,
      free_trial_ends_at: null,
      free_trial_task_usage: 0,
      trial_ended: false,
      total_task_usage: 0,
      total_execution_time_ms: 0,
      paying_customer: false,
      customer_status: "",
      keep_processing_workflows: true,
    });
  };

  const fetchStatus = async () => {
    if (!selectedAccount) {
      resetState();
      return;
    }
    try {
      const account_data = await api.billing.getAccountStatus(
        selectedAccount.account_id,
      );
      console.log("Account data:", account_data);
      setSubscriptionData(account_data);
    } catch (error) {
      console.error("Error fetching subscription status", error);
      resetState();
    }
  };

  useEffect(() => {
    fetchStatus();
  }, [selectedAccount]);

  return (
    <SubscriptionContext.Provider value={subscriptionData}>
      {children}
    </SubscriptionContext.Provider>
  );
};
