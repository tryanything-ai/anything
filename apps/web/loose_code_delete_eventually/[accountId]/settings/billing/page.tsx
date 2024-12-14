import AccountBillingStatus from "@/components/basejump/account-billing-status";
import { createClient } from "@/lib/supabase/server";
import { Alert } from "@repo/ui/components/ui/alert";

export default async function TeamBillingPage({
  params: { accountSlug },
}: {
  params: { accountSlug: string };
}): Promise<JSX.Element> {
  const supabaseClient = await createClient();
  const { data: teamAccount }: any = await supabaseClient.rpc(
    "get_account_by_slug",
    // @ts-ignore
    {
      slug: accountSlug,
    },
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
      // accountId={teamAccount.account_id}
      // returnUrl={`${returnUrl}/dashboard/${accountSlug}/settings/billing`}
      />
    </div>
  );
}
