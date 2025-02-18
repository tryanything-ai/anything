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
import { Trash2 } from "lucide-react";
import { InputProperty } from "./create-variable-schema";

export default function DeleteInputDialog({
  input,
}: {
  input: InputProperty;
}): JSX.Element {
  const { variables } = useAnything();

  const handleDelete = async () => {
    console.log("Delete Input: " + input.key);
    if (!input.key) return;
    await variables.deleteInput(input.key);
  };

  return (
    <AlertDialog>
      <AlertDialogTrigger asChild>
        {/* TODO: this savingStatus thing is a hack. Having deep problems preventing erros updating variables when you do it fast and we recreate json state in config to hyrate to server */}
        {/* Probbaly need to rebuild lots of state management to get around this much closer to the server. Allow endpoints for updating indivudal nodes in a flow versus just handling a large json object locally.  */}
        {/* Skipped a good fix to just get launched */}
        <Button variant="outline" size="sm" className="ml-2">
          <Trash2 className="size-5" />
        </Button>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
          <AlertDialogDescription>
            Deleting variables is permanent and can effect the entire workflow.
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction onClick={handleDelete}>
            Delete Input
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
