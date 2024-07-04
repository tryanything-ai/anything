"use client"

import { useState, useEffect } from "react";
import { Trash2 } from "lucide-react";
import api from "@/lib/anything-api";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

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
//   import { Button } from "@/components/ui/button"

//   export function AlertDialogDemo() {
//     return (

//     )
//   }


export default function AccountsPage() {
    const [secrets, setSecrets] = useState<any[]>([]);
    const [showDeleteDialog, setShowDeleteDialog] = useState(false);
    const [secretToDelete, setSecretToDelete] = useState<any>({});

    const fetchSecrets = async () => {
        try {
            const response = await api.secrets.getSecrets();
            setSecrets(response);
        } catch (error) {
            console.error('Error fetching secrets:', error);
        }
    }

    const deleteSecret = async (secret_id: string) => {
        try {
            await api.secrets.deleteSecret(secret_id);
            fetchSecrets();
        } catch (error) {
            console.error('Error deleting secret:', error);
        } finally {
            secretToDelete({});
            setShowDeleteDialog(false);
        }
    }

    const openDialog = (secret: any) => {
        setShowDeleteDialog(true);
        setSecretToDelete(secret);
    }

    useEffect(() => {
        fetchSecrets();
    }, []);

    return (
        <div className="flex flex-col gap-y-4 py-12 h-full w-full max-w-screen-md mx-auto text-center">
            <h1 className="text-2xl font-bold">Secrets</h1>

            <p>
                {secrets.map((secret: any, index) => (
                    <div key={index} className="flex m-2 items-center justify-center content-center">
                        {/* <PartyPopper className="size-5" /> */}
                        <div className="text-lg font-bold mr-2">{secret.secret_name}</div>
                        <Input type="" value={secret.vault_secret_id} readOnly />
                        <Button variant="outline" size="sm" className="ml-2" onClick={() => openDialog(secret)}>
                            <Trash2 className="size-5" />
                        </Button>
                    </div>
                ))}
            </p>
            <AlertDialog open={showDeleteDialog} onOpenChange={(open) => { setShowDeleteDialog(open); setSecretToDelete({}); }}>
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                        <AlertDialogDescription>
                            {`This action cannot be undone. This will permanently delete the secret "${secretToDelete.secret_name}"`}
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                        <AlertDialogAction className="bg-red-500" onClick={() => deleteSecret(secretToDelete.secret_id)}>Delete Secret</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>
        </div>
    )
}