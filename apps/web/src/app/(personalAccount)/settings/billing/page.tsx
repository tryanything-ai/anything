import {createClient} from "@/lib/supabase/server";
import AccountBillingStatus from "@/components/basejump/account-billing-status";

const returnUrl: string = process.env.NODE_ENV === 'production' 
  ? `https://${process.env.NEXT_PUBLIC_VERCEL_URL}` 
  : `http://${process.env.NEXT_PUBLIC_VERCEL_URL}`;

export default async function PersonalAccountBillingPage() {
    const supabaseClient = createClient();
    const {data: personalAccount}: any = await supabaseClient.rpc('get_personal_account');

    return (
        <div>
            <AccountBillingStatus accountId={personalAccount.account_id} returnUrl={`${returnUrl}/dashboard/settings/billing`} />
        </div>
    )
}