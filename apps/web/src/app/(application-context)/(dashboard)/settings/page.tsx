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

import api from "@repo/anything-api";

import { BaseNodeIcon } from "@/components/studio/nodes/node-icon";
import { format } from "date-fns";
import NewAccountDialog from "@/components/secrets/new-account-dialog";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";

export default function AccountsPage(): JSX.Element {
  const [accounts, setAccounts] = useState<any[]>([]);
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const fetchAccounts = async () => {
    try {
      if (!selectedAccount) return;
      let res = await api.auth.getAuthAccounts(
        await createClient(),
        selectedAccount.account_id
      );
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
              {accounts?.map((account) => (
                <TableRow
                  key={account.acount_id}
                  className="border-1 rounded-md"
                >
                  <TableCell>
                    <BaseNodeIcon icon={account?.auth_provider?.provider_icon} />
                  </TableCell>

                  <TableCell>
                    <div className="flex items-center gap-2">
                      {account?.account_auth_provider_account_label}
                    </div>
                  </TableCell>
                  <TableCell>
                    {account?.created_at
                      ? format(new Date(account.created_at), "Pp")
                      : "N/A"}
                  </TableCell>
                  <TableCell>
                    {account?.failed && (
                      <span className="rounded-full bg-red-100 px-2 py-1 text-xs font-medium text-red-800">
                        Broken
                      </span>
                    )}
                    {/* //TODO: a fix button someday */}
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
