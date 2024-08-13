import { useState, useEffect } from "react";
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogTitle,
  DialogHeader,
  DialogDescription,
} from "@repo/ui/dialog";
import { Button } from "@repo/ui/button";
import api from "@/lib/anything-api";
import { Table, TableBody, TableCell, TableRow } from "@repo/ui/table";
import { BaseNodeIcon } from "../studio/nodes/node-icon";

const NewAccountDialog = () => {
  const [providers, setProviders] = useState<any[]>([]);

  const fetchAccounts = async () => {
    try {
      let res = await api.auth.getProviders();
      console.log("providres res:", res);
      setProviders(res);
    } catch (error) {
      console.error("Error fetching accounts:", error);
    }
  };

  const addConnection = async (provider: any) => {
    try {
      console.log("Adding connection for provider:", provider);
      //Create Link
      //Create CSFR State
      //Open new window to serv ice
      //Redirect back to ?
    } catch (error) {
      console.error("Error adding connection:", error);
    }
  };

  useEffect(() => {
    // Fetch accounts
    fetchAccounts();
  }, []);

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button>Add New Account</Button>
      </DialogTrigger>
      {/* <DialogContent className="sm:max-w-[425px]"> */}
      <DialogContent className="max-w-3xl h-2/3">
        <DialogHeader>
          <DialogTitle>Available Apps</DialogTitle>
          <DialogDescription>Find the app to integrate here.</DialogDescription>
        </DialogHeader>
        <Table>
          <TableBody className="border border-gray-300">
            {providers.map((provider) => (
              <TableRow key={provider.acount_id}>
                <TableCell>
                  <BaseNodeIcon icon={provider.provider_icon} />
                </TableCell>

                <TableCell>{provider.provider_label}</TableCell>
                <TableCell className="text-right">
                  <Button onClick={() => addConnection(provider)}>Add</Button>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </DialogContent>
    </Dialog>
  );
};

export default NewAccountDialog;
