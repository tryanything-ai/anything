"use client";

import { useRouter } from "next/navigation";
import AccountSelector from "@/components/basejump/account-selector";

interface Props {
  accountId: string;
}
export default function NavigatingAccountSelector({
  accountId,
}: Props): JSX.Element {
  const router = useRouter();

  return <AccountSelector accountId={accountId} />;
}
