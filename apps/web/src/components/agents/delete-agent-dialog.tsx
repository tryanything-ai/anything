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
import { useRouter } from "next/navigation";
import api from "@repo/anything-api";
import { createClient } from "@/lib/supabase/client";

export default function DeleteAgentDialog({
  agentId,
}: {
  agentId: string;
}): JSX.Element {
  const navigate = useRouter();
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const handleDelete = async () => {
    try {
      console.log("Deleting Agent in DeleteAgentDialog");
      if (!selectedAccount) return;
      await api.agents.deleteAgent(
        await createClient(),
        selectedAccount.account_id,
        agentId,
      );
      navigate.push("/agents");
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <AlertDialog>
      <AlertDialogTrigger asChild>
        <Button variant="destructive">Delete Agent</Button>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
          <AlertDialogDescription>
            Deleting Agents is permanent.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction onClick={handleDelete}>
            Delete Agent
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
