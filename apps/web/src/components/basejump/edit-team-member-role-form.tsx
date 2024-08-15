"use client";

import { SubmitButton } from "@/components/submit-button";
import { Label } from "@repo/ui/components/ui/label";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@repo/ui/components/ui/select";
import { updateTeamMemberRole } from "@/lib/actions/members";
import { GetAccountMembersResponse } from "@usebasejump/shared";
import { useState } from "react";
import { Checkbox } from "@repo/ui/components/ui/checkbox";
import { usePathname } from "next/navigation";

type Props = {
  accountId: string;
  isPrimaryOwner: boolean;
  teamMember: GetAccountMembersResponse[0];
};

const memberOptions = [
  { label: "Owner", value: "owner" },
  { label: "Member", value: "member" },
];

export default function EditTeamMemberRoleForm({
  accountId,
  teamMember,
  isPrimaryOwner,
}: Props): JSX.Element {
  const [teamRole, setTeamRole] = useState(teamMember.account_role as string);
  const pathName = usePathname();

  return (
    <form className="animate-in flex-1 flex flex-col w-full justify-center gap-y-6 text-foreground">
      <input type="hidden" name="accountId" value={accountId} />
      <input type="hidden" name="userId" value={teamMember.user_id} />
      <input type="hidden" name="returnUrl" value={pathName} />
      <div className="flex flex-col gap-y-2">
        <Label htmlFor="accountRole">Team Role</Label>
        <Select value={teamRole} onValueChange={setTeamRole} name="accountRole">
          <SelectTrigger>
            <SelectValue placeholder="Member type" />
          </SelectTrigger>
          <SelectContent>
            {memberOptions.map((option) => (
              <SelectItem key={option.value} value={option.value}>
                {option.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
      {teamRole === "owner" && isPrimaryOwner && (
        <div className="flex items-center space-x-2">
          <Checkbox id="makePrimaryOwner" name="makePrimaryOwner" />
          <label
            htmlFor="makePrimaryOwner"
            className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
          >
            Make this user the primary owner
          </label>
        </div>
      )}
      <SubmitButton formAction={updateTeamMemberRole} pendingText="Updating...">
        Update Role
      </SubmitButton>
    </form>
  );
}
