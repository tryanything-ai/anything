"use client";

import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";
import ManageTeamMembers from "@/components/basejump/manage-team-members";
import ManageTeamInvitations from "@/components/basejump/manage-team-invitations";
import { Alert } from "@repo/ui/components/ui/alert";
import api from "@repo/anything-api";
import { useEffect, useState } from "react";

export default function TeamMembersPage() {
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const [teamAccount, setTeamAccount] = useState<any>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchTeamAccount() {
      if (!selectedAccount?.slug) {
        setIsLoading(false);
        return;
      }

      try {
        const supabaseClient = await createClient();
        const result = await api.accounts.getAccountBySlug(
          supabaseClient,
          selectedAccount.account_id,
          selectedAccount.slug,
        );
        setTeamAccount(result);
        // console.log("In component: ", result);
      } catch (err) {
        setError("Failed to load team account");
        console.error(err);
      } finally {
        setIsLoading(false);
      }
    }

    fetchTeamAccount();
  }, [selectedAccount?.slug]);

  if (!selectedAccount) {
    return (
      <Alert variant="destructive">You are not a member of any team</Alert>
    );
  }

  if (isLoading) {
    return <div>Loading...</div>; // Consider using a proper loading spinner component
  }

  if (error) {
    return <Alert variant="destructive">{error}</Alert>;
  }

  if (teamAccount?.account_role !== "owner") {
    return (
      <Alert variant="destructive">
        You do not have permission to access this page
      </Alert>
    );
  }

  return (
    <div className="flex flex-col gap-y-8">
      <ManageTeamInvitations accountId={teamAccount.account_id} />
      <ManageTeamMembers accountId={teamAccount.account_id} />
    </div>
  );
}
