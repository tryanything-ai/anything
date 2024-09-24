import { Input } from "@repo/ui/components/ui/input";
import { SubmitButton } from "@/components/submit-button";
import { editTeamName } from "@/lib/actions/teams";
import { Label } from "@repo/ui/components/ui/label";
import { GetAccountResponse } from "@usebasejump/shared";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";

type Props = {
  account: GetAccountResponse;
};

export default function EditTeamName({ account }: Props): JSX.Element {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Team Info</CardTitle>
        <CardDescription>
          Your team name and identifier are unique for your team
        </CardDescription>
      </CardHeader>
      <form className="animate-in flex-1 text-foreground">
        <input type="hidden" name="accountId" value={account.account_id} />
        <CardContent className="flex flex-col gap-y-6">
          <div className="flex flex-col gap-y-2">
            <Label htmlFor="name">Team Name</Label>
            <Input
              defaultValue={account.name}
              name="name"
              placeholder="My Team"
              required
            />
          </div>
        </CardContent>
        <CardFooter>
          <SubmitButton formAction={editTeamName} pendingText="Updating...">
            Save
          </SubmitButton>
        </CardFooter>
      </form>
    </Card>
  );
}
