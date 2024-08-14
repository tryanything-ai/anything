import { createClient } from "@/lib/supabase/server";
import AccountBillingStatus from "@/components/basejump/account-billing-status";
import { Alert } from "@repo/ui/components/ui/alert";

const returnUrl: string =
  process.env.NODE_ENV === "production"
    ? `https://${process.env.NEXT_PUBLIC_VERCEL_URL}`
    : `http://${process.env.NEXT_PUBLIC_VERCEL_URL}`;

export default async function TeamBillingPage({
  params: { accountSlug },
}: {
  params: { accountSlug: string };
}) {
  const supabaseClient = createClient();
  const { data: teamAccount }: any = await supabaseClient.rpc(
    "get_account_by_slug",
    {
      slug: accountSlug,
    } as any,
  );

  if (teamAccount.account_role !== "owner") {
    return (
      <Alert variant="destructive">
        You do not have permission to access this page
      </Alert>
    );
  }

  return (
    <div>
      <AccountBillingStatus
        accountId={teamAccount.account_id}
        returnUrl={`${returnUrl}/dashboard/${accountSlug}/settings/billing`}
      />
    </div>
  );
}
