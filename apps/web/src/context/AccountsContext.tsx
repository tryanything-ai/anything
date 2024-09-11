"use client";

import {
  createContext,
  ReactNode,
  useEffect,
  useState,
  useMemo,
  useContext,
} from "react";
import { useAccounts } from "@/lib/hooks/use-accounts";
import { useRouter } from "next/navigation";

export type Account = {
  account_id: string;
  name: string;
  personal_account: boolean;
};

export interface AccountsContextInterface {
  accounts: Account[] | undefined;
  personalAccount: Account | undefined;
  teamAccounts: Account[] | undefined;
  selectedAccount: Account | undefined;
  setSelectedAccount: (account: Account) => void;
  isLoading: boolean;
}

export const AccountsContext = createContext<AccountsContextInterface>({
  isLoading: true,
  accounts: undefined,
  personalAccount: undefined,
  teamAccounts: undefined,
  selectedAccount: undefined,
  setSelectedAccount: () => {},
});

export const useAccountsContext = () => useContext(AccountsContext);

export const AccountsProvider = ({
  children,
}: {
  children: ReactNode;
}): JSX.Element => {
  const [selectedAccount, setSelectedAccountState] = useState<
    Account | undefined
  >();
  const router = useRouter();

  const { data: accounts, isLoading } = useAccounts();

  const { personalAccount, teamAccounts } = useMemo(() => {
    const personalAccount = accounts?.find(
      (account) => account.personal_account,
    );
    const teamAccounts = accounts?.filter(
      (account) => !account.personal_account,
    );

    return {
      personalAccount,
      teamAccounts,
    };
  }, [accounts]);

  useEffect(() => {
    const storedAccount = localStorage.getItem("selectedAccount");
    if (storedAccount) {
      setSelectedAccountState(JSON.parse(storedAccount));
    } else if (teamAccounts && teamAccounts.length > 0 && !selectedAccount) {
      setSelectedAccountState(teamAccounts[0]);
    }
  }, []);

  const setSelectedAccount = (account: Account) => {
    setSelectedAccountState(account);
    localStorage.setItem("selectedAccount", JSON.stringify(account));
    router.push("/");
  };

  return (
    <AccountsContext.Provider
      value={{
        isLoading,
        accounts,
        personalAccount,
        teamAccounts,
        selectedAccount,
        setSelectedAccount,
      }}
    >
      {children}
    </AccountsContext.Provider>
  );
};
