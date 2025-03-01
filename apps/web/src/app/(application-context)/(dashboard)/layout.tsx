import { createClient } from "@/lib/supabase/server";
import DashboardHeader from "@/components/dashboard/dashboard-header";

export default async function PersonalAccountDashboard({
  children,
}: {
  children: React.ReactNode;
}): Promise<JSX.Element> {
  const supabaseClient = await createClient();

  const { data: personalAccount, error }: any = await supabaseClient.rpc(
    "get_personal_account",
  );

  const navigation = [
    // {
    //   name: "Dashboard",
    //   href: "/",
    // },
    {
      name: "Agents",
      href: "/agents",
    },
    {
      name: "Automations",
      href: "/workflows",
    },
    { name: "Campaigns", href: "/campaigns" },

    // {
    //   name: "Templates",
    //   href: "/templates",
    // },
    // {
    {
      name: "Calls",
      href: "/inbox",
    },
    {
      name: "Settings",
      href: "/settings",
    },
  ];

  return (
    <>
      <DashboardHeader
        accountId={personalAccount.account_id}
        navigation={navigation}
      />
      <div className="w-full p-4">{children}</div>
    </>
  );
}
