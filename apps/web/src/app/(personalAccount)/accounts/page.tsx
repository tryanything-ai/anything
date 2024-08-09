"use client";

import { useEffect, useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogTrigger,
  DialogClose,
  DialogContent,
  DialogTitle,
  DialogHeader,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import api from "@/lib/anything-api";
import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";

export default function AccountsPage() {
  const [accounts, setAccounts] = useState<any[]>([]);

  const fetchAccounts = async () => {
    try {
      let res = await api.auth.getAuthAccounts();
      console.log("accounts res:", res);
      setAccounts(res);
    } catch (error) {
      console.error("Error fetching accounts:", error);
    }
  };
  useEffect(() => {
    // Fetch accounts
    fetchAccounts();
  }, []);

  return (
    <>
      <Card>
        <CardHeader className="flex flex-row">
          <div className="flex flex-col space-y-1.5 p-6">
            <CardTitle>Accounts</CardTitle>
            <CardDescription>Connect other applications</CardDescription>
          </div>
          <div className="ml-auto py-6">
            <NewAccountDialog />
          </div>
        </CardHeader>

        <CardContent>
          <Table>
            <TableBody>
              {accounts.map((account) => (
                <TableRow
                  key={account.acount_id}
                  className="border-1 rounded-md"
                >
                  <TableCell>
                    <BaseNodeIcon icon={account.auth_provider.provider_icon} />
                  </TableCell>

                  <TableCell>
                    {account.account_auth_provider_account_label}
                  </TableCell>
                  <TableCell>{account.created_at}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </>
  );
}

const NewAccountDialog = () => {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button>Add New Account</Button>
      </DialogTrigger>
      {/* <DialogContent className="sm:max-w-[425px]"> */}
      <DialogContent className="max-w-5xl h-2/3">
        <DialogHeader>
          <DialogTitle>Edit profile</DialogTitle>
          <DialogDescription>
            Make changes to your profile here. Click save when you're done.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <div className="grid grid-cols-4 items-center gap-4">
            {/* <Label htmlFor="name" className="text-right">
              Name
            </Label>
            <Input
              id="name"
              defaultValue="Pedro Duarte"
              className="col-span-3"
            /> */}
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            {/* <Label htmlFor="username" className="text-right">
              Username
            </Label>
            <Input
              id="username"
              defaultValue="@peduarte"
              className="col-span-3"
            /> */}
          </div>
        </div>
        <DialogFooter>
          <Button type="submit">Save changes</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
