"use client"

import { useState, useEffect } from "react";
import { Trash2 } from "lucide-react";
import api from "@/lib/anything-api";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import DashboardTitleWithAction from "@/components/workflows/dashboard-title-with-action";
import { Separator } from "@/components/ui/separator";

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
import { EditSecret, CreateNewSecret } from "@/components/secrets/secret-input";


export default function AccountsPage() {
    const [secrets, setSecrets] = useState<any[]>([]);
    const [showDeleteDialog, setShowDeleteDialog] = useState(false);
    const [secretToDelete, setSecretToDelete] = useState<any>({});
    const [showNewSecretEditor, setShowNewSecretEditor] = useState(false);

    const cancel = () => {
        setShowNewSecretEditor(false);
    }

    const fetchSecrets = async () => {
        try {
            const response = await api.secrets.getSecrets();
            if (response.length === 0) {
                setSecrets([]);
                return;
            }
            setSecrets(response);
        } catch (error) {
            console.error('Error fetching secrets:', error);
        }
    }

    const updateSecret = async (secret_id: string, secret_name: string, secret_value: string, secret_description: string) => {
        try {
            await api.secrets.updateSecret(secret_id, secret_name, secret_value, secret_description);
            fetchSecrets();
        } catch (error) {
            console.error('Error updating secret:', error);
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

    const saveNewSecret = async (secret_name: string, secret_value: string, secret_description: string) => {
        try {
            await api.secrets.createSecret(secret_name, secret_value, secret_description);
            fetchSecrets();
        } catch (error) {
            console.error('Error deleting secret:', error);
        } finally {
            setSecretToDelete({});
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
        // <div className="flex flex-col gap-y-4 py-12 h-full w-full max-w-screen-md mx-auto text-center">
        <div className="space-y-6 w-full">
            <DashboardTitleWithAction title="Secrets" description="Manage secrets." action={() => setShowNewSecretEditor(true)} />
            <Separator />
            {/* <h1 className="text-2xl font-bold">Secrets</h1> */}
            {showNewSecretEditor && (
                <div className="w-full ">
                    <CreateNewSecret cancel={cancel} saveSecret={saveNewSecret} />
                </div>)
            }
            <div className="w-full ">
                {secrets.map((secret: any, index) => (
                    <EditSecret key={index} secret={secret} deleteSecret={openDialog} updateSecret={updateSecret} />
                ))}
            </div>
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