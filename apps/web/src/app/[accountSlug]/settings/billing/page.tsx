import {createClient} from "@/lib/supabase/server";
import AccountBillingStatus from "@/components/basejump/account-billing-status";
import { Alert } from "@/components/ui/alert";

const returnUrl = process.env.NEXT_PUBLIC_URL as string;

export default async function TeamBillingPage({params: {accountSlug}}: {params: {accountSlug: string}}) {
    const supabaseClient = createClient();
    const {data: teamAccount}: any = await supabaseClient.rpc('get_account_by_slug', {
        slug: accountSlug
    } as any);

    if (teamAccount.account_role !== 'owner') {
        return (
            <Alert variant="destructive">You do not have permission to access this page</Alert>
        )
    }


    return (
        <div>
            <AccountBillingStatus accountId={teamAccount.account_id} returnUrl={`${returnUrl}/dashboard/${accountSlug}/settings/billing`} />
        </div>
    )
}