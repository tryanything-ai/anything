"use client";

import { useState, useEffect } from "react";
import { Button } from "@repo/ui/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@repo/ui/components/ui/dropdown-menu";
import Link from "next/link";
import { UserIcon } from "lucide-react";
import { createClient } from "@/lib/supabase/client";
import { useRouter } from "next/navigation";

export default function UserAccountButton(): JSX.Element {
  const [personalAccount, setPersonalAccount] = useState<any>(null);
  const router = useRouter();
  const supabaseClient = createClient();

  useEffect(() => {
    const fetchPersonalAccount = async () => {
      const { data } = await supabaseClient.rpc("get_personal_account");
      setPersonalAccount(data);
    };

    fetchPersonalAccount();
  }, []);

  const signOut = async () => {
    await supabaseClient.auth.signOut();
    router.push("/login");
  };

  if (!personalAccount) {
    return null; // Or a loading spinner
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost">
          <UserIcon />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-56" align="end" forceMount>
        <DropdownMenuLabel className="font-normal">
          <div className="flex flex-col space-y-1">
            <p className="text-sm font-medium leading-none">
              {personalAccount.name}
            </p>
            <p className="text-xs leading-none text-muted-foreground">
              {personalAccount.email}
            </p>
          </div>
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuItem asChild>
            <Link href="/settings">My Account</Link>
          </DropdownMenuItem>
          <DropdownMenuItem asChild>
            <Link href="/settings">Settings</Link>
          </DropdownMenuItem>
          {/* <DropdownMenuItem asChild>
            <Link href="/settings/teams">Teams</Link>
          </DropdownMenuItem> */}
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuItem>
          <button onClick={signOut}>Log out</button>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
