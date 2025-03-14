"use client";

import { useEffect, useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import {
  Table,
  TableRow,
  TableBody,
  TableCell,
} from "@repo/ui/components/ui/table";
import { Badge } from "@repo/ui/components/ui/badge";
import { createClient } from "@/lib/supabase/client";
import TeamMemberOptions from "./team-member-options";
import api from "@repo/anything-api";

type Props = {
  accountId: string;
};

export default function ManageTeamMembers({ accountId }: Props) {
  const [members, setMembers] = useState<any[]>([]);
  const [currentUser, setCurrentUser] = useState<any>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchData() {
      try {
        const supabaseClient = await createClient();

        // Get members
        const membersData = await api.accounts.getAccountMembers(
          supabaseClient,
          accountId,
        );
        setMembers(membersData || []);

        // Get current user
        const { data } = await supabaseClient.auth.getUser();
        setCurrentUser(data?.user);
      } catch (err) {
        setError("Failed to load team members");
        console.error(err);
      } finally {
        setIsLoading(false);
      }
    }

    fetchData();
  }, [accountId]);

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>{error}</div>;

  const isPrimaryOwner = members?.find(
    (member) => member.user_id === currentUser?.id,
  )?.is_primary_owner;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Team Members</CardTitle>
        <CardDescription>These are the users in your team</CardDescription>
      </CardHeader>
      <CardContent>
        <Table>
          <TableBody>
            {members?.map((member) => (
              <TableRow key={member.user_id}>
                <TableCell>
                  <div className="flex gap-x-2">
                    {member.name}
                    <Badge
                      variant={
                        member.account_role === "owner" ? "default" : "outline"
                      }
                    >
                      {member.is_primary_owner
                        ? "Primary Owner"
                        : member.account_role}
                    </Badge>
                  </div>
                </TableCell>
                <TableCell className="text-right">
                  {!Boolean(member.is_primary_owner) && (
                    <TeamMemberOptions
                      teamMember={member}
                      accountId={accountId}
                      isPrimaryOwner={isPrimaryOwner}
                    />
                  )}
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}
