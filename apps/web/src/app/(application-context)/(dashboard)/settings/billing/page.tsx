"use client";

import AccountBillingStatus from "@/components/basejump/account-billing-status";

// const returnUrl: string =
//   process.env.NODE_ENV === "production"
//     ? `https://${process.env.NEXT_PUBLIC_VERCEL_URL}`
//     : `http://${process.env.NEXT_PUBLIC_VERCEL_URL}`;

export default function PersonalAccountBillingPage(): JSX.Element {
  return <AccountBillingStatus />;
}
