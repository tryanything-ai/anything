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
import { useRouter } from "next/navigation"

export default function DeleteFlowDialog({ workflowId }: { workflowId: string }) {
    const navigate = useRouter();
    const { workflows } = useAnything();

    const handleDelete = async () => {
        try {
            console.log("Deleting Flow in DeleteFlowDialog");
            await workflows.deleteWorkflow(workflowId);
            navigate.back();
        } catch (error) {
            console.error(error);
        }
    }


    return (
        <AlertDialog>
            <AlertDialogTrigger asChild>
                <Button className="absolute bottom-0 w-full mb-2" variant="destructive">Delete Workflow</Button>
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
                    <AlertDialogAction onClick={handleDelete}>Delete Workflow</AlertDialogAction>
                </AlertDialogFooter>
            </AlertDialogContent>
        </AlertDialog>

    )
}