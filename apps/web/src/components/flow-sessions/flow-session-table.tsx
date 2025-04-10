import { useState } from "react";
import { useParams } from "next/navigation"; // Assuming Next.js for params
import useSWR from "swr";
import { format } from "date-fns";
import { createClient } from "@/lib/supabase/client"; // Import Supabase client hook
import api from "@repo/anything-api";
import { useAnything } from "@/context/AnythingContext";

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@repo/ui/components/ui/table";
import { Badge } from "@repo/ui/components/ui/badge";
import { Button } from "@repo/ui/components/ui/button";
import {
  Alert,
  AlertDescription,
  AlertTitle,
} from "@repo/ui/components/ui/alert";
import { Loader2, ChevronDown, ChevronRight, AlertCircle } from "lucide-react";
import { ResultComponent } from "@/components/studio/forms/testing/task-card"; // Reuse existing component if suitable

// Helper to format duration
const formatDuration = (start: string | null, end: string | null): string => {
  if (!start || !end) return "N/A";
  const durationMs = new Date(end).getTime() - new Date(start).getTime();
  if (durationMs < 0) return "N/A";
  if (durationMs < 1000) return `${durationMs} ms`;
  return `${(durationMs / 1000).toFixed(2)} s`;
};

// Helper for status badge styling
const getStatusVariant = (
  status: any["status"] | string,
): "default" | "secondary" | "destructive" | "outline" => {
  switch (status) {
    case "completed":
      return "default"; // Green in default theme
    case "running":
      return "secondary"; // Blue-ish
    case "failed":
      return "destructive"; // Red
    case "stopped":
      return "outline"; // Gray
    case "pending":
      return "outline";
    default:
      return "outline";
  }
};

interface FlowSessionTableProps {
  workflowId?: string; // Optional filter
  itemsPerPage?: number;
  accountId: string;
}

export function FlowSessionTable({
  accountId,
  workflowId,
  itemsPerPage = 20,
}: FlowSessionTableProps): JSX.Element {
  const {
    accounts: { selectedAccount },
  } = useAnything();

  // const params = useParams();
  // const accountId = params.accountId as string; // Assuming accountId is in route params
  // const supabase = await createClient();

  const [pageIndex, setPageIndex] = useState(0);
  const [expandedSessionIds, setExpandedSessionIds] = useState<Set<string>>(
    new Set(),
  );
  const [sessions, setSessions] = useState<any[] | undefined>(undefined);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const offset = pageIndex * itemsPerPage;

  // Use an array key for SWR, including all dependencies for the fetcher
  // const swrKey =
  //   accountId && supabase
  //     ? ["flowSessions", accountId, { workflowId, limit: itemsPerPage, offset }]
  //     : null;

  // const {
  //   data: sessions,
  //   error,
  //   isLoading,
  // } = useSWR<any[] | undefined>(
  //   swrKey,
  //   async ([, accId, options]) => {
  //     // Destructure the key array
  //     if (!supabase) throw new Error("Supabase client not available");
  //     return getFlowSessions(supabase, accId, options);
  //   },
  //   {
  //     keepPreviousData: true, // Keep showing old data while loading new page
  //   },
  // );

  const flowSessionsFetcher = async () => {
    if (!selectedAccount) return [];
    const supabase = await createClient();
    return api.flow_sessions.getFlowSessions(supabase, selectedAccount.account_id);
  };

  const toggleExpand = (sessionId: string) => {
    setExpandedSessionIds((prev) => {
      const newSet = new Set(prev);
      if (newSet.has(sessionId)) {
        newSet.delete(sessionId);
      } else {
        newSet.add(sessionId);
      }
      return newSet;
    });
  };


  if (isLoading && !sessions) {
    // Show loader only on initial load
    return (
      <div className="flex justify-center items-center h-40">
        <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    );
  }

  if (error) {
    return (
      <Alert variant="destructive">
        <AlertCircle className="h-4 w-4" />
        <AlertTitle>Error Fetching Flow Sessions</AlertTitle>
        <AlertDescription>
          {error.message || "An unknown error occurred."}
        </AlertDescription>
      </Alert>
    );
  }

  const hasMore = sessions && sessions.length === itemsPerPage;
  const hasPrevious = pageIndex > 0;

  return (
    <div className="space-y-4">
      {isLoading && (
        <div className="absolute top-2 right-2">
          <Loader2 className="h-4 w-4 animate-spin text-muted-foreground" />
        </div>
      )}{" "}
      {/* Loading indicator for page changes */}
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-4"></TableHead> {/* Expander */}
            <TableHead>Status</TableHead>
            <TableHead>Session ID</TableHead>
            {/* <TableHead>Workflow ID</TableHead> */}
            {/* <TableHead>Version ID</TableHead> */}
            <TableHead>Started</TableHead>
            <TableHead>Duration</TableHead>
            <TableHead className="text-right">Ended</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {!sessions || sessions.length === 0 ? (
            <TableRow>
              <TableCell colSpan={6} className="text-center h-24">
                No flow sessions found.
              </TableCell>
            </TableRow>
          ) : (
            sessions.map((session) => (
              <>
                <TableRow
                  key={session.flow_session_id}
                  className="cursor-pointer hover:bg-muted/50"
                  onClick={() => toggleExpand(session.flow_session_id)}
                >
                  <TableCell className="w-4 px-2">
                    {(session.trigger_event ||
                      session.final_output ||
                      session.error_message) &&
                      (expandedSessionIds.has(session.flow_session_id) ? (
                        <ChevronDown className="h-4 w-4 text-muted-foreground" />
                      ) : (
                        <ChevronRight className="h-4 w-4 text-muted-foreground" />
                      ))}
                  </TableCell>
                  <TableCell>
                    <Badge
                      variant={getStatusVariant(session.status)}
                      className="capitalize"
                    >
                      {session.status}
                    </Badge>
                  </TableCell>
                  <TableCell className="font-mono text-xs">
                    {session.flow_session_id}
                  </TableCell>
                  {/* <TableCell className="font-mono text-xs">{session.workflow_id}</TableCell> */}
                  {/* <TableCell className="font-mono text-xs">{session.workflow_version_id}</TableCell> */}
                  <TableCell>
                    {session.started_at
                      ? format(new Date(session.started_at), "Pp")
                      : "N/A"}
                  </TableCell>
                  <TableCell>
                    {formatDuration(session.started_at, session.ended_at)}
                  </TableCell>
                  <TableCell className="text-right">
                    {session.ended_at
                      ? format(new Date(session.ended_at), "Pp")
                      : session.status === "running"
                        ? "Running..."
                        : "N/A"}
                  </TableCell>
                </TableRow>
                {expandedSessionIds.has(session.flow_session_id) && (
                  <TableRow className="bg-muted/20 hover:bg-muted/30">
                    <TableCell colSpan={6}>
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 p-4">
                        {session.trigger_event && (
                          <div>
                            <h4 className="font-semibold mb-2 text-sm">
                              Trigger Event
                            </h4>
                            <div
                              suppressHydrationWarning
                              className="h-full max-h-96 overflow-auto rounded border bg-background p-2"
                            >
                              <ResultComponent
                                result={session.trigger_event}
                                collapseStringsAfterLength={500}
                                collapsed={false} // Start expanded
                              />
                            </div>
                          </div>
                        )}
                        {session.final_output && (
                          <div>
                            <h4 className="font-semibold mb-2 text-sm">
                              Final Output
                            </h4>
                            <div
                              suppressHydrationWarning
                              className="h-full max-h-96 overflow-auto rounded border bg-background p-2"
                            >
                              <ResultComponent
                                result={session.final_output}
                                collapseStringsAfterLength={500}
                                collapsed={false} // Start expanded
                              />
                            </div>
                          </div>
                        )}
                        {session.error_message && (
                          <div className="md:col-span-2">
                            <h4 className="font-semibold mb-2 text-sm text-destructive">
                              Error
                            </h4>
                            <pre className="text-xs p-2 border border-destructive/50 bg-destructive/10 rounded max-h-60 overflow-auto">
                              {session.error_message}
                            </pre>
                          </div>
                        )}
                        {!session.trigger_event &&
                          !session.final_output &&
                          !session.error_message && (
                            <p className="text-muted-foreground text-sm md:col-span-2">
                              No detailed data available for this session.
                            </p>
                          )}
                      </div>
                    </TableCell>
                  </TableRow>
                )}
              </>
            ))
          )}
        </TableBody>
      </Table>
      <div className="flex items-center justify-end space-x-2 py-4">
        <Button
          variant="outline"
          size="sm"
          onClick={() => setPageIndex((old) => Math.max(0, old - 1))}
          disabled={!hasPrevious || isLoading}
        >
          Previous
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={() => setPageIndex((old) => old + 1)}
          disabled={!hasMore || isLoading}
        >
          Next
        </Button>
      </div>
    </div>
  );
}
