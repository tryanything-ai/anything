"use client";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";

export default function ToolsTab() {
  return (
    <ScrollArea>
      <Card>
        <CardHeader>
          <CardTitle>Agent Tools</CardTitle>
          <CardDescription>
            Configure the tools available to your agent
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="text-muted-foreground">
            Tools configuration coming soon
          </div>
        </CardContent>
      </Card>
    </ScrollArea>
  );
}
