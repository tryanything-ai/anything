import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import { createClient } from "@/lib/supabase/server";
import {
  Table,
  TableRow,
  TableBody,
  TableCell,
} from "@repo/ui/components/ui/table";
import { Badge } from "@repo/ui/components/ui/badge";
import CreateTeamInvitationButton from "./create-team-invitation-button";
import { formatDistanceToNow } from "date-fns";
import DeleteTeamInvitationButton from "./delete-team-invitation-button";

type Props = {
  accountId: string;
};

export default async function ManageTeamInvitations({
  accountId,
}: Props): Promise<JSX.Element> {
  const supabaseClient = await createClient();

  const { data: invitations }: any = await supabaseClient.rpc(
    "get_account_invitations",
    // @ts-ignore
    {
      account_id: accountId,
    } as any,
  );

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
                    <DeleteTeamInvitationButton
                      invitationId={invitation.invitation_id}
                    />
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
