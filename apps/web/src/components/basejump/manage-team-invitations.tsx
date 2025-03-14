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
import { formatDistanceToNow } from "date-fns";
import { createClient } from "@/lib/supabase/client";
import api from "@repo/anything-api";
import CreateTeamInvitationButton from "./create-team-invitation-button";
import DeleteTeamInvitationButton from "./delete-team-invitation-button";

type Props = {
  accountId: string;
};

export default function ManageTeamInvitations({ accountId }: Props) {
  const [invitations, setInvitations] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchInvitations() {
      try {
        const supabaseClient = await createClient();
        const result = await api.accounts.getAccountInvitations(
          supabaseClient,
          accountId,
        );
        setInvitations(result || []);
      } catch (err) {
        setError("Failed to load invitations");
        console.error(err);
      } finally {
        setIsLoading(false);
      }
    }

    fetchInvitations();
  }, [accountId]);

  if (isLoading) return <div>Loading...</div>;
  if (error) return <div>{error}</div>;

  return (
    <Card>
      <CardHeader>
        <div className="flex justify-between">
          <div>
            <CardTitle>Pending Invitations</CardTitle>
            <CardDescription>
              These are the pending invitations for your team
            </CardDescription>
          </div>
          <CreateTeamInvitationButton accountId={accountId} />
        </div>
      </CardHeader>
      {Boolean(invitations?.length) && (
        <CardContent>
          <Table>
            <TableBody>
              {invitations?.map((invitation: any) => (
                <TableRow key={invitation.invitation_id}>
                  <TableCell>
                    <div className="flex gap-x-2">
                      {formatDistanceToNow(invitation.created_at, {
                        addSuffix: true,
                      })}
                      <Badge
                        variant={
                          invitation.invitation_type === "24_hour"
                            ? "default"
                            : "outline"
                        }
                      >
                        {invitation.invitation_type}
                      </Badge>
                      <Badge
                        variant={
                          invitation.account_role === "owner"
                            ? "default"
                            : "outline"
                        }
                      >
                        {invitation.account_role}
                      </Badge>
                    </div>
                  </TableCell>
                  <TableCell className="text-right">
                    {/* <DeleteTeamInvitationButton
                      invitationId={invitation.invitation_id}
                    /> */}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      )}
    </Card>
  );
}
