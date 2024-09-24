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
  hydrated: boolean;
}

export const AccountsContext = createContext<AccountsContextInterface>({
  isLoading: true,
  hydrated: false,
  accounts: undefined,
  personalAccount: undefined,
  teamAccounts: undefined,
  selectedAccount: undefined,
  setSelectedAccount: () => {},
});

const useAccountsContext = () => useContext(AccountsContext);

export { useAccountsContext as useAccounts };

export const AccountsProvider = ({
  children,
}: {
  children: ReactNode;
}): JSX.Element => {
  console.log("[ACCOUNT CONTEXT] Initializing AccountsProvider");
  const [selectedAccount, setSelectedAccountState] = useState<
    Account | undefined
  >();
  const router = useRouter();

  const [hydrated, setHydrated] = useState(false);
  const { data: accounts, isLoading } = useAccounts();

  console.log("[ACCOUNT CONTEXT] Accounts data:", accounts);
  console.log("[ACCOUNT CONTEXT] Is loading:", isLoading);

  const { personalAccount, teamAccounts } = useMemo(() => {
    console.log("[ACCOUNT CONTEXT] Calculating personal and team accounts");
    const personalAccount = accounts?.find(
      (account) => account.personal_account,
    );
    const teamAccounts = accounts?.filter(
      (account) => !account.personal_account,
    );

    console.log("[ACCOUNT CONTEXT] Personal account:", personalAccount);
    console.log("[ACCOUNT CONTEXT] Team accounts:", teamAccounts);

    return {
      personalAccount,
      teamAccounts,
    };
  }, [accounts]);

  useEffect(() => {
    console.log("[ACCOUNT CONTEXT] Running account selection effect");
    if (!selectedAccount && teamAccounts && teamAccounts.length > 0) {
      console.log("[ACCOUNT CONTEXT] No account selected, updating selection");
      console.log("[ACCOUNT CONTEXT] Setting default team account");
      setSelectedAccountState(teamAccounts[0]);
    }
  }, [teamAccounts, selectedAccount]);

  useEffect(() => {
    console.log("[ACCOUNT CONTEXT] Running hydration effect");
    const storedAccount = localStorage.getItem("selectedAccount");
    if (storedAccount) {
      console.log("[ACCOUNT CONTEXT] Found stored account:", storedAccount);
      setSelectedAccountState(JSON.parse(storedAccount));
    } else if (teamAccounts && teamAccounts.length > 0 && !selectedAccount) {
      console.log("[ACCOUNT CONTEXT] Setting default team account");
      setSelectedAccountState(teamAccounts[0]);
    }
    setHydrated(true);
    console.log("[ACCOUNT CONTEXT] Hydration complete");
  }, []);

  const setSelectedAccount = (account: Account) => {
    console.log("[ACCOUNT CONTEXT] Setting selected account:", account);
    setSelectedAccountState(account);
    localStorage.setItem("selectedAccount", JSON.stringify(account));
    console.log("[ACCOUNT CONTEXT] Navigating to home page");
    router.push("/");
  };

  console.log("[ACCOUNT CONTEXT] Rendering AccountsContext.Provider");
  return (
    <AccountsContext.Provider
      value={{
        isLoading,
        hydrated,
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
