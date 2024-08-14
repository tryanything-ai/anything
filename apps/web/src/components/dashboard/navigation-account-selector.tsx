"use client";

import { useRouter } from "next/navigation";
// import AccountSelector from "@/components/basejump/account-selector";

interface Props {
  accountId: string;
}
export default function NavigatingAccountSelector({
  accountId,
}: Props): JSX.Element {
  const router = useRouter();

  return (
    <>derp</>
    // <AccountSelector
    //   accountId={accountId}
    //   onAccountSelected={(account) =>
    //     router.push(
    //       account?.personal_account
    //         ? `/dashboard`
    //         : `/dashboard/${account?.slug}`,
    //     )
    //   }
    // />
  );
}
