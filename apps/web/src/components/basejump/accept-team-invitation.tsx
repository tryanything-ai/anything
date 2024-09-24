import { acceptInvitation } from "@/lib/actions/invitations";
import { createClient } from "@/lib/supabase/server";
import { Alert } from "@repo/ui/components/ui/alert";
import { Card, CardContent } from "@repo/ui/components/ui/card";
import { SubmitButton } from "@/components/submit-button";

type Props = {
  token: string;
};
export default async function AcceptTeamInvitation({
  token,
}: Props): Promise<JSX.Element> {
  const supabaseClient = createClient();
  const { data: invitation }: any = await supabaseClient.rpc(
    "lookup_invitation",
     // @ts-ignore
    {
      lookup_invitation_token: token,
    } as any,
  );

  return (
    <Card>
      <CardContent className="p-8 text-center flex flex-col gap-y-8">
        <div>
          <p>You've been invited to join</p>
          <h1 className="text-xl font-bold">{invitation.account_name}</h1>
        </div>
        {Boolean(invitation.active) ? (
          <form>
            <input type="hidden" name="token" value={token} />
            <SubmitButton
              formAction={acceptInvitation}
              pendingText="Accepting invitation..."
            >
              Accept invitation
            </SubmitButton>
          </form>
        ) : (
          <Alert variant="destructive">
            This invitation has been deactivated. Please contact the account
            owner for a new invitation.
          </Alert>
        )}
      </CardContent>
    </Card>
  );
}
