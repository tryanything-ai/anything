import SettingsNavigation from "@/components/dashboard/settings-navigation";
import DashboardTitle from "@/components/dashboard/dashboard-title";
import { Separator } from "@repo/ui/components/ui/separator";

export default function AuthAndSecretsPage({
  children,
}: {
  children: React.ReactNode;
}): JSX.Element {
  const items = [
    { name: "Accounts", href: "/connections" },
    { name: "Secrets", href: "/connections/secrets" },
    { name: "API Keys", href: "/connections/keys" },
  ];

  return (
    <div className="space-y-6 w-full">
      <DashboardTitle
        title="Connections"
        description="Manage your connections to other apps"
      />
      <Separator />
      <div className="flex flex-col space-y-8 lg:flex-row lg:space-x-12 lg:space-y-0 w-full max-w-6xl mx-auto">
        <aside className="-mx-4 lg:w-1/4">
          <SettingsNavigation items={items} />
        </aside>
        <div className="flex-1">{children}</div>
      </div>
    </div>
  );
}
