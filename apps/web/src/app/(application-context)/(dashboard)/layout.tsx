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
    {
      name: "Dashboard",
      href: "/",
    },
    {
      name: "Agents",
      href: "/agents",
    },
    {
      name: "Workflows",
      href: "/workflows",
    },
    // {
    //   name: "Templates",
    //   href: "/templates",
    // },
    {
      name: "Connections",
      href: "/connections",
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
