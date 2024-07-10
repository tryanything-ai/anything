import { useCallback } from "react";

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
} from "@/components/ui/alert-dialog"
import { Button } from "@/components/ui/button"
import { useAnything } from "@/context/AnythingContext";
import { Trash2 } from "lucide-react";
import { VariableProperty } from "./edit-variable-schema";
import { EditVariableFormMode } from "@/context/VariablesContext";

export default function DeleteVariableDialog({ variable }: { variable: VariableProperty }) {

    const { variables } = useAnything();

    const handleDelete = useCallback(async () => {
        console.log("Delete Variable");
        if (!variable.key) return;
        variables.setSelectedProperty(variable);
        await variables.deleteVariable(variable.key);
    }, []);

    return (
        <AlertDialog>
            <AlertDialogTrigger asChild>
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
                    <AlertDialogAction onClick={handleDelete}>Delete Variable</AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>

    )
}