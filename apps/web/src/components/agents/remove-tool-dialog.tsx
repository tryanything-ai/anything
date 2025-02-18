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
  
  export default function RemoveToolDialog({
    agentId,
    toolId,
    onRemove,
  }: {
    agentId: string;
    toolId: string;
    onRemove?: () => void;
  }): JSX.Element {
    const {
      accounts: { selectedAccount },
    } = useAnything();
  
    const handleRemove = async () => {
      try {
        console.log("Removing tool from agent");
        if (!selectedAccount) return;
        await api.agents.removeToolFromAgent(
          await createClient(),
          selectedAccount.account_id,
          agentId,
          toolId
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
            <AlertDialogTitle>Remove this tool?</AlertDialogTitle>
            <AlertDialogDescription>
              This will remove the tool from your agent. You can add it back later if needed.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={handleRemove}>
              Remove Tool
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    );
  }