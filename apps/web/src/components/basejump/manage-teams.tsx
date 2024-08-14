import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import { createClient } from "@/lib/supabase/server";
import {
  Table,
  TableRow,
  TableBody,
  TableCell,
} from "@repo/ui/components/ui/table";
import { Button } from "@repo/ui/components/ui/button";
import Link from "next/link";
import { Badge } from "@repo/ui/components/ui/badge";

export default async function ManageTeams() {
  const supabaseClient = createClient();

  const { data }: any = await supabaseClient.rpc("get_accounts");

  const teams: any[] = data?.filter(
    (team: any) => team.personal_account === false,
  );

  return (
    <Card>
      <CardHeader>
        <CardTitle>Teams</CardTitle>
        <CardDescription>These are the teams you belong to</CardDescription>
      </CardHeader>
      <CardContent>
        <Table>
          <TableBody>
            {teams.map((team) => (
              <TableRow key={team.account_id}>
                <TableCell>
                  <div className="flex gap-x-2">
                    {team.name}
                    <Badge
                      variant={
                        team.account_role === "owner" ? "default" : "outline"
                      }
                    >
                      {team.is_primary_owner
                        ? "Primary Owner"
                        : team.account_role}
                    </Badge>
                  </div>
                </TableCell>
                <TableCell className="text-right">
                  <Button variant="outline" asChild>
                    <Link href={`/dashboard/${team.slug}`}>View</Link>
                  </Button>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}
