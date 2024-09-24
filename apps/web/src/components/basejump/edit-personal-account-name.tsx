import { Input } from "@repo/ui/components/ui/input";
import { SubmitButton } from "@/components/submit-button";
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
import { editPersonalAccountName } from "@/lib/actions/personal-account";

type Props = {
  account: GetAccountResponse;
};

export default function EditPersonalAccountName({
  account,
}: Props): JSX.Element {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Your info</CardTitle>
        <CardDescription>
          Your name is used on your personal profile as well as in your teams
        </CardDescription>
      </CardHeader>
      <form className="animate-in flex-1 text-foreground">
        <input type="hidden" name="accountId" value={account.account_id} />
        <CardContent className="flex flex-col gap-y-6">
          <div className="flex flex-col gap-y-2">
            <Label htmlFor="name">Name</Label>
            <Input
              defaultValue={account.name}
              name="name"
              placeholder="Marty Mcfly"
              required
            />
          </div>
        </CardContent>
        <CardFooter>
          <SubmitButton
            formAction={editPersonalAccountName}
            pendingText="Updating..."
          >
            Save
          </SubmitButton>
        </CardFooter>
      </form>
    </Card>
  );
}
