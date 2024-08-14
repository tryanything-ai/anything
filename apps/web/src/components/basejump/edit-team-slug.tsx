"use client";

import { Input } from "@repo/ui/components/ui/input";
import { SubmitButton } from "@/components/submit-button";
import { editTeamSlug } from "@/lib/actions/teams";
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

export default function EditTeamSlug({ account }: Props) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Team Identifier</CardTitle>
        <CardDescription>Your team identifier must be unique</CardDescription>
      </CardHeader>
      <form className="animate-in flex-1 text-foreground">
        <input type="hidden" name="accountId" value={account.account_id} />
        <CardContent className="flex flex-col gap-y-6">
          <div className="flex flex-col gap-y-2">
            <Label htmlFor="slug">Identifier</Label>
            <div className="flex items-center gap-x-2">
              <span className="text-sm text-muted-foreground whitespace-nowrap grow">
                https://your-app.com/
              </span>
              <Input
                defaultValue={account.slug}
                name="slug"
                placeholder="my-team"
                required
              />
            </div>
          </div>
        </CardContent>
        <CardFooter>
          <SubmitButton formAction={editTeamSlug} pendingText="Updating...">
            Save
          </SubmitButton>
        </CardFooter>
      </form>
    </Card>
  );
}
