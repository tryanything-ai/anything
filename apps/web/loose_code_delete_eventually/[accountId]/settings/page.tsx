import EditTeamName from "@/components/basejump/edit-team-name";
import EditTeamSlug from "@/components/basejump/edit-team-slug";
import { createClient } from "@/lib/supabase/server";

export default async function TeamSettingsPage({
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
    } as any,
  );

  return (
    <div className="flex flex-col gap-y-8">
      <EditTeamName account={teamAccount} />
      <EditTeamSlug account={teamAccount} />
    </div>
  );
}
