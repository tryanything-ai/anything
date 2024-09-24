"use client";

import { useState, useEffect } from "react";
import { Trash2, Edit2, Eye, EyeOff } from "lucide-react";
import api from "@repo/anything-api";
import { Button } from "@repo/ui/components/ui/button";

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
import { EditSecret, CreateNewSecret } from "@/components/secrets/secret-input";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import {
  Table,
  TableRow,
  TableBody,
  TableCell,
} from "@repo/ui/components/ui/table";
import { useAnything } from "@/context/AnythingContext";

export default function AccountsPage(): JSX.Element {
  const [secrets, setSecrets] = useState<any[]>([]);
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);
  const [secretToDelete, setSecretToDelete] = useState<any>({});
  const [showNewSecretEditor, setShowNewSecretEditor] = useState(false);
  const [visibleSecrets, setVisibleSecrets] = useState<{
    [key: string]: boolean;
  }>({});
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const cancel = () => {
    setShowNewSecretEditor(false);
  };

  const fetchSecrets = async () => {
    try {
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      const response = await api.secrets.getSecrets(selectedAccount.account_id);
      if (response.length === 0) {
        setSecrets([]);
        return;
      }
      setSecrets(response);
    } catch (error) {
      console.error("Error fetching secrets:", error);
    }
  };

  const updateSecret = async (
    secret_id: string,
    secret_name: string,
    secret_value: string,
    secret_description: string,
  ) => {
    try {
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      await api.secrets.updateSecret(
        selectedAccount.account_id,
        secret_id,
        secret_name,
        secret_value,
        secret_description,
      );
      fetchSecrets();
    } catch (error) {
      console.error("Error updating secret:", error);
    }
  };

  const deleteSecret = async (secret_id: string) => {
    try {
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      await api.secrets.deleteSecret(selectedAccount.account_id, secret_id);
      fetchSecrets();
    } catch (error) {
      console.error("Error deleting secret:", error);
    } finally {
      setSecretToDelete({});
      setShowDeleteDialog(false);
    }
  };

  const saveNewSecret = async (
    secret_name: string,
    secret_value: string,
    secret_description: string,
  ) => {
    try {
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      await api.secrets.createSecret(
        selectedAccount.account_id,
        secret_name,
        secret_value,
        secret_description,
      );
      fetchSecrets();
    } catch (error) {
      console.error("Error deleting secret:", error);
    } finally {
      setSecretToDelete({});
      setShowDeleteDialog(false);
    }
  };

  const openDialog = (secret: any) => {
    setShowDeleteDialog(true);
    setSecretToDelete(secret);
  };

  const toggleSecretVisibility = (secretId: string) => {
    setVisibleSecrets((prev) => ({
      ...prev,
      [secretId]: !prev[secretId],
    }));
  };

  useEffect(() => {
    fetchSecrets();
  }, []);

  return (
    <>
      <Card>
        <CardHeader className="flex flex-row">
          <div className="flex flex-col space-y-1.5 p-6">
            <CardTitle>Secrets</CardTitle>
            <CardDescription>Manage API Keys etc</CardDescription>
          </div>
          <div className="ml-auto py-6">
            {!showNewSecretEditor && (
              <Button onClick={() => setShowNewSecretEditor(true)}>
                Create New Secret
              </Button>
            )}
          </div>
        </CardHeader>
        <CardContent>
          {showNewSecretEditor && (
            <div className="w-full mb-6">
              <CreateNewSecret
                cancel={() => setShowNewSecretEditor(false)}
                saveSecret={(
                  name: string,
                  value: string,
                  description: string,
                ) => {
                  saveNewSecret(name, value, description);
                  setShowNewSecretEditor(false);
                }}
              />
            </div>
          )}
          <Table>
            <TableBody className="border border-gray-300">
              {secrets.map((secret: any, index) => (
                <TableRow key={secret.secret_id} className="">
                  <TableCell>{secret.secret_name}</TableCell>
                  <TableCell className="w-64">
                    <span className="flex items-center">
                      {visibleSecrets[secret.secret_id] ? (
                        <span>{secret.secret_value}</span>
                      ) : (
                        <span className="text-lg tracking-widest">
                          ••••••••
                        </span>
                      )}
                      <Button
                        variant="ghost"
                        size="sm"
                        className="ml-2"
                        onClick={() => toggleSecretVisibility(secret.secret_id)}
                      >
                        {visibleSecrets[secret.secret_id] ? (
                          <EyeOff className="h-4 w-4" />
                        ) : (
                          <Eye className="h-4 w-4" />
                        )}
                      </Button>
                    </span>
                  </TableCell>

                  <TableCell className="text-right">
                    {/* <Button
                      variant="outline"
                      size="sm"
                      className="ml-2"
                      // onClick={() => setEditing(!editing)}
                    >
                      <Edit2 className="size-5" />
                    </Button> */}
                    <Button
                      variant="outline"
                      size="sm"
                      className="ml-2"
                      onClick={() => deleteSecret(secret.secret_id)}
                    >
                      <Trash2 className="size-5" />
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
      {/* Alert */}
      <AlertDialog
        open={showDeleteDialog}
        onOpenChange={(open) => {
          setShowDeleteDialog(open);
          setSecretToDelete({});
        }}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              {`This action cannot be undone. This will permanently delete the secret "${secretToDelete.secret_name}"`}
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              className="bg-red-500"
              onClick={() => deleteSecret(secretToDelete.secret_id)}
            >
              Delete Secret
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}
