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
import api from "@/lib/anything-api";
import {
  Table,
  TableBody,
  TableCell,
  TableRow,
} from "@repo/ui/components/ui/table";
import { BaseNodeIcon } from "../studio/nodes/node-icon";
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
      const res = await api.auth.getProviders();
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
      
      let { url } = await api.auth.initiateProviderAuth(provider.provider_name);

      console.log("url", url);
      //TODO: Add api route to "auth/airtable/initiate" to get the url with all the state in it
      // const clientId = provider.client_id; // Replace with your actual client_id
      // const redirectUri =
      //   "https://workflow-engine-axum-dev.up.railway.app/auth/airtable/callback";
      // // "https://anythingapp-git-dev-tryanything.vercel.app/auth/airtable/callback"; // Replace with your actual redirect_uri
      // const scope = "data.records:read data.records:write"; // Replace with your actual scopes
      // const state = generateState(); // Generate state for CSRF protection

      // const authUrl = `${provider.auth_url}?client_id=${clientId}&redirect_uri=${encodeURIComponent(redirectUri)}&response_type=code&scope=${encodeURIComponent(scope)}&state=${state}&code_challenge=${codeChallenge}&code_challenge_method=S256`;

      // Open the auth URL in a popup window
      window.open(url, "_blank", "noopener,noreferrer,width=600,height=600");
    } catch (error) {
      console.error("Error adding connection:", error);
    }
  };

  // Helper function to generate a secure state (use a more secure implementation for production)
  // const generateState = () => {
  //   return btoa(generateRandomString(16));
  // };

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
            {providers.map((provider) => (
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
