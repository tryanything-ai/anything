"use client";
import { useState } from "react";
import { SubmitButton } from "@/components/submit-button";
import { Label } from "@repo/ui/components/ui/label";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectGroup,
  SelectLabel,
  SelectItem,
} from "@repo/ui/components/ui/select";
import { createInvitation } from "@/lib/actions/invitations";
import fullInvitationUrl from "@/lib/full-invitation-url";

type Props = {
  accountId: string;
};

const invitationOptions = [
  { label: "24 Hour", value: "24_hour" },
  { label: "One time use", value: "one_time" },
];

const memberOptions = [
  { label: "Owner", value: "owner" },
  { label: "Member", value: "member" },
];

const initialState = {
  message: "",
  token: "",
};

export default function NewInvitationForm({ accountId }: Props): JSX.Element {
  const [state, setState] = useState(initialState);
  const [isPending, setIsPending] = useState(false);

  const handleFormAction = async (formData: FormData) => {
    setIsPending(true);
    try {
      const result: any = await createInvitation(state, formData);
      setState(result);
    } catch (error: any) {
      setState({ message: error.message, token: "" });
    } finally {
      setIsPending(false);
    }
  };

  return (
    <form className="animate-in flex-1 flex flex-col w-full justify-center gap-y-6 text-foreground">
      {Boolean(state?.token) ? (
        <div className="text-sm">{fullInvitationUrl(state.token!)}</div>
      ) : (
        <>
          <input type="hidden" name="accountId" value={accountId} />
          <div className="flex flex-col gap-y-2">
            <Label htmlFor="invitationType">Invitation Type</Label>
            <Select defaultValue="one_time" name="invitationType">
              <SelectTrigger>
                <SelectValue placeholder="Invitation type" />
              </SelectTrigger>
              <SelectContent>
                {invitationOptions.map((option) => (
                  <SelectItem key={option.value} value={option.value}>
                    {option.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="flex flex-col gap-y-2">
            <Label htmlFor="accountRole">Team Role</Label>
            <Select defaultValue="member" name="accountRole">
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
          <SubmitButton
            formAction={async (prevState: any, formData: FormData) =>
              handleFormAction(formData)
            }
            errorMessage={state?.message}
            pendingText="Creating..."
            aria-disabled={isPending}
          >
            {isPending ? "Creating..." : "Create invitation"}
          </SubmitButton>
        </>
      )}
    </form>
  );
}
