"use client";

import { createContext, ReactNode, useState } from "react";

export interface AccountsContextInterface {
  current_account_id: string;
  setCurrentAccoundId: (id: string) => void;
}

export const AccountsContext = createContext<AccountsContextInterface>({
  current_account_id: "",
  setCurrentAccoundId: () => {},
});

export const AccountsProvider = ({
  children,
}: {
  children: ReactNode;
}): JSX.Element => {
  const [current_account_id, setCurrentAccoundId] = useState<string>("");

  const resetState = () => {
    setCurrentAccoundId("");
  };

  return (
    <AccountsContext.Provider
      value={{
        current_account_id,
        setCurrentAccoundId,
      }}
    >
      {children}
    </AccountsContext.Provider>
  );
};
