import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger,
  } from "@repo/ui/components/ui/alert-dialog";
  import { Button } from "@repo/ui/components/ui/button";
  import { useAnything } from "@/context/AnythingContext";
  import api from "@repo/anything-api";
  import { createClient } from "@/lib/supabase/client";
  
  export default function RemovePhoneNumberDialog({
    agentId,
    phoneNumberId,
    onRemove,
  }: {
    agentId: string;
    phoneNumberId: string;
    onRemove?: () => void;
  }): JSX.Element {
    const {
      accounts: { selectedAccount },
    } = useAnything();
  
    const handleRemove = async () => {
      try {
        console.log("Removing phone number from agent");
        if (!selectedAccount) return;
        await api.agents.removePhoneNumberFromAgent(
          await createClient(),
          selectedAccount.account_id,
          agentId,
          phoneNumberId,
        );
        onRemove?.();
      } catch (error) {
        console.error(error);
      }
    };
  
    return (
      <AlertDialog>
        <AlertDialogTrigger asChild>
          <Button variant="destructive">Remove</Button>
        </AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Remove this phone number?</AlertDialogTitle>
            <AlertDialogDescription>
              This will disconnect the phone number from your agent. You can reconnect it later
              if needed.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={handleRemove}>
              Remove Phone Number
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    );
  }