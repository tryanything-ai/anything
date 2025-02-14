"use client";

import { useState, useEffect } from "react";
import {
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogTitle,
  DialogHeader,
  DialogDescription,
} from "@repo/ui/components/ui/dialog";
import { Button } from "@repo/ui/components/ui/button";
import api from "@repo/anything-api";
import {
  Table,
  TableBody,
  TableCell,
  TableRow,
} from "@repo/ui/components/ui/table";
import { BaseNodeIcon } from "../studio/nodes/node-icon";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";
// Helper function to generate a random string
const generateRandomString = (length: number) => {
  const charset =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
  let result = "";
  for (let i = 0; i < length; i++) {
    result += charset.charAt(Math.floor(Math.random() * charset.length));
  }
  return result;
};

const NewAccountDialog = (): JSX.Element => {
  const [providers, setProviders] = useState<any[]>([]);
  const [codeVerifier, setCodeVerifier] = useState(generateRandomString(43));
  const [codeChallenge, setCodeChallenge] = useState("");
  const {
    accounts: { selectedAccount },
  } = useAnything();

  useEffect(() => {
    // Fetch accounts
    fetchAccounts();
    // Generate code challenge when component mounts
    (async () => {
      const challenge = await generateCodeChallenge(codeVerifier);
      setCodeChallenge(challenge);
    })();
  }, []);

  const fetchAccounts = async () => {
    try {
      if (!selectedAccount) return;
      const res = await api.auth.getProviders(
        await createClient(),
        selectedAccount.account_id,
      );
      console.log("providers res:", res);
      setProviders(res);
    } catch (error) {
      console.error("Error fetching accounts:", error);
    }
  };

  // Helper function to generate a secure code challenge
  const generateCodeChallenge = async (codeVerifier: string) => {
    const encoder = new TextEncoder();
    const data = encoder.encode(codeVerifier);
    const hash = await crypto.subtle.digest("SHA-256", data);
    return btoa(String.fromCharCode(...new Uint8Array(hash)))
      .replace(/\+/g, "-")
      .replace(/\//g, "_")
      .replace(/=+$/, "");
  };

  const addConnection = async (provider: any) => {
    try {
      if (!selectedAccount) return;
      let { url } = await api.auth.initiateProviderAuth(
        await createClient(),
        selectedAccount?.account_id,
        provider.provider_name,
      );

      console.log("url", url);
      // Open the auth URL in a popup window
      const authWindow = window.open(url, "_blank", "width=600,height=600");

      window.addEventListener(
        "message",
        function (event) {
          if (event.data === "auth_success") {
            // Authentication was successful
            console.log("Authentication successful!");
            // You might want to refresh the page or update the UI here
            location.reload();
          }
        },
        false,
      );
    } catch (error) {
      console.error("Error adding connection:", error);
    }
  };

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button>Add New Account</Button>
      </DialogTrigger>
      <DialogContent className="max-w-3xl h-2/3">
        <DialogHeader>
          <DialogTitle>Available Apps</DialogTitle>
          <DialogDescription>Find the app to integrate here</DialogDescription>
        </DialogHeader>
        <Table>
          <TableBody className="border border-gray-300">
            {providers?.map((provider) => (
              <TableRow key={provider.account_id}>
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
