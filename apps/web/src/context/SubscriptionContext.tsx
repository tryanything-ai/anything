"use client";

import { createContext, ReactNode, useState } from "react";

export interface SubscriptionContextInterface {
  free_trial: boolean;
  free_trial_end_date?: string;
  paying_customer: boolean;
}

export const SubscriptionContext = createContext<SubscriptionContextInterface>({
  free_trial: false,
  free_trial_end_date: "",
  paying_customer: false,
});

export const SubscriptionProvider = ({
  children,
}: {
  children: ReactNode;
}): JSX.Element => {
  const [free_trial, setFreeTrial] = useState<boolean>(false);
  const [free_trial_end_date, setFreeTrialEndDate] = useState<string>("");
  const [paying_customer, setPayingCustomer] = useState<boolean>(false);

  const resetState = () => {
    setFreeTrial(false);
    setFreeTrialEndDate("");
    setPayingCustomer(false);
  };

  return (
    <SubscriptionContext.Provider
      value={{
        free_trial,
        free_trial_end_date,
        paying_customer,
      }}
    >
      {children}
    </SubscriptionContext.Provider>
  );
};
