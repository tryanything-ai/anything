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
export default function DeleteFlowDialog({
  workflowId,
}: {
  workflowId: string;
}): JSX.Element {
  const navigate = useRouter();
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const handleDelete = async () => {
    try {
      console.log("Deleting Flow in DeleteFlowDialog");
      if (!selectedAccount) return;
      await api.flows.deleteFlow(
        await createClient(),
        selectedAccount.account_id,
        workflowId,
      );
      navigate.push("/workflows");
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <AlertDialog>
      <AlertDialogTrigger asChild>
        <Button className="absolute bottom-0 w-full mb-2" variant="destructive">
          Delete Workflow
        </Button>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
          <AlertDialogDescription>
            Deleting Workflows is permanent.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction onClick={handleDelete}>
            Delete Workflow
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
