import EditPersonalAccountName from "@/components/basejump/edit-personal-account-name";
import {createClient} from "@/lib/supabase/server";

export default async function PersonalAccountSettingsPage(): Promise<JSX.Element> {
    const supabaseClient = await createClient();
    const {data: personalAccount}: any = await supabaseClient.rpc('get_personal_account');

    return (
        <div>
            <EditPersonalAccountName account={personalAccount} />
        </div>
    )
}