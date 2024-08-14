"use client";

import { useEffect, useState } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableRow,
} from "@repo/ui/components/ui/table";

import api from "@/lib/anything-api";
import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";
import { format } from "date-fns";
import NewAccountDialog from "@/components/secrets/new-account-dialog";

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
            <TableBody className="border border-gray-300">
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
                  <TableCell>
                    {account.created_at
                      ? format(new Date(account.created_at), "Pp")
                      : "N/A"}
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </>
  );
}
