import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Table, TableRow, TableBody, TableCell } from "@/components/ui/table";

export default async function AccountsPage() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Accounts</CardTitle>
        <CardDescription>Connect Accounts</CardDescription>
      </CardHeader>
      <CardContent>
        <Table>
          <TableBody>
            {/* {teams.map((team) => (
                            <TableRow key={team.account_id}>
                                <TableCell>
                                    <div className="flex gap-x-2">
                                    {team.name}
                                    <Badge variant={team.account_role === 'owner' ? 'default' : 'outline'}>{team.is_primary_owner ? 'Primary Owner' : team.account_role}</Badge></div>
                                </TableCell>
                                <TableCell className="text-right"><Button variant="outline" asChild><Link href={`/dashboard/${team.slug}`}>View</Link></Button></TableCell>
                            </TableRow>
                        ))} */}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}
