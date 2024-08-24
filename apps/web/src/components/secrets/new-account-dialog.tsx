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

const NewAccountDialog = (): JSX.Element => {
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
  // Helper function to generate a random string
  const generateRandomString = (length: any) => {
    const charset =
      "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
    let result = "";
    for (let i = 0; i < length; i++) {
      result += charset.charAt(Math.floor(Math.random() * charset.length));
    }
    return result;
  };

  // Helper function to generate a secure state (you should use a more secure implementation for production)
  const generateState = () => {
    return btoa(generateRandomString(16));
  };

  // Helper function to generate a secure code challenge (requires a real sha256 implementation)
  const generateCodeChallenge = async (code_verifier: string) => {
    const encoder = new TextEncoder();
    const data = encoder.encode(code_verifier);
    const hash = await crypto.subtle.digest("SHA-256", data);
    return btoa(String.fromCharCode(...new Uint8Array(hash)))
      .replace(/\+/g, "-")
      .replace(/\//g, "_")
      .replace(/=+$/, "");
  };

  const addConnection = async (provider: any) => {
    try {
      console.log("Adding connection for provider:", provider);
      //Create Link
      //Create CSFR State
      //Open new window to service
      // window.open(provider.auth_url, "_blank", "noopener,noreferrer");
      const client_id = provider.client_id; // Replace with your actual client_id
      const redirect_uri =
        // "https://workflow-engine-axum-dev.up.railway.app/auth/airtable/callback";
      "https://anythingapp-git-dev-tryanything.vercel.app/auth/airtable/callback"; // Replace with your actual redirect_uri
      const scope = "data.records:read data.records:write"; // Replace with your actual scopes
      const state = generateState();
      const code_challenge_method = "S256";

      const authUrl = `${provider.auth_url}?client_id=${client_id}&redirect_uri=${encodeURIComponent(redirect_uri)}&response_type=code&scope=${encodeURIComponent(scope)}&state=${state}&code_challenge=${code_challenge}&code_challenge_method=${code_challenge_method}`;
      window.open(authUrl, "_blank");
      //Redirect back to ?
    } catch (error) {
      console.error("Error adding connection:", error);
    }
  };

  useEffect(() => {
    // Fetch accounts
    fetchAccounts();
  }, []);

  const [code_verifier, setCodeVerifier] = useState(generateRandomString(43));
  const [code_challenge, setCodeChallenge] = useState("");

  useEffect(() => {
    // Generate code challenge when component mounts
    (async () => {
      const challenge = await generateCodeChallenge(code_verifier);
      setCodeChallenge(challenge);
    })();
  }, [code_verifier]);

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button>Add New Account</Button>
      </DialogTrigger>
      {/* <DialogContent className="sm:max-w-[425px]"> */}
      <DialogContent className="max-w-3xl h-2/3">
        <DialogHeader>
          <DialogTitle>Available Apps</DialogTitle>
          <DialogDescription>Find the app to integrate here</DialogDescription>
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
